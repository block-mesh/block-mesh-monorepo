use super::{
    v1::{QueryProfilesResponse, QueryTweetsResponse},
    v2::{parse_legacy_tweet, SearchEntryRaw},
};
use crate::types::profile::parse_profile;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct SearchTimeline {
    pub data: Option<SearchData>,
}

#[derive(Debug, Deserialize)]
pub struct SearchData {
    pub search_by_raw_query: Option<SearchByRawQuery>,
}

#[derive(Debug, Deserialize)]
pub struct SearchByRawQuery {
    pub search_timeline: Option<SearchTimelineData>,
}

#[derive(Debug, Deserialize)]
pub struct SearchTimelineData {
    pub timeline: Option<TimelineData>,
}

#[derive(Debug, Deserialize)]
pub struct TimelineData {
    pub instructions: Option<Vec<SearchInstruction>>,
}

#[derive(Debug, Deserialize)]
pub struct SearchInstruction {
    pub entries: Option<Vec<SearchEntryRaw>>,
    pub entry: Option<SearchEntryRaw>,
    #[serde(rename = "type")]
    pub instruction_type: Option<String>,
}

pub fn parse_tweets(timeline: &SearchTimeline) -> QueryTweetsResponse {
    let mut bottom_cursor = None;
    let mut top_cursor = None;
    let mut tweets = Vec::new();

    let instructions = timeline
        .data
        .as_ref()
        .and_then(|data| data.search_by_raw_query.as_ref())
        .and_then(|search| search.search_timeline.as_ref())
        .and_then(|timeline| timeline.timeline.as_ref())
        .and_then(|timeline| timeline.instructions.as_ref())
        .unwrap_or(const { &Vec::new() });

    println!("parse_tweets instructions.len = {}", instructions.len());

    for instruction in instructions {
        if let Some(instruction_type) = &instruction.instruction_type {
            if matches!(
                instruction_type.as_str(),
                "TimelineAddEntries" | "TimelineReplaceEntry"
            ) {
                if let Some(content) = instruction.entry.as_ref().and_then(|e| e.content.as_ref()) {
                    match content.cursor_type.as_deref() {
                        Some("Bottom") => {
                            bottom_cursor = content.value.clone();
                            continue;
                        }
                        Some("Top") => {
                            top_cursor = content.value.clone();
                            continue;
                        }
                        _ => {}
                    }
                }

                let entries = instruction
                    .entries
                    .as_ref()
                    .unwrap_or(const { &Vec::new() });

                for entry in entries {
                    (|| -> Option<()> {
                        let content = entry.content.as_ref()?;

                        if let Some(item_content) = content
                            .item_content
                            .as_ref()
                            .filter(|it| it.tweet_display_type.as_deref() == "Tweet".into())
                        {
                            let tweet_results = item_content.tweet_results.as_ref()?;
                            let result = tweet_results.result.as_ref()?;

                            let user_legacy = result
                                .core
                                .as_ref()
                                .and_then(|core| core.user_results.as_ref())
                                .and_then(|user_results| user_results.result.as_ref())
                                .and_then(|result| result.legacy.as_ref());

                            if let Ok(tweet_result) =
                                parse_legacy_tweet(user_legacy, result.legacy.as_deref())
                            {
                                if tweet_result.views.is_none() {
                                    if let Some(Ok(view_count)) = result
                                        .views
                                        .as_ref()
                                        .and_then(|views| views.count.as_ref())
                                        .map(|count| count.parse::<i32>())
                                    {
                                        let mut tweet = tweet_result;
                                        tweet.views = Some(view_count);
                                        tweets.push(tweet);
                                    }
                                } else {
                                    tweets.push(tweet_result);
                                }
                            }
                        } else if let Some(cursor_type) = &content.cursor_type {
                            match cursor_type.as_str() {
                                "Bottom" => bottom_cursor = content.value.clone(),
                                "Top" => top_cursor = content.value.clone(),
                                _ => {}
                            }
                        }

                        Some(())
                    })();
                }
            }
        }
    }

    QueryTweetsResponse {
        tweets,
        next: bottom_cursor,
        previous: top_cursor,
    }
}

