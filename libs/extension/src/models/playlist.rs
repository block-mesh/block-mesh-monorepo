use serde::Deserialize;

// data -> playlistV2 -> content -> items -> [itemV2 -> data -> uri]
// ... content -> totalCount
// ... itemV2 -> data -> playability -> playable
// ... playlistV2 -> ownerV2 -> data -> uri

// Playlist ownership -------------------------------------
#[derive(Debug, Deserialize)]
#[serde(rename(deserialize = "data"))]
pub struct OwnerV2Data {
    /// E.g. `spotify:user:nzmusicianmagazine`
    pub uri: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename(deserialize = "ownerV2"))]
pub struct OwnerV2 {
    pub data: OwnerV2Data,
}

// list of tracks ------------------------------------------
#[derive(Debug, Deserialize, Default)]
pub struct Playability {
    pub playable: bool,
}

#[derive(Debug, Deserialize)]
pub struct ItemV2Data {
    /// E.g. `spotify:track:0tpZIPW5LXXtg6YgKr4q48`
    pub uri: Option<String>,
    #[serde(default)]
    pub playability: Playability,
}

#[derive(Debug, Deserialize)]
pub struct ItemV2 {
    pub data: ItemV2Data,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub item_v2: ItemV2,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content {
    pub items: Vec<Item>,
    pub total_count: usize,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistV2 {
    pub owner_v2: OwnerV2,
    pub content: Content,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Playlist {
    pub playlist_v2: PlaylistV2,
}

// The root node -------------------------------------------
#[derive(Debug, Deserialize)]
pub struct PlaylistRoot {
    pub data: Playlist,
}
