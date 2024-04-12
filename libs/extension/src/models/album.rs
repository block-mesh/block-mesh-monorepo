// data -> albumUnion -> tracks -> items -> [track -> uri]
// ... tracks -> totalCount
// ... track -> playability -> playable

use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct Playability {
    pub playable: bool,
}

#[derive(Debug, Deserialize)]
pub struct Track {
    pub uri: String,
    pub playability: Playability,
}

#[derive(Debug, Deserialize)]
pub struct Items {
    pub track: Track,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tracks {
    pub items: Vec<Items>,
    pub total_count: usize,
}

#[derive(Debug, Deserialize)]
pub struct AlbumUnion {
    pub tracks: Tracks,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RootData {
    pub album_union: AlbumUnion,
}

#[derive(Debug, Deserialize)]
pub struct AlbumTracksRoot {
    pub data: RootData,
}
