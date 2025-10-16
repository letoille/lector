use anyhow::Result;
use readability_rust;
use regex::Regex;
use scraper::{Html, Selector};
use url::Url;

pub struct ScrapedData {
    pub title: String,
    pub markdown: String,
    pub author: Option<String>,
    pub site_name: Option<String>,
    pub published_time: Option<String>,
}

pub async fn scrape(url: &str) -> Result<ScrapedData> {
    let base_url = Url::parse(url)?;
    let product = fetch_and_extract(&base_url).await?;
    let content = product.content.clone().unwrap_or_default();
    let re = Regex::new(r"<img[^>]*>").unwrap();
    let new_content = re.replace_all(&content, "");
    let markdown = html2md::parse_html(&new_content);
    let re = Regex::new(r"\*\*").unwrap();
    let markdown = re.replace_all(&markdown, "").to_string();
    let title = product.title.clone().unwrap_or_default();

    let document = Html::parse_document(&content);

    let author_selector = Selector::parse("meta[name='author']").unwrap();
    let author = document.select(&author_selector).next().and_then(|element| element.value().attr("content").map(|s| s.to_string()));

    let site_name_selector = Selector::parse("meta[property='og:site_name']").unwrap();
    let site_name = document.select(&site_name_selector).next().and_then(|element| element.value().attr("content").map(|s| s.to_string()));

    let published_time_selector = Selector::parse("meta[property='article:published_time']").unwrap();
    let published_time = document.select(&published_time_selector).next().and_then(|element| element.value().attr("content").map(|s| s.to_string()));

    Ok(ScrapedData {
        title,
        markdown,
        author,
        site_name,
        published_time,
    })
}

async fn fetch_and_extract(url: &Url) -> Result<readability_rust::Article> {
    let client = reqwest::Client::builder()
    .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.36")
    .build()?;

    let res = client.get(url.as_str()).send().await?;
    let text = res.text().await?;

    let mut parser = readability_rust::Readability::new(&text, None).unwrap();
    let product = parser.parse().unwrap();
    Ok(product)
}
