use anyhow::anyhow;
use regex::Regex;
use scraper::{Html, Selector};

pub fn text_to_num(text: String) -> Option<u32> {
    let text = text.trim_matches(|c| c == ' ');
    if text.is_empty() {
        Some(0)
    } else {
        match text.parse() {
            Ok(i) => Some(i),
            Err(_) => None,
        }
    }
}

pub fn get_number_by_selector(fragment: &Html, selector: &str) -> anyhow::Result<String> {
    let re = Regex::new(r"^\d+").map_err(|e| anyhow!(e.to_string()))?;
    let text = Selector::parse(selector).map_err(|e| anyhow!(e.to_string()))?;
    for element in fragment.select(&text) {
        if let Some(s) = element.value().attr("aria-label") {
            return Ok(re
                .find(s)
                .map(|m| m.as_str())
                .unwrap_or_default()
                .to_string());
        }
    }
    Ok("".to_string())
}

pub fn get_text_by_selector(fragment: &Html, selector: &str) -> anyhow::Result<String> {
    let text = Selector::parse(selector).map_err(|e| anyhow!(e.to_string()))?;
    if let Some(element) = fragment.select(&text).next() {
        let t = element.text().collect();
        return Ok(t);
    }
    Ok("".to_string())
}
