use scraper::{Html, Selector};

pub fn html_elements(html: &str) {
    let document = Html::parse_document(html);

    let links = get_all_links(&document);
    let images = get_all_images(&document);
    let scripts = get_all_scripts(&document);

    dbg!(links);
    dbg!(images);
    dbg!(scripts);
}

pub fn get_all_links(document: &Html) -> Vec<String> {
    document
        .select(&Selector::parse("a").unwrap())
        .map(|element| element.value().attr("href"))
        .filter(|attr| attr.is_some())
        .map(|attr| attr.unwrap().to_string())
        .collect::<Vec<_>>()
}

pub fn get_all_images(document: &Html) -> Vec<String> {
    document
        .select(&Selector::parse("img").unwrap())
        .map(|element| element.value().attr("src"))
        .filter(|attr| attr.is_some())
        .map(|attr| attr.unwrap().to_string())
        .collect::<Vec<_>>()
}

pub fn get_all_scripts(document: &Html) -> Vec<String> {
    document
        .select(&Selector::parse("script").unwrap())
        .map(|element| element.value().attr("src"))
        .filter(|attr| attr.is_some())
        .map(|attr| attr.unwrap().to_string())
        .collect::<Vec<_>>()
}
