use anyhow::Result;
use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use pulldown_cmark::{html, Options, Parser};
use rss::{Channel, ChannelBuilder, Guid, Item, ItemBuilder};
use serde::{Deserialize, Deserializer};
use std::{fs, path::Path};
use walkdir::WalkDir;

fn deserialize_date<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;

    if let Some(date_str) = s {
        if let Ok(dt) = DateTime::parse_from_rfc3339(&date_str) {
            return Ok(Some(dt.with_timezone(&Utc)));
        }

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

    pub path: String, // relative path from src
}

pub fn parse_markdown_file(root: &Path, path: &Path) -> Result<Article> {
    let text = fs::read_to_string(path)?;

    let mut lines = text.lines();

    let mut yaml = String::new();

    let mut in_yaml = false;

    // Extract YAML front matter from markdown file

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

    // Remaining content after front matter

    let content = lines.collect::<Vec<_>>().join("\n") + "\n";

    // Deserialize YAML front matter or fallback to defaults

    let fm = if !yaml.trim().is_empty() {
        serde_yaml::from_str(&yaml).unwrap_or_else(|_| FrontMatter {
            title: path
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned(),

            date: None,

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

            date: None,

            author: None,

            description: Some(content.clone()),
        }
    };

    // Compute relative path from root directory

    let rel_path = path.strip_prefix(root).unwrap_or(path);

    let _html_path = rel_path
        .to_string_lossy()
        .replace('\\', "/")
        .replace(".md", ".html")
        .replace("/README.html", "/index.html");

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

        // Accept files with md or markdown extension (case-insensitive)

        if !matches!(

            path.extension().and_then(|e| e.to_str().map(|s| s.to_ascii_lowercase())),

            Some(ref ext) if ext == "md" || ext == "markdown"

        ) {
            continue;
        }

        // Skip SUMMARY.md files as these are special in mdBook

        let file_name = path.file_name().unwrap().to_string_lossy();

        if file_name.eq_ignore_ascii_case("SUMMARY.md") {
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

    // Sort articles by date newest first

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
    eprintln!("build_feed called with site_url = '{}'", site_url);

    let articles = collect_articles(src_dir)?;

    let base_url = site_url.trim_end_matches('/');

    eprintln!("Using base_url = '{}'", base_url);

    let items: Vec<Item> = articles
        .into_iter()
        .map(|article| {
            eprintln!("Generating RSS item for: {}", article.path);

            // Convert src-relative path to HTML path with forward slashes

            let html_path = article
                .path
                .replace('\\', "/")
                .replace(".md", ".html")
                .replace("/README.html", "/index.html");

            let link = format!("{}/{}", base_url, html_path);

            let raw_html = markdown_to_html(
                article
                    .fm
                    .description
                    .as_deref()
                    .unwrap_or(&article.content),
            );

            // Manual XML escaping to avoid issues with rss crate CDATA

            let safe_description = raw_html
                .replace('&', "&amp;")
                .replace('<', "&lt;")
                .replace('>', "&gt;")
                .replace('"', "&quot;")
                .replace('\'', "&apos;");

            let mut item_builder = ItemBuilder::default();

            item_builder.title(Some(article.fm.title.clone()));

            item_builder.link(Some(link.clone()));

            item_builder.description(Some(safe_description));

            item_builder.guid(Some(Guid {
                value: link.clone(),

                permalink: true,
            }));

            if let Some(date) = article.fm.date {
                item_builder.pub_date(Some(date.to_rfc2822()));
            }

            if let Some(author) = &article.fm.author {
                item_builder.author(Some(author.clone()));
            }

            item_builder.build()
        })
        .collect();

    eprintln!("Total RSS items generated: {}", items.len());

    let mut channel_builder = ChannelBuilder::default();

    channel_builder.title(title.to_string());

    channel_builder.link(site_url.to_string());

    let safe_channel_desc = description
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;");

    channel_builder.description(safe_channel_desc);

    channel_builder.items(items);

    channel_builder.generator(Some("mdbook-rss 0.1.0".to_string()));

    let channel = channel_builder.build();

    eprintln!("Final channel item count: {}", channel.items().len());

    Ok(channel)
}
