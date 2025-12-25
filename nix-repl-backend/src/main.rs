use axum::{
    Json, Router,
    extract::{Request, State},
    http::{HeaderMap, HeaderValue, StatusCode, header},
    middleware::{self, Next},
    response::Response,
};
use serde::{Deserialize, Serialize};
use std::{env, net::SocketAddr, process::Stdio, sync::Arc, time::Duration};
use tokio::{process::Command, time::timeout};
use tower_http::cors::CorsLayer;

const MAX_BODY_SIZE: usize = 1024 * 1024; // 1MB
const EVAL_TIMEOUT: Duration = Duration::from_secs(5);
const MAX_OUTPUT: usize = 10_000;

#[derive(Clone)]
struct AppState {
    token: Option<String>,
}

#[derive(Deserialize)]
struct EvalRequest {
    code: String,
}

#[derive(Serialize)]
struct EvalResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    stdout: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Fail closed: require token
    let token = match env::var("NIX_REPL_TOKEN") {
        Ok(t) if !t.trim().is_empty() => Some(t),
        _ => {
            eprintln!("ERROR: NIX_REPL_TOKEN must be set (refusing to start).");
            std::process::exit(2);
        }
    };

    let state = Arc::new(AppState { token });

    // Strict CORS allowlist (mdbook serve defaults to localhost:3000)
    let allowed_origins = [
        HeaderValue::from_static("http://localhost:3000"),
        HeaderValue::from_static("http://127.0.0.1:3000"),
    ];

    let cors = CorsLayer::new()
        .allow_origin(allowed_origins)
        .allow_methods([axum::http::Method::POST, axum::http::Method::OPTIONS])
        .allow_headers([
            header::CONTENT_TYPE,
            header::HeaderName::from_static("x-nix-repl-token"),
        ]);

    let app = Router::new()
        .route("/", axum::routing::post(handle_eval))
        .route("/", axum::routing::options(handle_preflight))
        .layer(cors)
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .layer(tower_http::limit::RequestBodyLimitLayer::new(MAX_BODY_SIZE))
        .with_state(state);

    // Default to localhost for safety (native runs).
    // For container runs, override with: -e NIX_REPL_BIND=0.0.0.0
    let bind_addr = env::var("NIX_REPL_BIND").unwrap_or_else(|_| "127.0.0.1".to_string());
    let addr: SocketAddr = format!("{}:8080", bind_addr).parse()?;

    println!("🔒 nix-repl secure server (Rust) listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
async fn handle_preflight() -> StatusCode {
    StatusCode::NO_CONTENT
}

async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, (StatusCode, Json<ErrorResponse>)> {
    // Skip auth check for OPTIONS
    if request.method() == axum::http::Method::OPTIONS {
        return Ok(next.run(request).await);
    }

    if let Some(expected) = &state.token {
        let auth_header = headers
            .get("x-nix-repl-token")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");

        if auth_header != expected.as_str() {
            eprintln!("DEBUG: Expected='{}' Received='{}'", expected, auth_header);
            return Err((
                StatusCode::FORBIDDEN,
                Json(ErrorResponse {
                    error: "Forbidden: Invalid Token".to_string(),
                }),
            ));
        }
    }

    Ok(next.run(request).await)
}

async fn handle_eval(
    Json(req): Json<EvalRequest>,
) -> Result<Json<EvalResponse>, (StatusCode, Json<ErrorResponse>)> {
    if req.code.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "No code provided".to_string(),
            }),
        ));
    }

    // Run nix eval with timeout
    let result = timeout(EVAL_TIMEOUT, run_nix_eval(&req.code)).await;

    match result {
        Ok(Ok((stdout, stderr, exit_code))) => {
            if exit_code == 0 {
                Ok(Json(EvalResponse {
                    stdout: Some(truncate(stdout, MAX_OUTPUT)),
                    error: None,
                }))
            } else {
                Ok(Json(EvalResponse {
                    stdout: None,
                    error: Some(truncate(stderr, MAX_OUTPUT)),
                }))
            }
        }
        Ok(Err(e)) => Ok(Json(EvalResponse {
            stdout: None,
            error: Some(format!("Server error: {}", e)),
        })),
        Err(_) => Ok(Json(EvalResponse {
            stdout: None,
            error: Some("Execution timed out (5s limit)".to_string()),
        })),
    }
}

async fn run_nix_eval(code: &str) -> anyhow::Result<(String, String, i32)> {
    let child = Command::new("nix")
        .args(["eval", "--json", "--expr", code])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let output = child.wait_with_output().await?;

    Ok((
        String::from_utf8_lossy(&output.stdout).to_string(),
        String::from_utf8_lossy(&output.stderr).to_string(),
        output.status.code().unwrap_or(-1),
    ))
}

fn truncate(s: String, max_len: usize) -> String {
    if s.len() <= max_len {
        s
    } else {
        s.chars().take(max_len).collect()
    }
}
