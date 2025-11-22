// src/bin/mdbook-rss.rs
use mdbook_rss::{build_feed, collect_articles};
use serde_json::Value;
use std::io::{self, Read, Write};
use std::path::PathBuf;

fn main() {
    // ← NO eprintln! here — that was the compile error
    let args: Vec<String> = std::env::args().collect();
    if args.get(1).map(|s| s.as_str()) == Some("supports") {
        println!("true");
        return;
    }

    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read stdin");

    let mut input_json: Vec<Value> = serde_json::from_str(&input).expect("Invalid JSON");
    if input_json.len() != 2 {
        eprintln!("ERROR: expected [context, book]");
        std::process::exit(1);
    }

    let context = input_json.remove(0);
    let book = input_json.remove(0);

    let root = context
        .pointer("/root")
        .and_then(|v| v.as_str())
        .unwrap_or(".");
    let src_dir = PathBuf::from(root).join("src");

    let site_url = context
        .pointer("/config/output/html/site-url")
        .and_then(|v| v.as_str())
        .unwrap_or("https://example.com/");

    let feed_title = context
        .pointer("/config/book/title")
        .and_then(|v| v.as_str())
        .unwrap_or("My mdBook");

    let feed_description = context
        .pointer("/config/book/description")
        .and_then(|v| v.as_str())
        .unwrap_or("An mdBook-generated site");

    eprintln!("Using site-url = {}", site_url);
    eprintln!("Using title = {}", feed_title);

    let channel = build_feed(&src_dir, feed_title, site_url, feed_description)
        .expect("Failed to build RSS feed");

    // Write the RSS file to the book directory
    let book_dir = PathBuf::from(root).join("book");
    std::fs::create_dir_all(&book_dir).unwrap();
    let rss_path = book_dir.join("rss.xml");
    std::fs::write(&rss_path, channel.to_string()).unwrap();
    eprintln!("RSS feed written to {}", rss_path.display());

    // Still output the (unchanged) book JSON to stdout so mdbook continues
    println!("{}", serde_json::to_string(&book).unwrap());
    io::stdout().flush().unwrap();
}
