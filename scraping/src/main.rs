mod html;
mod types;

use crate::types::{ScrapedPage, ScrapingPageQueue, ScrapingStatus};
use chromiumoxide::browser::{Browser, BrowserConfig};
use futures::StreamExt;
use std::error::Error;
use std::time;
use std::time::Duration;
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let start_page = Url::parse("https://blog.otso.fr").unwrap();
    let host = Url::parse(&format!(
        "{}://{}",
        start_page.scheme(),
        start_page.host_str().unwrap()
    ))
    .unwrap();

    let mut pages = ScrapingPageQueue::init(start_page.as_ref());
    pages.insert(start_page.as_ref(), ScrapingStatus::InQueue);

    while let Some(url) = pages.get_next_queued_url() {
        println!("Scraping {}", url);

        let mut res = scrape(&url, false).await?;
        if res.status.is_redirection() {
            tokio::time::sleep(time::Duration::from_millis(250)).await;
            println!("Following redirects for {}", res.url);
            if let Some(p) = pages.get_mut(&res.url) {
                *p = ScrapingStatus::Completed;
            }

            res = scrape(&url, true).await?;
            pages.insert(&res.url, ScrapingStatus::InQueue);
        }

        let elements = html::get_elements(&url, &res.content.unwrap());
        pages.insert_many(host.as_str(), elements.links);

        if let Some(p) = pages.get_mut(&res.url) {
            *p = ScrapingStatus::Completed;
        }
        tokio::time::sleep(time::Duration::from_millis(250)).await;
    }

    dbg!(pages);

    Ok(())
}

// TODO: try to replace later with a lighter alternative (WebDriver, wry, ... ?)
async fn scrape(url: &str, allow_redirects: bool) -> Result<ScrapedPage, Box<dyn Error>> {
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
        .arg("--headless=new")
        .disable_cache()
        .incognito()
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
    page.evaluate(
        r#"() =>
            new Promise((resolve) => {
              if (document.readyState === 'complete') {
                resolve('completed-no-event')
              } else {
                addEventListener('load', () => {
                  resolve('complete-event')
                })
              }
            })
        "#,
    )
    .await?;

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
