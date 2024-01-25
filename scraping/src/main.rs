use chromiumoxide::browser::{Browser, BrowserConfig};
use chromiumoxide::cdp::browser_protocol::performance::Metric;
use futures::StreamExt;
use reqwest::StatusCode;
use std::error::Error;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // let mut res = scrape("https://blog.otso.fr", false).await?; // 200
    // let mut res = scrape("https://www.google.com/404", false).await?; // 404
    let mut res = scrape(
        "https://blog.otso.fr/2024-01-17-passer-zsh-fish-shell.html",
        false,
    )
    .await?; // 308
    if res.status.is_redirection() {
        println!("Following redirects for {}", res.url);

        res = scrape(
            "https://blog.otso.fr/2024-01-17-passer-zsh-fish-shell.html",
            true,
        )
        .await?;
    }
    dbg!(res);

    Ok(())
}

// TODO: try to replace later with a lighter alternative (WebDriver, wry, ... ?)
async fn scrape(url: &str, allow_redirects: bool) -> Result<ScrapedPage, Box<dyn Error>> {
    println!("Scraping {}", url);

    let redirect_policy = match allow_redirects {
        true => reqwest::redirect::Policy::default(),
        false => reqwest::redirect::Policy::none(),
    };

    // TODO: won't work on protected website (Datadome and friends) but good enough to start as long as I don't find a way to retrieve the status from chromiumoxide
    let client = reqwest::ClientBuilder::new()
        .redirect(redirect_policy)
        .timeout(Duration::from_secs(5))
        .build()?;
    let response = client.head(url).send().await?;
    let status = response.status();
    if status.is_redirection() {
        println!("Redirection: {:?}", status);
        return Ok(ScrapedPage::error(url, status));
    }

    if !status.is_success() {
        println!("Error: {:?}", status);
        return Ok(ScrapedPage::error(url, status));
    }

    let config = BrowserConfig::builder()
        .disable_cache()
        .incognito()
        // .with_head()
        .respect_https_errors()
        .window_size(1920, 1080)
        .viewport(None)
        .build()?;
    let (mut browser, mut handler) = Browser::launch(config).await?;

    let handle = tokio::task::spawn(async move {
        while let Some(h) = handler.next().await {
            if h.is_err() {
                break;
            }
        }
    });

    let page = browser.new_page(url).await?;
    page.wait_for_navigation_response().await?;

    let content = page.content().await?;
    let metrics = page.metrics().await?;
    let url = page.url().await?.unwrap_or(url.to_string());

    browser.close().await?;
    handle.await?;

    Ok(ScrapedPage {
        url,
        status,
        content: Some(content),
        metrics: Some(metrics),
    })
}

#[derive(Debug, Default)]
struct ScrapedPage {
    url: String,
    status: StatusCode,
    content: Option<String>,
    metrics: Option<Vec<Metric>>,
}

impl ScrapedPage {
    fn error(url: &str, status: StatusCode) -> Self {
        Self {
            url: url.to_string(),
            status,
            ..Default::default()
        }
    }
}
