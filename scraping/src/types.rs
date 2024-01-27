use chromiumoxide::cdp::browser_protocol::performance::Metric;
use reqwest::StatusCode;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq)]
pub enum ScrapingStatus {
    InQueue,
    Completed,
    Error,
}

#[derive(Debug)]
pub struct ScrapingPageQueue(HashMap<String, ScrapingStatus>);

impl ScrapingPageQueue {
    pub fn init(url: &str) -> Self {
        let mut pages = HashMap::new();
        pages.insert(url.to_string(), ScrapingStatus::InQueue);
        Self(pages)
    }

    pub fn insert(&mut self, url: &str, status: ScrapingStatus) {
        self.0.insert(url.to_string(), status);
    }

    pub fn insert_many(&mut self, host: &str, urls: Vec<String>) {
        urls.iter()
            .filter(|url| url.starts_with(host) && !self.0.contains_key(&**url))
            .collect::<Vec<_>>()
            .iter()
            .for_each(|url| {
                self.0.insert(url.to_string(), ScrapingStatus::InQueue);
            });
    }

    pub fn get_mut(&mut self, url: &str) -> Option<&mut ScrapingStatus> {
        self.0.get_mut(url)
    }

    pub fn get_next_queued_url(&self) -> Option<String> {
        self.0
            .iter()
            .find(|(_, status)| **status == ScrapingStatus::InQueue)
            .map(|(url, _)| url.to_string())
    }
}

#[derive(Debug, Default)]
pub struct ScrapedPage {
    pub url: String,
    pub status: StatusCode,
    pub content: Option<String>,
    pub metrics: Option<Vec<Metric>>,
}

impl ScrapedPage {
    pub fn error(url: &str, status: StatusCode) -> Self {
        Self {
            url: url.to_string(),
            status,
            ..Default::default()
        }
    }
}

#[derive(Debug)]
pub struct HtmlElements {
    pub title: Option<String>,
    pub links: Vec<String>,
    pub images: Vec<String>,
    pub scripts: Vec<String>,
    pub metas: HashMap<String, String>,
}
