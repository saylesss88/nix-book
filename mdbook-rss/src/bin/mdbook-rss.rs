use serde_json::Value;
use std::env;
use std::io::{self, Read, Write};
use std::path::PathBuf;

fn main() {
    eprintln!(
        "mdbook-rss: STARTED, args = {:?}",
        std::env::args().collect::<Vec<_>>()
    );

    let args: Vec<String> = env::args().collect();

    // Handle "supports" command early
    if args.get(1).map(|s| s.as_str()) == Some("supports") {
        println!("true");
        return;
    }

    // Read full JSON book from stdin
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read stdin");

    // Parse input JSON as an array of two elements [Context, Book]
    let mut input_json: Vec<Value> =
        serde_json::from_str(&input).expect("Input JSON should be an array");

    if input_json.len() != 2 {
        eprintln!("mdbook-rss: error - input JSON array length is not 2");
        std::process::exit(1);
    }
    eprintln!(
        "mdbook-rss: raw stdin = {:?}",
        &input[..input.len().min(200)]
    );

    // Extract context and book JSON objects
    let context = input_json.remove(0);
    let book = input_json.remove(0);

    // Extract root path from context for debugging
    let root = context
        .pointer("/root")
        .and_then(|v| v.as_str())
        .unwrap_or(".");
    eprintln!("mdbook-rss: root directory from context = {}", root);

    let src_dir = PathBuf::from(root).join("src");
    eprintln!("mdbook-rss: src directory = {}", src_dir.display());

    // Here you would call your build_feed & write RSS logic
    // For example:
    match mdbook_rss::build_feed(
        &src_dir,
        "nix-book",
        "https://saylesss88.github.io/nix-book",
        "A book about Nix",
    ) {
        Ok(channel) => {
            eprintln!(
                "mdbook-rss: feed built successfully with {} items",
                channel.items().len()
            );
            let book_dir = PathBuf::from(root).join("book"); // not book/html
            std::fs::create_dir_all(&book_dir).expect("Failed to create book directory");
            std::fs::write(book_dir.join("rss.xml"), channel.to_string())
                .expect("Failed to write rss.xml");

            eprintln!(
                "mdbook-rss: RSS feed written to {}",
                book_dir.join("rss.xml").display()
            );
        }
        Err(e) => {
            eprintln!("mdbook-rss: error building feed: {}", e);
            std::process::exit(1);
        }
    }

    // Output the processed book JSON only (as required by mdbook)
    println!("{}", serde_json::to_string(&book).unwrap());
    std::io::stdout().flush().unwrap();
}