pub fn parse_users(timeline: &SearchTimeline) -> QueryProfilesResponse {
    let instructions = timeline
        .data
        .as_ref()
        .and_then(|d| d.search_by_raw_query.as_ref())
        .and_then(|s| s.search_timeline.as_ref())
        .and_then(|t| t.timeline.as_ref())
        .and_then(|t| t.instructions.as_ref())
        .unwrap_or(const { &Vec::new() });

    let (mut bottom_cursor, mut top_cursor) = (None, None);
    let mut profiles = Vec::new();

    for instr in instructions {
        if !matches!(
            instr.instruction_type.as_deref(),
            Some("TimelineAddEntries" | "TimelineReplaceEntry")
        ) {
            continue;
        }

        if let Some((cursor_type, value)) = instr
            .entry
            .as_ref()
            .and_then(|e| e.content.as_ref())
            .and_then(|c| c.cursor_type.as_deref().map(|t| (t, c.value.clone())))
        {
            match cursor_type {
                "Bottom" => {
                    bottom_cursor = value;
                    continue;
                }
                "Top" => {
                    top_cursor = value;
                    continue;
                }
                _ => {}
            }
        }

        let entries = instr.entries.as_ref().unwrap_or(const { &Vec::new() });

        for entry in entries {
            (|| -> Option<()> {
                let content = entry.content.as_ref()?;

                if let Some(item) = &content.item_content {
                    if item.user_display_type.as_deref() == Some("User") {
                        let result = item.user_results.as_ref()?.result.as_ref()?;
                        let legacy = result.legacy.as_ref()?;
                        let mut profile = parse_profile(legacy, result.is_blue_verified);
                        if profile.id.is_empty() {
                            profile.id = result.rest_id.clone().unwrap_or_default();
                        }
                        profiles.push(profile);
                    }
                } else if let Some(t) = content.cursor_type.as_deref() {
                    match t {
                        "Bottom" => bottom_cursor = content.value.clone(),
                        "Top" => top_cursor = content.value.clone(),
                        _ => {}
                    }
                }

                Some(())
            })();
        }
    }

    QueryProfilesResponse {
        profiles,
        next: bottom_cursor,
        previous: top_cursor,
    }
}

#[derive(Clone, Serialize, Deserialize, Default, Debug, strum::EnumString, strum::Display)]
pub enum Product {
    #[default]
    #[serde(rename = "Top")]
    Top,
    #[serde(rename = "Latest")]
    Latest,
    #[serde(rename = "Photos")]
    Photos,
    #[serde(rename = "People")]
    People,
    #[serde(rename = "Videos")]
    Videos,
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub enum QuerySource {
    #[serde(rename = "typed_query")]
    #[default]
    TypedQuery,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Features(Value);

impl Default for Features {
    fn default() -> Self {
        Self(serde_json::json!({
            "rweb_lists_timeline_redesign_enabled": true,
            "responsive_web_graphql_exclude_directive_enabled": true,
            "verified_phone_label_enabled": false,
            "creator_subscriptions_tweet_preview_api_enabled": true,
            "responsive_web_graphql_timeline_navigation_enabled": true,
            "responsive_web_graphql_skip_user_profile_image_extensions_enabled": false,
            "tweetypie_unmention_optimization_enabled": true,
            "responsive_web_edit_tweet_api_enabled": true,
            "graphql_is_translatable_rweb_tweet_is_translatable_enabled": true,
            "view_counts_everywhere_api_enabled": true,
            "longform_notetweets_consumption_enabled": true,
            "responsive_web_twitter_article_tweet_consumption_enabled": false,
            "tweet_awards_web_tipping_enabled": false,
            "freedom_of_speech_not_reach_fetch_enabled": true,
            "standardized_nudges_misinfo": true,
            "tweet_with_visibility_results_prefer_gql_limited_actions_policy_enabled": true,
            "longform_notetweets_rich_text_read_enabled": true,
            "longform_notetweets_inline_media_enabled": true,
            "responsive_web_media_download_video_enabled": false,
            "responsive_web_enhance_cards_enabled": false,
        }))
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Default)]
pub struct FieldToggles {
    pub with_article_rich_content_state: bool,
}
#[derive(Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Variables {
    pub raw_query: String,
    pub count: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    pub query_source: QuerySource,
    pub product: Product,
}

#[derive(Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SearchTimelineParams {
    pub variables: Variables,
    pub features: Features,
    pub field_toggles: FieldToggles,
}

impl SearchTimelineParams {
    pub fn params(&self) -> anyhow::Result<Vec<(&str, String)>> {
        let params = &[
            ("variables", serde_json::to_string(&self.variables)?),
            ("features", serde_json::to_string(&self.features)?),
            ("fieldToggles", serde_json::to_string(&self.field_toggles)?),
        ];
        Ok(params.to_vec())
    }

    pub fn update_cursor(&mut self, cursor: Option<String>) {
        if let Some(cursor) = cursor {
            self.variables.cursor = Some(cursor);
        }
    }

    pub fn update_raw_query(&mut self, raw_query: &str) {
        self.variables.raw_query = raw_query.to_string();
    }

    pub fn update_count(&mut self, count: u32) {
        if count > 50 {
            self.variables.count = 50
        } else {
            self.variables.count = count;
        }
    }

    pub fn update_product(&mut self, product: Product) {
        self.variables.product = product;
    }
}
