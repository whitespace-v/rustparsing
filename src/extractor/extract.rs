use scraper::{ElementRef, Html};

use crate::extend::Cutter;

pub fn extract_attrs(document: &Html, attr: &str, selector_str: &str) -> Vec<String> {
    let mut res: Vec<String> = vec![];
    for e in document.select(&scraper::Selector::parse(&selector_str).unwrap()) {
        res.push(e.value().attr(&attr).unwrap().to_string())
    }
    res
}
pub fn extract_attr(document: &Html, attr: &str, selector_str: &str) -> String {
    document
        .select(&scraper::Selector::parse(&selector_str).unwrap())
        .next()
        .map(|e| e.value().attr(&attr))
        .unwrap()
        .unwrap()
        .cut_off()
}
pub fn extract_value(document: &Html, selector_str: &str) -> String {
    document
        .select(&scraper::Selector::parse(&selector_str).unwrap())
        .next()
        .map(|e| e.text().collect::<String>())
        .unwrap()
        .cut_off()
}
pub fn extract_values(document: &Html, selector_str: &str) -> Vec<String> {
    let mut res: Vec<String> = vec![];
    for e in document.select(&scraper::Selector::parse(&selector_str).unwrap()) {
        res.push(e.text().collect::<String>().trim().cut_off());
    }
    res
}
pub fn with_checked(document: &Html, selector_str: &str) -> Vec<String> {
    let mut res: Vec<String> = vec![];
    for parent in document.select(&scraper::Selector::parse(&selector_str).unwrap()) {
        for child in parent
            .children()
            .filter_map(|child| ElementRef::wrap(child))
        {
            match child.value().attr("checked") {
                Some(_) => {
                    let bbb = parent
                        .children()
                        .filter_map(|child| ElementRef::wrap(child))
                        .flat_map(|el| el.text())
                        .collect::<String>();
                    res.push(bbb)
                }
                _ => (),
            }
        }
    }
    res
}
