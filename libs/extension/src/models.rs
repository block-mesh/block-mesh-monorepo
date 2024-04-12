use serde::Deserialize;
use serde::Serialize;
pub(crate) mod album;
pub(crate) mod albums_playlists;
pub(crate) mod playlist;

/// Use this response type to discard the response payload
#[derive(Debug, Deserialize, Serialize)]
pub struct IgnoredData {}

/// A serializable struct for adding variables to a spotify request
///
/// {"filters":["Albums"],"order":null,"textFilter":"","features":["LIKED_SONGS","YOUR_EPISODES"],"limit":50,"offset":0,"flatten":false,"expandedFolders":[],"folderUri":null,"includeFoldersWhenFlattening":true,"withCuration":false}
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct VariablesV3Items {
    pub filters: Vec<String>,
    pub order: String,
    pub limit: usize,
    pub offset: usize,
    pub text_filter: String,
}

impl Default for VariablesV3Items {
    fn default() -> Self {
        Self {
            filters: Vec::new(),
            order: "Recently Added".to_owned(),
            limit: 50,
            offset: 0,
            text_filter: String::new(),
        }
    }
}

/// A serializable struct for adding variables to a spotify request
///
/// {"uri":"spotify:album:7h5qFidHM4sqhcCHSbiMzL","locale":"","offset":0,"limit":50}
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct VariablesAlbumOrPlaylistTracks {
    pub limit: usize,
    pub offset: usize,
    pub uri: String,
    pub locale: String,
}

impl Default for VariablesAlbumOrPlaylistTracks {
    fn default() -> Self {
        Self {
            limit: 50,
            offset: 0,
            uri: String::new(),
            locale: String::new(),
        }
    }
}

/// A serializable struct for adding variables to a spotify request
/// ````
///     "variables": {
///         "uris": [
///         "spotify:track:7lcFWApQa0PE2Dw4mT8N1I",
///         "spotify:track:1L7mNK7YX1qI42V5lG7kDf",
///         "spotify:track:1YbZZIJOBrfTJ56rqEBpIZ",
///         "spotify:track:0PV4VX0Oy652WmCA3AstAH",
///         "spotify:track:7EknPu06BycAlX2DI6tUnu",
///         "spotify:track:3NpxaxQQdRLjYoU5g02iaw"
///         ],
///         "playlistUri": "spotify:playlist:70Y4rw4zDsRBvyolsQATni",
///         "newPosition": {
///         "moveType": "BOTTOM_OF_PLAYLIST",
///         "fromUid": null
///         }
///     },
/// ```

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct VariablesAddTracksToPlaylist {
    pub uris: Vec<String>,
    pub playlist_uri: String,
    pub new_position: NewPosition,
}

/// Used inside `VariablesAddTracksToPlaylist`
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct NewPosition {
    move_type: String,
    from_uid: Option<String>,
}

impl Default for VariablesAddTracksToPlaylist {
    fn default() -> Self {
        Self {
            uris: Vec::new(),
            playlist_uri: String::new(),
            new_position: NewPosition {
                from_uid: None,
                move_type: "BOTTOM_OF_PLAYLIST".to_owned(),
            },
        }
    }
}

/// The top level structure for including variables as POST payload to a spotify request.
/// Get requests have the top level members as HTTP query params and should be serialized individually.
/// ````
/// {
///     "variables": {
///         ... <T> ...
///     },
///     "operationName": "addToPlaylist",
///     "extensions": {
///         "persistedQuery": {
///         "version": 1,
///         "sha256Hash": "200b7618afd05364c4aafb95e2070249ed87ee3f08fc4d2f1d5d04fdf1a516d9"
///         }
///     }
/// }
/// ```
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Payload<T: Serialize> {
    pub variables: T,
    pub operation_name: String,
    pub extensions: PayloadExtensions,
}

/// Used inside `Payload`
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PayloadExtensions {
    pub persisted_query: PersistedQuery,
}

/// Used inside `Payload`
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PersistedQuery {
    pub version: usize,
    pub sha256_hash: String,
}

impl Default for PersistedQuery {
    fn default() -> Self {
        Self {
            version: 1,
            sha256_hash: String::new(),
        }
    }
}
