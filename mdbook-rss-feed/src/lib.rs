use anyhow::Result;
use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use pulldown_cmark::{html, Options, Parser};
use rss::{Channel, ChannelBuilder, Guid, Item, ItemBuilder};
use serde::{Deserialize, Deserializer};
use std::{fs, path::Path, time::SystemTime};
use walkdir::WalkDir;

// Helper: convert file modification time → UTC DateTime
fn systemtime_to_utc(st: SystemTime) -> DateTime<Utc> {
    DateTime::<Utc>::from(st)
}

fn deserialize_date<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;

    if let Some(date_str) = s {
        // Try RFC3339 first (e.g. 2025-11-20T10:00:00Z)
        if let Ok(dt) = DateTime::parse_from_rfc3339(&date_str) {
            return Ok(Some(dt.with_timezone(&Utc)));
        }

        // Then try YYYY-MM-DD
        if let Ok(nd) = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d") {
            return Ok(Some(
                Utc.from_utc_datetime(&nd.and_hms_opt(0, 0, 0).unwrap()),
            ));
        }
    }
    Ok(None)
}

#[derive(Debug, Deserialize, Clone)]
pub struct FrontMatter {
    pub title: String,

    #[serde(deserialize_with = "deserialize_date")]
    pub date: Option<DateTime<Utc>>,

    pub author: Option<String>,

    pub description: Option<String>,
}

#[derive(Debug)]
pub struct Article {
    pub fm: FrontMatter,
    pub content: String,
    pub path: String, // relative path from src/
}

pub fn parse_markdown_file(root: &Path, path: &Path) -> Result<Article> {
    let text = fs::read_to_string(path)?;

    let mut lines = text.lines();
    let mut yaml = String::new();
    let mut in_yaml = false;

    // Extract front-matter
    for line in lines.by_ref() {
        let trimmed = line.trim();
        if trimmed == "---" {
            if !in_yaml {
                in_yaml = true;
                continue;
            } else {
                break;
            }
        }
        if in_yaml {
            yaml.push_str(line);
            yaml.push('\n');
        }
    }

    // Remaining markdown content
    let content = lines.collect::<Vec<_>>().join("\n") + "\n";

    // Fallback date from file modification time
    let fallback_date = path
        .metadata()
        .ok()
        .and_then(|m| m.modified().ok())
        .map(systemtime_to_utc);

    // Parse front-matter or use defaults
    let fm = if !yaml.trim().is_empty() {
        serde_yaml::from_str(&yaml).unwrap_or_else(|_| FrontMatter {
            title: path
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned(),
            date: fallback_date,
            author: None,
            description: Some(content.clone()),
        })
    } else {
        FrontMatter {
            title: path
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned(),
            date: fallback_date,
            author: None,
            description: Some(content.clone()),
        }
    };

    let rel_path = path.strip_prefix(root).unwrap_or(path);

    Ok(Article {
        fm,
        content,
        path: rel_path.to_string_lossy().into_owned(),
    })
}

pub fn collect_articles(src_dir: &Path) -> Result<Vec<Article>> {
    let mut articles = Vec::new();

    for entry in WalkDir::new(src_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|s| s.to_ascii_lowercase());

        if !matches!(ext.as_deref(), Some("md" | "markdown")) {
            continue;
        }

        if path
            .file_name()
            .and_then(|n| n.to_str())
            .map(|n| n.eq_ignore_ascii_case("SUMMARY.md"))
            .unwrap_or(false)
        {
            continue;
        }

        match parse_markdown_file(src_dir, path) {
            Ok(article) => {
                if article.fm.title.trim().is_empty() && article.fm.description.is_none() {
                    eprintln!("Skipping empty article: {}", path.display());
                } else {
                    eprintln!("Parsed article: {} ({})", article.fm.title, article.path);
                    articles.push(article);
                }
            }
            Err(e) => eprintln!("Failed to parse {}: {}", path.display(), e),
        }
    }

    // Sort newest first (fallback date works automatically)
    articles.sort_by_key(|a| a.fm.date);
    articles.reverse();

    Ok(articles)
}

fn markdown_to_html(md: &str) -> String {
    let mut html = String::new();
    let parser = Parser::new_ext(md, Options::all());
    html::push_html(&mut html, parser);
    html
}

pub fn build_feed(
    src_dir: &Path,
    title: &str,
    site_url: &str,
    description: &str,
) -> Result<Channel> {
    eprintln!("build_feed called with site_url = '{site_url}'");

    let articles = collect_articles(src_dir)?;

    // IMPORTANT: channel.link MUST be non-empty or the rss crate silently drops all items!
    let base_url = site_url.trim_end_matches('/');

    eprintln!("Using base_url = '{base_url}'");

    let items: Vec<Item> = articles
        .into_iter()
        .map(|article| {
            eprintln!("Generating RSS item for: {}", article.path);

            let html_path = article
                .path
                .replace('\\', "/")
                .replace(".md", ".html")
                .replace("/README.html", "/index.html");

            let link = format!("{base_url}/{html_path}");

            let raw_html = markdown_to_html(
                article
                    .fm
                    .description
                    .as_deref()
                    .unwrap_or(&article.content),
            );

            // Proper XML escaping
            let safe_description = raw_html
                .replace('&', "&amp;")
                .replace('<', "&lt;")
                .replace('>', "&gt;")
                .replace('"', "&quot;")
                .replace('\'', "&apos;");

            let mut item = ItemBuilder::default();

            item.title(Some(article.fm.title.clone()));
            item.link(Some(link.clone()));
            item.description(Some(safe_description));
            item.guid(Some(Guid {
                value: link.clone(),
                permalink: true,
            }));

            if let Some(date) = article.fm.date {
                item.pub_date(Some(date.to_rfc2822()));
            }
            if let Some(author) = article.fm.author {
                item.author(Some(author));
            }

            item.build()
        })
        .collect();

    eprintln!("Total RSS items generated: {}", items.len());

    let mut channel_builder = ChannelBuilder::default();
    channel_builder.title(title.to_string());
    channel_builder.link(base_url.to_string());

    let safe_desc = description
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;");

    channel_builder.description(&safe_desc);
    channel_builder.items(items);
    channel_builder.generator(Some("mdbook-rss-feed 0.1.0".to_string()));

    let mut channel = channel_builder.build();

    // Explicitly set to ensure builder applies them (overrides any issues)
    channel.set_title(title.to_string());
    let full_link = format!("{}/", base_url); // Trailing / for site root
    channel.set_link(full_link);
    channel.set_description(safe_desc);
    channel.set_generator(Some("mdbook-rss-feed 0.1.0".to_string()));

    eprintln!("Final channel item count: {}", channel.items().len());
    eprintln!("Channel title: {}", channel.title());
    eprintln!("Channel link: {}", channel.link());
    eprintln!("Channel description: {}", channel.description());
    let rss_output = channel.to_string();
    eprintln!(
        "Full RSS preview (first 500 chars): {}",
        &rss_output[..rss_output.len().min(500)]
    );

    Ok(channel)
}
