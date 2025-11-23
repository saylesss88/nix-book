use mdbook_rss_feed::build_feed;
use serde_json::Value;
use std::fs;
use std::io::{self, Read, Write}; // Added for flush and Write trait
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.get(1).map(|s| s.as_str()) == Some("supports") {
        println!("true");
        return;
    }

    let mut input = String::new();
    match std::io::stdin().read_to_string(&mut input) {
        Ok(0) => {
            eprintln!("ERROR: Read 0 bytes from stdin – empty input?");
            eprintln!(
                "Current dir: {}",
                std::env::current_dir().unwrap_or_default().display()
            );
            eprintln!("Args: {:?}", std::env::args().collect::<Vec<_>>());
            std::process::exit(1);
        }
        Ok(len) => eprintln!("Read {} bytes from stdin", len),
        Err(e) => {
            eprintln!("ERROR: Failed to read stdin: {}", e);
            std::process::exit(1);
        }
    }

    if input.trim().is_empty() {
        eprintln!("ERROR: Stdin empty after read");
        std::process::exit(1);
    }

    eprintln!(
        "Stdin preview (first 100 chars): {}",
        &input[..input.len().min(100)]
    );

    let input_array: Vec<Value> = serde_json::from_str(&input).expect("Invalid JSON from mdBook");

    if input_array.len() < 2 {
        eprintln!("ERROR: mdBook sent less than 2 JSON objects");
        std::process::exit(1);
    }

    let context = &input_array[0];
    let book = &input_array[1]; // THIS IS THE REAL BOOK!

    let root = context
        .pointer("/root")
        .and_then(|v| v.as_str())
        .unwrap_or(".");

    let src_dir = PathBuf::from(root).join("src");

    // --- Robust Site URL Extraction ---
    let site_url = context
        .pointer("/config/output/html/site-url")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        // Fallback to a valid URL if config is missing or empty
        .unwrap_or("https://example.com/")
        .trim_end_matches('/') // Ensure no trailing slash
        .to_string();
    // ----------------------------------

    let feed_title = context
        .pointer("/config/book/title")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or("My mdBook");

    let feed_description = context
        .pointer("/config/book/description")
        .and_then(|v| v.as_str())
        .unwrap_or("An mdBook-generated site");

    eprintln!("Root: {root}");
    eprintln!("Site URL: {site_url}");
    eprintln!("Title: {feed_title}");

    let channel = build_feed(&src_dir, feed_title, &site_url, feed_description)
        .expect("Failed to generate RSS feed");

    // Write to src/rss.xml (auto-copied by renderer to book/rss.xml)
    let rss_path = src_dir.join("rss.xml");

    // Debug: Generate content and check dir
    let rss_content = channel.to_string();
    let rss_bytes = rss_content.as_bytes();
    eprintln!("RSS content size before write: {} bytes", rss_bytes.len());
    eprintln!("RSS path: {}", rss_path.display());
    eprintln!("Src dir exists: {}", src_dir.exists());
    eprintln!(
        "Src dir writable: {}",
        src_dir
            .metadata()
            .map(|m| m.permissions().readonly() == false)
            .unwrap_or(false)
    );

    // Attempt write with error handling
    match fs::write(&rss_path, rss_bytes) {
        Ok(_) => {
            let written_metadata = rss_path.metadata();
            match written_metadata {
                Ok(m) => {
                    eprintln!("Write succeeded! Written file size: {} bytes", m.len());
                    if m.len() == 0 {
                        eprintln!("ERROR: Wrote 0 bytes—possible I/O truncation");
                    }
                }
                Err(e) => eprintln!("ERROR: Failed to get metadata after write: {}", e),
            }
        }
        Err(e) => {
            eprintln!("ERROR: fs::write failed: {}", e);
            std::process::exit(1);
        }
    }

    // Read back immediately for verification
    match fs::read_to_string(&rss_path) {
        Ok(read_back) => {
            eprintln!("Read-back success! Size: {} bytes", read_back.len());
            eprintln!(
                "Read-back preview (first 400 chars): {}",
                &read_back[..read_back.len().min(400)]
            );
            if read_back.len() != rss_bytes.len() {
                eprintln!(
                    "WARNING: Truncation! Expected {}, read {}",
                    rss_bytes.len(),
                    read_back.len()
                );
            }
            // Flush logs to ensure they appear before stdout
            io::stderr().flush().unwrap();
        }
        Err(e) => eprintln!("ERROR: Failed to read back RSS after write: {}", e),
    }

    eprintln!("RSS feed written to {}", rss_path.display());

    // ECHO BACK THE SECOND ELEMENT — THE ACTUAL BOOK
    println!("{}", serde_json::to_string(book).unwrap());
}
