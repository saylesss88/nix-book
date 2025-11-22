use mdbook_rss::build_feed;
use serde_json::Value;
use std::io::{self, Read, Write};
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // Handle the "supports" command
    if args.get(1).map(|s| s.as_str()) == Some("supports") {
        println!("true");
        return;
    }

    // Read the JSON book from stdin
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read stdin");

    let mut input_json: Vec<Value> =
        serde_json::from_str(&input).expect("Input JSON should be an array [context, book]");

    if input_json.len() != 2 {
        eprintln!("mdbook-rss: expected JSON array of length 2 [context, book]");
        std::process::exit(1);
    }

    // Extract context and book JSON
    let context = input_json.remove(0);
    let book = input_json.remove(0);

    // Extract root directory
    let root = context
        .pointer("/root")
        .and_then(|v| v.as_str())
        .unwrap_or(".");
    let src_dir = PathBuf::from(root).join("src");
    eprintln!("mdbook-rss: src directory = {}", src_dir.display());

    // Site URL
    let site_url = context
        .pointer("/config/output/html/site-url")
        .and_then(|v| v.as_str())
        .unwrap_or("https://example.com/");

    // Book title and description
    let feed_title = context
        .pointer("/config/book/title")
        .and_then(|v| v.as_str())
        .unwrap_or("My mdBook");
    let feed_description = context
        .pointer("/config/book/description")
        .and_then(|v| v.as_str())
        .unwrap_or("An mdBook-generated site");

    // Build RSS feed
    match build_feed(&src_dir, feed_title, site_url, feed_description) {
        Ok(channel) => {
            let book_dir = PathBuf::from(root).join("book");
            std::fs::create_dir_all(&book_dir).expect("Failed to create book directory");
            let rss_path = book_dir.join("rss.xml");
            std::fs::write(&rss_path, channel.to_string()).expect("Failed to write rss.xml");
            eprintln!("mdbook-rss: RSS feed written to {}", rss_path.display());
        }
        Err(e) => {
            eprintln!("mdbook-rss: Failed to build RSS feed: {}", e);
        }
    }

    // Only print the book JSON back to stdout (required by mdBook)
    println!("{}", serde_json::to_string(&book).unwrap());
    io::stdout().flush().unwrap();
}
