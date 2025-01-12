use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::proactive::types::profile::LegacyUserRaw;
use crate::proactive::types::profile::Profile;
use crate::proactive::types::tweets::PlaceRaw;
use crate::proactive::types::tweets::Tweet;

#[derive(Debug, Deserialize, Serialize)]
pub struct Hashtag {
    pub text: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TimelineUserMentionBasicRaw {
    pub id_str: Option<String>,
    pub name: Option<String>,
    pub screen_name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TimelineMediaBasicRaw {
    pub media_url_https: Option<String>,
    pub r#type: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TimelineUrlBasicRaw {
    pub expanded_url: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ExtSensitiveMediaWarningRaw {
    pub adult_content: Option<bool>,
    pub graphic_violence: Option<bool>,
    pub other: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VideoVariant {
    pub bitrate: Option<i32>,
    pub url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VideoInfo {
    pub variants: Option<Vec<VideoVariant>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TimelineMediaExtendedRaw {
    pub id_str: Option<String>,
    pub media_url_https: Option<String>,
    pub ext_sensitive_media_warning: Option<ExtSensitiveMediaWarningRaw>,
    pub r#type: Option<String>,
    pub url: Option<String>,
    pub video_info: Option<VideoInfo>,
    pub ext_alt_text: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchResultRaw {
    pub rest_id: Option<String>,
    pub __typename: Option<String>,
    pub core: Option<UserResultsCore>,
    pub views: Option<Views>,
    pub note_tweet: Option<NoteTweet>,
    pub quoted_status_result: Option<QuotedStatusResult>,
    pub legacy: Option<LegacyTweetRaw>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserResultsCore {
    pub user_results: Option<UserResults>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserResults {
    pub result: Option<UserResult>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserResult {
    pub is_blue_verified: Option<bool>,
    pub legacy: Option<LegacyUserRaw>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Views {
    pub count: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NoteTweet {
    pub note_tweet_results: Option<NoteTweetResults>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NoteTweetResults {
    pub result: Option<NoteTweetResult>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NoteTweetResult {
    pub text: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QuotedStatusResult {
    pub result: Option<Box<SearchResultRaw>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TimelineResultRaw {
    pub result: Option<Box<TimelineResultRaw>>,
    pub rest_id: Option<String>,
    pub __typename: Option<String>,
    pub core: Option<TimelineCore>,
    pub views: Option<TimelineViews>,
    pub note_tweet: Option<TimelineNoteTweet>,
    pub quoted_status_result: Option<Box<TimelineQuotedStatus>>,
    pub legacy: Option<Box<LegacyTweetRaw>>,
    pub tweet: Option<Box<TimelineResultRaw>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TimelineCore {
    pub user_results: Option<TimelineUserResults>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TimelineUserResults {
    pub result: Option<TimelineUserResult>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TimelineUserResult {
    pub is_blue_verified: Option<bool>,
    pub legacy: Option<LegacyUserRaw>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TimelineViews {
    pub count: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TimelineNoteTweet {
    pub note_tweet_results: Option<TimelineNoteTweetResults>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TimelineNoteTweetResults {
    pub result: Option<TimelineNoteTweetResult>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TimelineNoteTweetResult {
    pub text: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TimelineQuotedStatus {
    pub result: Option<Box<TimelineResultRaw>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LegacyTweetRaw {
    pub bookmark_count: Option<i32>,
    pub conversation_id_str: Option<String>,
    pub created_at: Option<String>,
    pub favorite_count: Option<i32>,
    pub full_text: Option<String>,
    pub entities: Option<TweetEntities>,
    pub extended_entities: Option<TweetExtendedEntities>,
    pub id_str: Option<String>,
    pub in_reply_to_status_id_str: Option<String>,
    pub place: Option<PlaceRaw>,
    pub reply_count: Option<i32>,
    pub retweet_count: Option<i32>,
    pub retweeted_status_id_str: Option<String>,
    pub retweeted_status_result: Option<TimelineRetweetedStatus>,
    pub quoted_status_id_str: Option<String>,
    pub time: Option<String>,
    pub user_id_str: Option<String>,
    pub ext_views: Option<TweetExtViews>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TweetEntities {
    pub hashtags: Option<Vec<Hashtag>>,
    pub media: Option<Vec<TimelineMediaBasicRaw>>,
    pub urls: Option<Vec<TimelineUrlBasicRaw>>,
    pub user_mentions: Option<Vec<TimelineUserMentionBasicRaw>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TweetExtendedEntities {
    pub media: Option<Vec<TimelineMediaExtendedRaw>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TimelineRetweetedStatus {
    pub result: Option<TimelineResultRaw>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TweetExtViews {
    pub state: Option<String>,
    pub count: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TimelineGlobalObjectsRaw {
    pub tweets: Option<HashMap<String, Option<LegacyTweetRaw>>>,
    pub users: Option<HashMap<String, Option<LegacyUserRaw>>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TimelineDataRawCursor {
    pub value: Option<String>,
    pub cursor_type: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TimelineDataRawEntity {
    pub id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TimelineDataRawModuleItem {
    pub client_event_info: Option<ClientEventInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ClientEventInfo {
    pub details: Option<ClientEventDetails>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ClientEventDetails {
    pub guide_details: Option<GuideDetails>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GuideDetails {
    pub transparent_guide_details: Option<TransparentGuideDetails>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TransparentGuideDetails {
    pub trend_metadata: Option<TrendMetadata>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TrendMetadata {
    pub trend_name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TimelineDataRawAddEntry {
    pub content: Option<TimelineEntryContent>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TimelineDataRawPinEntry {
    pub content: Option<TimelinePinContent>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TimelinePinContent {
    pub item: Option<TimelineItem>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TimelineDataRawReplaceEntry {
    pub content: Option<TimelineReplaceContent>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TimelineReplaceContent {
    pub operation: Option<TimelineOperation>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TimelineDataRawInstruction {
    pub add_entries: Option<TimelineAddEntries>,
    pub pin_entry: Option<TimelineDataRawPinEntry>,
    pub replace_entry: Option<TimelineDataRawReplaceEntry>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TimelineAddEntries {
    pub entries: Option<Vec<TimelineDataRawAddEntry>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TimelineDataRaw {
    pub instructions: Option<Vec<TimelineDataRawInstruction>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TimelineV1 {
    pub global_objects: Option<TimelineGlobalObjectsRaw>,
    pub timeline: Option<TimelineDataRaw>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct QueryTweetsResponse {
    pub tweets: Vec<Tweet>,
    pub next: Option<String>,
    pub previous: Option<String>,
}

impl QueryTweetsResponse {
    pub fn merge(&mut self, mut other: Vec<Tweet>) {
        self.tweets.append(&mut other);
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct QueryProfilesResponse {
    pub profiles: Vec<Profile>,
    pub next: Option<String>,
    pub previous: Option<String>,
}

impl QueryProfilesResponse {
    pub fn merge(&mut self, mut other: Vec<Profile>) {
        self.profiles.append(&mut other);
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TimelineEntryContent {
    pub item: Option<TimelineItem>,
    pub operation: Option<TimelineOperation>,
    pub timeline_module: Option<TimelineModule>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TimelineItem {
    pub content: Option<TimelineContent>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TimelineContent {
    pub tweet: Option<TimelineDataRawEntity>,
    pub user: Option<TimelineDataRawEntity>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TimelineOperation {
    pub cursor: Option<TimelineDataRawCursor>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TimelineModule {
    pub items: Option<Vec<TimelineModuleItemWrapper>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TimelineModuleItemWrapper {
    pub item: Option<TimelineDataRawModuleItem>,
}
