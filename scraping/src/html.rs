use crate::types::HtmlElements;
use scraper::{Html, Selector};
use std::collections::HashMap;
use url::Url;

pub fn get_elements(current_url: &str, html: &str) -> HtmlElements {
    let document = Html::parse_document(html);

    let title = get_title(&document);
    let links = get_all_links(current_url, &document);
    let images = get_all_images(current_url, &document);
    let scripts = get_all_scripts(current_url, &document);
    let metas = get_all_metas(&document);

    HtmlElements {
        title,
        links,
        images,
        scripts,
        metas,
    }
}

pub fn get_title(document: &Html) -> Option<String> {
    document
        .select(&Selector::parse("title").unwrap())
        .next()
        .map(|element| element.inner_html())
}

pub fn get_all_links(current_url: &str, document: &Html) -> Vec<String> {
    document
        .select(&Selector::parse("a").unwrap())
        .map(|element| element.value().attr("href"))
        .filter(|attr| attr.is_some())
        .map(|attr| clean_url(current_url, attr.unwrap()))
        .collect::<Vec<_>>()
}

pub fn get_all_images(current_url: &str, document: &Html) -> Vec<String> {
    document
        .select(&Selector::parse("img").unwrap())
        .map(|element| element.value().attr("src"))
        .filter(|attr| attr.is_some())
        .map(|attr| clean_url(current_url, attr.unwrap()))
        .collect::<Vec<_>>()
}

pub fn get_all_scripts(current_url: &str, document: &Html) -> Vec<String> {
    document
        .select(&Selector::parse("script").unwrap())
        .map(|element| element.value().attr("src"))
        .filter(|attr| attr.is_some())
        .map(|attr| clean_url(current_url, attr.unwrap()))
        .collect::<Vec<_>>()
}

pub fn get_all_metas(document: &Html) -> HashMap<String, String> {
    document
        .select(&Selector::parse("meta").unwrap())
        .map(|element| {
            (
                element
                    .value()
                    .attr("name")
                    .or(element.value().attr("property")),
                element.value().attr("content"),
            )
        })
        .filter(|meta| meta.0.is_some() && meta.1.is_some())
        .map(|meta| (meta.0.unwrap().to_string(), meta.1.unwrap().to_string()))
        .collect()
}

pub fn clean_url(current_url: &str, path: &str) -> String {
    if !path.starts_with("http://") && !path.starts_with("https://") {
        let this_page = Url::parse(current_url).unwrap();
        this_page.join(path).unwrap().to_string()
    } else {
        path.to_string()
    }
}
