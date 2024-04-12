// data -> me -> libraryV3 -> items -> item -> data -> uri

use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct Data {
    pub uri: String,
}

#[derive(Debug, Deserialize)]
pub struct Item {
    pub data: Data,
}

#[derive(Debug, Deserialize)]
pub struct Items {
    pub item: Item,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LibraryV3 {
    pub items: Vec<Items>,
    pub total_count: usize,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Me {
    pub library_v3: LibraryV3,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RootData {
    pub me: Me,
}

#[derive(Debug, Deserialize)]
pub struct LibV3ItemsRoot {
    pub data: RootData,
}
