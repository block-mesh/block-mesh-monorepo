use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub id: String,
    pub username: String,
    pub name: String,
    pub description: Option<String>,
    pub location: Option<String>,
    pub url: Option<String>,
    pub protected: bool,
    pub verified: bool,
    pub followers_count: i32,
    pub following_count: i32,
    pub tweets_count: i32,
    pub listed_count: i32,
    pub created_at: DateTime<Utc>,
    pub profile_image_url: Option<String>,
    pub profile_banner_url: Option<String>,
    pub pinned_tweet_id: Option<String>,
    pub is_blue_verified: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub id: String,
    pub id_str: String,
    pub name: String,
    pub screen_name: String,
    pub location: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub protected: bool,
    pub followers_count: i32,
    pub friends_count: i32,
    pub listed_count: i32,
    pub created_at: String,
    pub favourites_count: i32,
    pub verified: bool,
    pub statuses_count: i32,
    pub profile_image_url_https: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegacyUserRaw {
    pub created_at: Option<String>,
    pub description: Option<String>,
    pub entities: Option<UserEntitiesRaw>,
    #[serde(default)]
    pub favourites_count: i32,
    #[serde(default)]
    pub followers_count: i32,
    #[serde(default)]
    pub friends_count: i32,
    #[serde(default)]
    pub media_count: i32,
    #[serde(default)]
    pub statuses_count: i32,
    pub id_str: Option<String>,
    #[serde(default)]
    pub listed_count: i32,
    pub name: Option<String>,
    pub location: String,
    pub geo_enabled: Option<bool>,
    pub pinned_tweet_ids_str: Option<Vec<String>>,
    pub profile_background_color: Option<String>,
    pub profile_banner_url: Option<String>,
    pub profile_image_url_https: Option<String>,
    #[serde(default)]
    pub protected: bool,
    pub screen_name: Option<String>,
    #[serde(default)]
    pub verified: bool,
    pub has_custom_timelines: Option<bool>,
    pub has_extended_profile: Option<bool>,
    pub url: Option<String>,
    pub can_dm: Option<bool>,
    #[serde(rename = "userId")]
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserEntitiesRaw {
    pub url: Option<UserUrlEntity>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserUrlEntity {
    pub urls: Option<Vec<ExpandedUrl>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpandedUrl {
    pub expanded_url: Option<String>,
}

pub fn parse_profile(user: &LegacyUserRaw, is_blue_verified: Option<bool>) -> Profile {
    let mut profile = Profile {
        id: user.user_id.clone().unwrap_or_default(),
        username: user.screen_name.clone().unwrap_or_default(),
        name: user.name.clone().unwrap_or_default(),
        description: user.description.clone(),
        location: Some(user.location.clone()),
        url: user.url.clone(),
        protected: user.protected,
        verified: user.verified,
        followers_count: user.followers_count,
        following_count: user.friends_count,
        tweets_count: user.statuses_count,
        listed_count: user.listed_count,
        is_blue_verified: Some(is_blue_verified.unwrap_or_default()),
        created_at: user
            .created_at
            .as_ref()
            .and_then(|date_str| {
                DateTime::parse_from_str(date_str, "%a %b %d %H:%M:%S %z %Y")
                    .ok()
                    .map(|dt| dt.with_timezone(&Utc))
            })
            .unwrap_or_else(Utc::now),
        profile_image_url: user
            .profile_image_url_https
            .as_ref()
            .map(|url| url.replace("_normal", "")),
        profile_banner_url: user.profile_banner_url.clone(),
        pinned_tweet_id: user
            .pinned_tweet_ids_str
            .as_ref()
            .and_then(|ids| ids.first().cloned()),
    };

    let url = user
        .entities
        .as_ref()
        .and_then(|entities| entities.url.as_ref())
        .and_then(|url_entity| url_entity.urls.as_ref())
        .and_then(|urls| urls.first())
        .and_then(|first_url| first_url.expanded_url.as_ref());

    if let Some(expanded_url) = url {
        profile.url = Some(expanded_url.clone());
    }

    profile
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResults {
    pub result: UserResult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "__typename")]
#[allow(clippy::large_enum_variant)]
pub enum UserResult {
    User(UserData),
    UserUnavailable(UserUnavailable),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserData {
    pub id: String,
    pub rest_id: String,
    pub affiliates_highlighted_label: Option<serde_json::Value>,
    pub has_graduated_access: bool,
    pub is_blue_verified: bool,
    pub profile_image_shape: String,
    pub legacy: LegacyUserRaw,
    pub smart_blocked_by: bool,
    pub smart_blocking: bool,
    pub legacy_extended_profile: Option<serde_json::Value>,
    pub is_profile_translatable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserUnavailable {
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRaw {
    pub data: UserRawData,
    pub errors: Option<Vec<TwitterApiErrorRaw>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRawData {
    pub user: UserRawUser,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRawUser {
    pub result: UserRawResult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRawResult {
    pub rest_id: Option<String>,
    pub is_blue_verified: Option<bool>,
    pub legacy: LegacyUserRaw,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwitterApiErrorRaw {
    pub message: String,
    pub code: i32,
}
