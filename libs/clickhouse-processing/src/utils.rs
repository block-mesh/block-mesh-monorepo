use anyhow::anyhow;
use regex::Regex;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, clickhouse::Row)]
pub struct DataSinkClickHouse {
    #[serde(with = "clickhouse::serde::uuid")]
    pub id: Uuid,
    #[serde(with = "clickhouse::serde::uuid")]
    pub user_id: Uuid,
    pub created_at: u64,
    pub updated_at: u64,
    pub raw: String,
    pub origin: String,
    pub origin_id: String,
    pub user_name: String,
    pub link: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExportData {
    pub link: String,
    pub text: String,
    pub user_name: String,
    pub date: DateTime<Utc>,
    pub id: String,
    pub reply: u64,
    pub retweet: u64,
    pub like: u64,
}

impl ExportData {
    pub fn get_text_by_selector(fragment: &Html, selector: &str) -> anyhow::Result<String> {
        let re = Regex::new(r"^\d+").unwrap();
        let text = Selector::parse(selector).map_err(|e| anyhow!(e.to_string()))?;
        // let iter = fragment.select(&text);
        // let count = iter.count();
        // if count != 1 {
        //     return Err(anyhow!(
        //         "Unexpected select matches {} for selector {}",
        //         count,
        //         selector
        //     ));
        // }
        // let element = fragment.select(&text).take(1).next().unwrap();
        // let mut output_text = "".to_string();
        for element in fragment.select(&text) {
            if let Some(s) = element.value().attr("aria-label") {
                return Ok(re
                    .find(s)
                    .map(|m| m.as_str())
                    .unwrap_or_default()
                    .to_string());
            }
            // println!("x {:?}", x);
            // let t = element.text().collect::<Vec<_>>();
            // let t = t.join(" ");
            // output_text = format!("{} {}", output_text, t);
        }
        Ok("".to_string())
    }

    pub fn text_to_num(text: String) -> anyhow::Result<u64> {
        let text = text.trim_matches(|c| c == ' ');
        if text.is_empty() {
            Ok(0)
        } else {
            match text.parse() {
                Ok(i) => Ok(i),
                Err(e) => Err(anyhow!("Error parsing '{}' | {}", text, e)),
            }
        }
    }
}

impl TryFrom<DataSinkClickHouse> for ExportData {
    type Error = anyhow::Error;

    fn try_from(value: DataSinkClickHouse) -> Result<Self, Self::Error> {
        let fragment = Html::parse_fragment(&value.raw);
        let tweet_text = Self::get_text_by_selector(&fragment, "[data-testid='tweetText']")?;
        let reply_text = Self::get_text_by_selector(&fragment, "[data-testid='reply']")?;
        let retweet_text = Self::get_text_by_selector(&fragment, "[data-testid='retweet']")?;
        let like_text = Self::get_text_by_selector(&fragment, "[data-testid='like']")?;
        let date = DateTime::<Utc>::from_timestamp_nanos(value.created_at as i64);
        Ok(ExportData {
            link: format!("https://x.com{}", value.link),
            user_name: value.user_name,
            text: tweet_text,
            date,
            id: value.origin_id,
            reply: Self::text_to_num(reply_text)?,
            retweet: Self::text_to_num(retweet_text)?,
            like: Self::text_to_num(like_text)?,
        })
    }
}
