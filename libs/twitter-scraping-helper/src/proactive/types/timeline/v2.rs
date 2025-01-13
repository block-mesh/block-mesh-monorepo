use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::proactive::types::{
    profile::LegacyUserRaw,
    tweets::{Mention, Photo, Tweet, Video},
};

use super::v1::{LegacyTweetRaw, TimelineMediaExtendedRaw, TimelineResultRaw};

#[derive(Debug, Deserialize, Serialize)]
pub struct Timeline {
    pub timeline: Option<TimelineItems>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TimelineContent {
    pub instructions: Option<Vec<TimelineInstruction>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TimelineData {
    pub user: Option<TimelineUser>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TimelineEntities {
    pub hashtags: Option<Vec<Hashtag>>,
    pub user_mentions: Option<Vec<UserMention>>,
    pub urls: Option<Vec<UrlEntity>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TimelineEntry {
    #[serde(rename = "entryId")]
    pub entry_id: Option<String>,
    pub content: Option<EntryContent>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TimelineEntryItemContent {
    pub item_type: Option<String>,
    pub tweet_display_type: Option<String>,
    pub tweet_result: Option<TweetResult>,
    pub tweet_results: Option<TweetResult>,
    pub user_display_type: Option<String>,
    pub user_results: Option<TimelineUserResult>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TimelineEntryItemContentRaw {
    #[serde(rename = "itemType")]
    pub item_type: Option<String>,
    #[serde(rename = "tweetDisplayType")]
    pub tweet_display_type: Option<String>,
    #[serde(rename = "tweetResult")]
    pub tweet_result: Option<TweetResultRaw>,
    pub tweet_results: Option<TweetResultRaw>,
    #[serde(rename = "userDisplayType")]
    pub user_display_type: Option<String>,
    pub user_results: Option<TimelineUserResultRaw>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TimelineItems {
    pub instructions: Option<Vec<TimelineInstruction>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TimelineUser {
    pub result: Option<TimelineUserResult>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TimelineUserResult {
    pub rest_id: Option<String>,
    pub legacy: Option<LegacyUserRaw>,
    pub is_blue_verified: Option<bool>,
    pub timeline_v2: Option<Box<TimelineV2>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TimelineUserResultRaw {
    pub result: Option<TimelineUserResult>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TimelineV2 {
    pub data: Option<TimelineData>,
    pub timeline: Option<TimelineItems>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ThreadedConversation {
    pub data: Option<ThreadedConversationData>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ThreadedConversationData {
    pub threaded_conversation_with_injections_v2: Option<TimelineContent>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TweetResult {
    pub result: Option<TimelineResultRaw>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TweetResultRaw {
    pub result: Option<TimelineResultRaw>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EntryContent {
    #[serde(rename = "cursorType")]
    pub cursor_type: Option<String>,
    pub value: Option<String>,
    pub items: Option<Vec<EntryItem>>,
    #[serde(rename = "itemContent")]
    pub item_content: Option<TimelineEntryItemContent>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EntryItem {
    #[serde(rename = "entryId")]
    pub entry_id: Option<String>,
    pub item: Option<ItemContent>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ItemContent {
    pub content: Option<TimelineEntryItemContent>,
    #[serde(rename = "itemContent")]
    pub item_content: Option<TimelineEntryItemContent>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Hashtag {
    pub text: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UrlEntity {
    pub expanded_url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserMention {
    pub id_str: Option<String>,
    pub name: Option<String>,
    pub screen_name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TimelineInstruction {
    pub entries: Option<Vec<TimelineEntry>>,
    pub entry: Option<TimelineEntry>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchEntryRaw {
    #[serde(rename = "entryId")]
    pub entry_id: String,
    #[serde(rename = "sortIndex")]
    pub sort_index: String,
    pub content: Option<SearchEntryContentRaw>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchEntryContentRaw {
    #[serde(rename = "cursorType")]
    pub cursor_type: Option<String>,
    #[serde(rename = "entryType")]
    pub entry_type: Option<String>,
    #[serde(rename = "__typename")]
    pub typename: Option<String>,
    pub value: Option<String>,
    pub items: Option<Vec<SearchEntryItemRaw>>,
    #[serde(rename = "itemContent")]
    pub item_content: Option<TimelineEntryItemContentRaw>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchEntryItemRaw {
    pub item: Option<SearchEntryItemInnerRaw>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchEntryItemInnerRaw {
    pub content: Option<TimelineEntryItemContentRaw>,
}

pub fn parse_legacy_tweet(
    user: Option<&LegacyUserRaw>,
    tweet: Option<&LegacyTweetRaw>,
) -> anyhow::Result<Tweet> {
    let tweet = tweet.ok_or(anyhow::format_err!(
        "Tweet was not found in the timeline object",
    ))?;

    let user = user.ok_or(anyhow::format_err!(
        "User was not found in the timeline object",
    ))?;

    let id_str = tweet
        .id_str
        .as_ref()
        .or(tweet.conversation_id_str.as_ref())
        .ok_or(anyhow::format_err!("Tweet ID was not found in object"))?;

    let hashtags = tweet
        .entities
        .as_ref()
        .and_then(|e| e.hashtags.as_ref())
        .map(|h| h.iter().filter_map(|h| h.text.clone()).collect())
        .unwrap_or_default();

    let mentions = tweet
        .entities
        .as_ref()
        .and_then(|e| e.user_mentions.as_ref())
        .map(|mentions| {
            mentions
                .iter()
                .map(|m| Mention {
                    id: m.id_str.clone().unwrap_or_default(),
                    name: m.name.clone(),
                    username: m.screen_name.clone(),
                })
                .collect()
        })
        .unwrap_or_default();

    let (photos, videos, _) = if let Some(media) = tweet
        .extended_entities
        .as_ref()
        .and_then(|extended_entities| extended_entities.media.as_ref())
    {
        parse_media_groups(media)
    } else {
        (Vec::new(), Vec::new(), false)
    };

    let mut tweet = Tweet {
        bookmark_count: tweet.bookmark_count,
        conversation_id: tweet.conversation_id_str.clone(),
        id: Some(id_str.clone()),
        hashtags,
        likes: tweet.favorite_count,
        mentions,
        name: user.name.clone(),
        permanent_url: Some(format!(
            "https://twitter.com/{}/status/{}",
            user.screen_name.as_ref().unwrap_or(&String::new()),
            id_str
        )),
        photos,
        replies: tweet.reply_count,
        retweets: tweet.retweet_count,
        text: tweet.full_text.clone(),
        thread: Vec::new(),
        urls: tweet
            .entities
            .as_ref()
            .and_then(|e| e.urls.as_ref())
            .map(|urls| urls.iter().filter_map(|u| u.expanded_url.clone()).collect())
            .unwrap_or_default(),
        user_id: tweet.user_id_str.clone(),
        username: user.screen_name.clone(),
        videos,
        is_quoted: Some(false),
        is_reply: Some(false),
        is_retweet: Some(false),
        is_pin: Some(false),
        sensitive_content: Some(false),
        quoted_status: None,
        quoted_status_id: tweet.quoted_status_id_str.clone(),
        in_reply_to_status_id: tweet.in_reply_to_status_id_str.clone(),
        retweeted_status: None,
        retweeted_status_id: None,
        views: None,
        time_parsed: None,
        timestamp: None,
        place: tweet.place.clone(),
        in_reply_to_status: None,
        is_self_thread: None,
        poll: None,
        created_at: tweet.created_at.clone(),
        ext_views: None,
        quote_count: None,
        reply_count: None,
        retweet_count: None,
        screen_name: None,
        thread_id: None,
    };

    if let Some(created_at) = &tweet.created_at {
        if let Ok(time) = chrono::DateTime::parse_from_str(created_at, "%a %b %d %H:%M:%S %z %Y") {
            tweet.time_parsed = Some(time.with_timezone(&Utc));
            tweet.timestamp = Some(time.timestamp());
        }
    }

    if let Some(views) = tweet.ext_views {
        tweet.views = Some(views);
    }

    Ok(tweet)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryTweetsResponse {
    pub tweets: Vec<Tweet>,
    pub next: Option<String>,
    pub previous: Option<String>,
}

pub fn parse_media_groups(media: &[TimelineMediaExtendedRaw]) -> (Vec<Photo>, Vec<Video>, bool) {
    let mut photos = Vec::new();
    let mut videos = Vec::new();
    let mut sensitive_content = false;

    for m in media
        .iter()
        .filter(|m| m.id_str.is_some() && m.media_url_https.is_some())
    {
        match m.r#type.as_deref() {
            Some("photo") => {
                photos.push(Photo {
                    id: m.id_str.clone().unwrap(),
                    url: m.media_url_https.clone().unwrap(),
                    alt_text: m.ext_alt_text.clone(),
                });
            }
            Some("video") => {
                videos.push(parse_video(m));
            }
            _ => {}
        }

        if let Some(warning) = &m.ext_sensitive_media_warning {
            sensitive_content = warning.adult_content.unwrap_or(false)
                || warning.graphic_violence.unwrap_or(false)
                || warning.other.unwrap_or(false);
        }
    }

    (photos, videos, sensitive_content)
}

fn parse_video(m: &TimelineMediaExtendedRaw) -> Video {
    let mut video = Video {
        id: m.id_str.clone().unwrap(),
        preview: m.media_url_https.clone().unwrap(),
        url: None,
    };

    let mut max_bitrate = 0;
    if let Some(video_info) = &m.video_info {
        if let Some(variants) = &video_info.variants {
            for variant in variants {
                if let (Some(bitrate), Some(url)) = (&variant.bitrate, &variant.url) {
                    if *bitrate > max_bitrate {
                        let mut variant_url = url.clone();
                        if let Some(idx) = variant_url.find("?tag=10") {
                            variant_url = variant_url[..idx + 1].to_string();
                        }
                        video.url = Some(variant_url);
                        max_bitrate = *bitrate;
                    }
                }
            }
        }
    }

    video
}
