fn main() {
    println!("Hello, world!");
}

use reqwest;
use scraper::{Html, Selector};

async fn page_title(url: &str) -> Option<String> {
    // Send a GET request
    let response = reqwest::get(url).await.ok()?;

    // Extract response text
    let response_text = response.text().await.ok()?;

    // Parse HTML
    let document = Html::parse_document(&response_text);

    // Create a CSS selector for <title>
    let selector = Selector::parse("title").ok()?;

    // Find the first <title> element and extract its inner text
    let title_element = document.select(&selector).next()?;
    Some(title_element.inner_html())
}

// #[tokio::main]
// async fn main() {
//     let url = "https://example.com";
//     match page_title(url).await {
//         Some(title) => println!("Page title: {}", title),
//         None => println!("Could not get page title."),
//     }
// }
