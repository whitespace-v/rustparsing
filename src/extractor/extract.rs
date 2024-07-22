use std::{collections::HashMap, error::Error};

use scraper::{ElementRef, Html};
use serde_json::Value;

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
                    let checked_text = parent
                        .children()
                        .filter_map(|child| ElementRef::wrap(child))
                        .flat_map(|el| el.text())
                        .collect::<String>();
                    res.push(checked_text)
                }
                _ => (),
            }
        }
    }
    res
}
pub fn with_checked_label(document: &Html, selector_str: &str) -> Vec<String> {
    let mut res: Vec<String> = vec![];
    for parent in document.select(&scraper::Selector::parse(&selector_str).unwrap()) {
        for child in parent
            .children()
            .filter_map(|child| ElementRef::wrap(child))
        {
            match child.value().attr("checked") {
                Some("") => res.push(parent.text().collect::<String>().trim().cut_off()),
                _ => (),
            }
        }
    }
    res
}

pub fn extract_ids_from_json_js(
    document: &Html,
    selector_js_str: &str,
    start_str: &str,
    end_str: &str,
    future_selector_start: &str,
    future_selector_delimeter: &str,
    future_selector_end: &str,
    end_with_value: bool,
) -> Vec<HashMap<String, String>> {
    let source = document
        .select(&scraper::Selector::parse(&selector_js_str).unwrap())
        .next()
        .map(|e| e.inner_html())
        .unwrap();

    let start_position = source.find(start_str);
    let start_position = start_position.unwrap() + start_str.len();
    let source = &source[start_position..];
    let end_position = source.find(end_str).unwrap_or_default();
    let mut future: Vec<HashMap<String, String>> = vec![];
    for (key, value) in serde_json::from_str::<Value>(&source[..end_position])
        .unwrap()
        .as_object()
        .unwrap()
    {
        if value
            .as_str()
            .is_some_and(|x| x == "X" || x.parse::<u8>().unwrap() > 0)
        {
            let mut out = future_selector_start.to_owned() + key + future_selector_delimeter;

            if end_with_value {
                out = out + value.as_str().unwrap() + future_selector_end;
            } else {
                out = out + future_selector_end;
            }
            let hash_item = HashMap::from([(key.to_owned(), out)]);
            future.push(hash_item);
        }
    }
    future
}
