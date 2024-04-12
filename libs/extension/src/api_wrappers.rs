use crate::{
    constants,
    constants::log,
    models,
    models::{
        IgnoredData, Payload, PayloadExtensions, PersistedQuery, VariablesAddTracksToPlaylist,
        VariablesAlbumOrPlaylistTracks, VariablesV3Items,
    },
    BrowserRuntime, RetryAfter,
};

mod utils;
use utils::{build_get_url, execute_http_request, BUILD_POST_URL};

/// Returns IDs of all album tracks.
pub(crate) async fn fetch_album_tracks(
    auth_header_value: &str,
    token_header_value: &str,
    album_id: &str,
    max_number_of_tracks: usize,
    runtime: &BrowserRuntime,
) -> Vec<String> {
    log!("fetch_album_tracks for: {album_id}");

    // request examples
    // https://api-partner.spotify.com/pathfinder/v1/query?operationName=getAlbum&variables=%7B%22uri%22%3A%22spotify%3Aalbum%3A4kWmZoSHYtMxvejnCwv3ID%22%2C%22locale%22%3A%22%22%2C%22offset%22%3A0%2C%22limit%22%3A50%7D&extensions=%7B%22persistedQuery%22%3A%7B%22version%22%3A1%2C%22sha256Hash%22%3A%2246ae954ef2d2fe7732b4b2b4022157b2e18b7ea84f70591ceb164e4de1b5d5d3%22%7D%7D
    // operationName: getAlbum
    // variables: {"uri":"spotify:album:4kWmZoSHYtMxvejnCwv3ID","locale":"","offset":0,"limit":50}
    // extensions: {"persistedQuery":{"version":1,"sha256Hash":"46ae954ef2d2fe7732b4b2b4022157b2e18b7ea84f70591ceb164e4de1b5d5d3"}}

    // get the first page of albums to see how many there are
    let mut variables = VariablesAlbumOrPlaylistTracks {
        uri: [constants::ID_PREFIX_ALBUM, album_id].concat(),
        ..Default::default()
    };

    let mut url = match build_get_url(
        constants::operations::ALBUM_TRACKS,
        &variables,
        constants::persistent_queries::GET_ALBUM,
    ) {
        Ok(v) => v,
        Err(_) => return Vec::new(),
    };

    // get the list of album tracks from Spotify
    let album_tracks = match execute_http_request::<
        models::album::AlbumTracksRoot,
        Option<IgnoredData>,
    >(auth_header_value, token_header_value, &url, None, runtime)
    .await
    {
        Ok(v) => v,
        Err(e) if matches!(e, RetryAfter::Never) => {
            return Vec::new();
        }
        Err(_) => {
            unimplemented!("Retries are not implemented");
        }
    };

    // log!("{:?}", lib_v3_items);

    // get the number of lib_v3_items and calculate the number of pages that can be downloaded
    let total_track_count = album_tracks.data.album_union.tracks.total_count;
    let total_pages = total_track_count / constants::ITEMS_PER_PAGE;
    let total_pages = if total_track_count % constants::ITEMS_PER_PAGE > 0 {
        total_track_count / constants::ITEMS_PER_PAGE + 1
    } else {
        total_pages
    };
    log!("Tracks: {total_track_count}, pages: {total_pages}");

    // a collection of all albums from all pages
    let mut album_tracks = album_tracks
        .data
        .album_union
        .tracks
        .items
        .into_iter()
        .filter_map(|v| {
            if v.track.playability.playable {
                Some(v.track.uri.replace(constants::ID_PREFIX_TRACK, ""))
            } else {
                None
            }
        })
        .collect::<Vec<String>>();

    // check if there are any more pages to fetch
    if total_track_count <= constants::ITEMS_PER_PAGE {
        log!("Playable tracks in {album_id}: {}", album_tracks.len());
        return album_tracks;
    }

    // allocate enough space for all tracks since it is known in advance
    album_tracks.reserve(total_track_count as usize - album_tracks.len());

    // the next page will start where the first one ended
    variables.offset = variables.limit;
    url = match build_get_url(
        constants::operations::ALBUM_TRACKS,
        &variables,
        constants::persistent_queries::GET_ALBUM,
    ) {
        Ok(v) => v,
        Err(_) => return Vec::new(),
    };

    // fetch the rest of the track pages in a loop
    // this will end on the first error
    // TODO: make a more reliable loop when retries are available
    while let Ok(items) =
        execute_http_request::<models::album::AlbumTracksRoot, Option<IgnoredData>>(
            auth_header_value,
            token_header_value,
            &url,
            None,
            runtime,
        )
        .await
    {
        // check if spotify returned any items at all
        if items.data.album_union.tracks.items.is_empty() {
            log!("Spotify returned empty items list");
            log!("Playable tracks in {album_id}: {}", album_tracks.len());
            return album_tracks;
        }

        let mut items = items
            .data
            .album_union
            .tracks
            .items
            .into_iter()
            .filter_map(|v| {
                if v.track.playability.playable {
                    Some(v.track.uri.replace(constants::ID_PREFIX_TRACK, ""))
                } else {
                    None
                }
            })
            .collect::<Vec<String>>();

        // add the list of items to the local collection
        album_tracks.append(&mut items);
        // the next page will start where the first one ended
        variables.offset += variables.limit;
        url = match build_get_url(
            constants::operations::ALBUM_TRACKS,
            &variables,
            constants::persistent_queries::GET_ALBUM,
        ) {
            Ok(v) => v,
            Err(_) => return Vec::new(),
        };

        // protect against very large playlists
        if variables.offset >= max_number_of_tracks {
            log!(
                "Too many tracks ({}), limit {}",
                variables.offset,
                max_number_of_tracks
            );
            break;
        }

        // for debugging
        if variables.offset > 20 {
            break;
        }
    }

    log!("Playable tracks in {album_id}: {}", album_tracks.len());
    album_tracks
}

/// Contains the list of tracks and the owner ID of the playlist
pub(crate) struct PlaylistTracks {
    pub tracks: Vec<String>,
    pub owner_uri: String,
}

/// Returns IDs of all playlist tracks.
pub(crate) async fn fetch_playlist_tracks(
    auth_header_value: &str,
    token_header_value: &str,
    playlist_id: &str,
    max_number_of_tracks: usize,
    runtime: &BrowserRuntime,
) -> Option<PlaylistTracks> {
    log!("fetch_playlist_tracks for: {playlist_id}");

    // request examples
    // https://api-partner.spotify.com/pathfinder/v1/query?operationName=fetchPlaylist&variables=%7B%22uri%22%3A%22spotify%3Aplaylist%3A60C3LAHXga4hw73HeyU7U0%22%2C%22offset%22%3A0%2C%22limit%22%3A25%7D&extensions=%7B%22persistedQuery%22%3A%7B%22version%22%3A1%2C%22sha256Hash%22%3A%2273a3b3470804983e4d55d83cd6cc99715019228fd999d51429cc69473a18789d%22%7D%7D
    // operationName: fetchPlaylist
    // variables: {"uri":"spotify:playlist:60C3LAHXga4hw73HeyU7U0","offset":0,"limit":25}
    // extensions: {"persistedQuery":{"version":1,"sha256Hash":"73a3b3470804983e4d55d83cd6cc99715019228fd999d51429cc69473a18789d"}}

    // get the first page of albums to see how many there are
    let mut variables = VariablesAlbumOrPlaylistTracks {
        uri: [constants::ID_PREFIX_PLAYLIST, playlist_id].concat(),
        ..Default::default()
    };

    let mut url = match build_get_url(
        constants::operations::PLAYLIST_TRACKS,
        &variables,
        constants::persistent_queries::FETCH_PLAYLIST,
    ) {
        Ok(v) => v,
        Err(_) => {
            return None;
        }
    };

    // get the list of album tracks from Spotify
    let tracks = match execute_http_request::<models::playlist::PlaylistRoot, Option<IgnoredData>>(
        auth_header_value,
        token_header_value,
        &url,
        None,
        runtime,
    )
    .await
    {
        Ok(v) => v,
        Err(e) if matches!(e, RetryAfter::Never) => {
            return None;
        }
        Err(_) => {
            unimplemented!("Retries are not implemented");
        }
    };

    log!("{:?}", tracks.data.playlist_v2.owner_v2);

    // get the owner ID
    let owner_uri = tracks.data.playlist_v2.owner_v2.data.uri;

    // get the number of lib_v3_items and calculate the number of pages that can be downloaded
    let total_track_count = tracks.data.playlist_v2.content.total_count;
    let total_pages = total_track_count / constants::ITEMS_PER_PAGE;
    let total_pages = if total_track_count % constants::ITEMS_PER_PAGE > 0 {
        total_track_count / constants::ITEMS_PER_PAGE + 1
    } else {
        total_pages
    };
    log!("Tracks: {total_track_count}, pages: {total_pages}, owner: {owner_uri}");

    // a collection of all albums from all pages
    let mut tracks = tracks
        .data
        .playlist_v2
        .content
        .items
        .into_iter()
        .filter_map(|v| {
            // if the track is playable and is actually a track, then it should be included
            // it may be possible to have other items other than tracks in a playlist (not sure)
            // items of type NotFound have an empty data structure
            if let Some(uri) = v.item_v2.data.uri {
                if v.item_v2.data.playability.playable
                    && uri.starts_with(constants::ID_PREFIX_TRACK)
                {
                    Some(uri.replace(constants::ID_PREFIX_TRACK, ""))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect::<Vec<String>>();

    // check if there are any more pages to fetch
    if total_track_count <= constants::ITEMS_PER_PAGE {
        log!("Playable tracks in {playlist_id}: {}", tracks.len());
        return Some(PlaylistTracks { tracks, owner_uri });
    }

    // allocate enough space for all tracks since it is known in advance
    tracks.reserve(total_track_count as usize - tracks.len());

    // the next page will start where the first one ended
    variables.offset = variables.limit;
    url = match build_get_url(
        constants::operations::PLAYLIST_TRACKS,
        &variables,
        constants::persistent_queries::FETCH_PLAYLIST,
    ) {
        Ok(v) => v,
        Err(_) => return None,
    };

    // fetch the rest of the track pages in a loop
    // this will end on the first error
    // TODO: make a more reliable loop when retries are available
    while let Ok(items) =
        execute_http_request::<models::playlist::PlaylistRoot, Option<IgnoredData>>(
            auth_header_value,
            token_header_value,
            &url,
            None,
            runtime,
        )
        .await
    {
        // check if spotify returned any items at all
        if items.data.playlist_v2.content.items.is_empty() {
            log!("Spotify returned empty items list");
            log!("Playable tracks in {playlist_id}: {}", tracks.len());
            return Some(PlaylistTracks { tracks, owner_uri });
        }

        let mut items = items
            .data
            .playlist_v2
            .content
            .items
            .into_iter()
            .filter_map(|v| {
                if let Some(uri) = v.item_v2.data.uri {
                    if v.item_v2.data.playability.playable
                        && uri.starts_with(constants::ID_PREFIX_TRACK)
                    {
                        Some(uri.replace(constants::ID_PREFIX_TRACK, ""))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<String>>();

        // add the list of items to the local collection
        tracks.append(&mut items);
        // the next page will start where the first one ended
        variables.offset += variables.limit;
        url = match build_get_url(
            constants::operations::PLAYLIST_TRACKS,
            &variables,
            constants::persistent_queries::FETCH_PLAYLIST,
        ) {
            Ok(v) => v,
            Err(_) => return None,
        };

        // protect against very large playlists
        if variables.offset >= max_number_of_tracks {
            log!(
                "Too many tracks ({}), limit {}",
                variables.offset,
                max_number_of_tracks
            );
            break;
        }

        // for debugging
        if variables.offset > 20 {
            break;
        }
    }

    log!("Playable tracks in {playlist_id}: {}", tracks.len());

    Some(PlaylistTracks { tracks, owner_uri })
}

/// Returns IDs of either albums or playlists.
///
/// * filter - either Albums or Playlists, goes into request vars
pub(crate) async fn fetch_lib_v3_items(
    auth_header_value: &str,
    token_header_value: &str,
    filter: &str,
    runtime: &BrowserRuntime,
) -> Vec<String> {
    log!("fetch_lib_v3_items entered, filter: {filter}");

    // request examples
    // https://api-partner.spotify.com/pathfinder/v1/query?operationName=libraryV3&variables=%7B%22filters%22%3A%5B%22Playlists%22%5D%2C%22order%22%3Anull%2C%22textFilter%22%3A%22%22%2C%22features%22%3A%5B%22LIKED_SONGS%22%2C%22YOUR_EPISODES%22%5D%2C%22limit%22%3A50%2C%22offset%22%3A6%2C%22flatten%22%3Afalse%2C%22expandedFolders%22%3A%5B%5D%2C%22folderUri%22%3Anull%2C%22includeFoldersWhenFlattening%22%3Atrue%2C%22withCuration%22%3Afalse%7D&extensions=%7B%22persistedQuery%22%3A%7B%22version%22%3A1%2C%22sha256Hash%22%3A%2217d801ba80f3a3d7405966641818c334fe32158f97e9e8b38f1a92f764345df9%22%7D%7D
    // operationName: libraryV3
    // variables: {"filters":["Playlists"],"order":null,"textFilter":"","features":["LIKED_SONGS","YOUR_EPISODES"],"limit":50,"offset":6,"flatten":false,"expandedFolders":[],"folderUri":null,"includeFoldersWhenFlattening":true,"withCuration":false}

    // get the first page of albums to see how many there are
    let mut variables = VariablesV3Items::default();
    variables.filters.push(filter.to_owned());

    let mut url = match build_get_url(
        constants::operations::ALBUMS_PLAYLISTS,
        &variables,
        constants::persistent_queries::LIBRARY_V3,
    ) {
        Ok(v) => v,
        Err(_) => return Vec::new(),
    };

    // the part of the structure we use for our needs are identical between albums and playlists
    let lib_v3_items = match execute_http_request::<
        models::albums_playlists::LibV3ItemsRoot,
        Option<IgnoredData>,
    >(auth_header_value, token_header_value, &url, None, runtime)
    .await
    {
        Ok(v) => v,
        Err(e) if matches!(e, RetryAfter::Never) => {
            return Vec::new();
        }
        Err(_) => {
            unimplemented!("Retries are not implemented");
        }
    };

    // log!("{:?}", lib_v3_items);

    // get the number of lib_v3_items and calculate the number of pages that can be downloaded
    let total_item_count = lib_v3_items.data.me.library_v3.total_count;
    let total_pages = total_item_count / constants::ITEMS_PER_PAGE;
    let total_pages = if total_item_count % constants::ITEMS_PER_PAGE > 0 {
        total_item_count / constants::ITEMS_PER_PAGE + 1
    } else {
        total_pages
    };
    log!("Items: {total_item_count}, pages: {total_pages}");

    // a collection of all albums from all pages
    let mut all_lib_v3_items = lib_v3_items
        .data
        .me
        .library_v3
        .items
        .into_iter()
        .map(|v| v.item.data.uri)
        .collect::<Vec<String>>();

    // check if there are any more pages to fetch
    if total_item_count <= constants::ITEMS_PER_PAGE {
        log!("Total {filter}: {}", all_lib_v3_items.len());
        return all_lib_v3_items;
    }

    // allocate enough space for all albums since it is known in advance
    all_lib_v3_items.reserve(total_item_count as usize - all_lib_v3_items.len());

    // the next page will start where the first one ended
    variables.offset = variables.limit;
    url = match build_get_url(
        constants::operations::ALBUMS_PLAYLISTS,
        &variables,
        constants::persistent_queries::LIBRARY_V3,
    ) {
        Ok(v) => v,
        Err(_) => return Vec::new(),
    };

    // fetch the rest of the item pages in a loop
    // this will end on the first error
    // TODO: make a more reliable loop when retries are available
    while let Ok(items) = execute_http_request::<
        models::albums_playlists::LibV3ItemsRoot,
        Option<IgnoredData>,
    >(auth_header_value, token_header_value, &url, None, runtime)
    .await
    {
        // check if spotify returned any items at all
        if items.data.me.library_v3.items.is_empty() {
            log!("Spotify returned empty items list");
            log!("Total {filter}: {}", all_lib_v3_items.len());
            return all_lib_v3_items;
        }

        let mut items = items
            .data
            .me
            .library_v3
            .items
            .into_iter()
            .map(|v| v.item.data.uri.replace(constants::ID_PREFIX_ALBUM, ""))
            .collect::<Vec<String>>();

        // add the list of items to the local collection
        all_lib_v3_items.append(&mut items);
        // the next page will start where the first one ended
        variables.offset += variables.limit;
        url = match build_get_url(
            constants::operations::ALBUMS_PLAYLISTS,
            &variables,
            constants::persistent_queries::LIBRARY_V3,
        ) {
            Ok(v) => v,
            Err(_) => return Vec::new(),
        };

        // for debugging
        // if variables.offset > 50 {
        //     break;
        // }
    }

    log!("Total {filter}: {}", all_lib_v3_items.len());
    all_lib_v3_items
}

/// Adds the specified list of tracks to the playlist and reports on its progress.
/// Failed additions are logged, but do not stop processing. Does not panic.
/// Returns the number of tracks added.
pub(crate) async fn add_tracks_to_playlist(
    auth_header_value: &str,
    token_header_value: &str,
    playlist_id: &str,
    tracks_to_add: Vec<String>,
    runtime: &BrowserRuntime,
) -> usize {
    log!(
        "add_tracks_to_playlist for: {playlist_id}, tracks: {}",
        tracks_to_add.len()
    );

    // total counters for logging and returning the result
    let mut tracks_added: usize = 0;
    let mut tracks_missed: usize = 0;

    // request examples
    // POST / https://api-partner.spotify.com/pathfinder/v1/query
    // {"variables":{"uris":["spotify:track:7lcFWApQa0PE2Dw4mT8N1I","spotify:track:1L7mNK7YX1qI42V5lG7kDf","spotify:track:1YbZZIJOBrfTJ56rqEBpIZ","spotify:track:0PV4VX0Oy652WmCA3AstAH","spotify:track:7EknPu06BycAlX2DI6tUnu","spotify:track:3NpxaxQQdRLjYoU5g02iaw"],"playlistUri":"spotify:playlist:70Y4rw4zDsRBvyolsQATni","newPosition":{"moveType":"BOTTOM_OF_PLAYLIST","fromUid":null}},"operationName":"addToPlaylist","extensions":{"persistedQuery":{"version":1,"sha256Hash":"200b7618afd05364c4aafb95e2070249ed87ee3f08fc4d2f1d5d04fdf1a516d9"}}}

    // only the list of tracks will change per request
    // unlike with GET requests, we need to prepare the entire payload as a struct
    let mut payload = Payload::<VariablesAddTracksToPlaylist> {
        variables: VariablesAddTracksToPlaylist {
            playlist_uri: [constants::ID_PREFIX_PLAYLIST, playlist_id].concat(),
            ..Default::default()
        },
        operation_name: constants::operations::ADD_TO_PLAYLIST.to_owned(),
        extensions: PayloadExtensions {
            persisted_query: PersistedQuery {
                sha256_hash: constants::persistent_queries::ADD_TO_PLAYLIST.to_owned(),
                ..Default::default()
            },
        },
    };

    // add tracks in lots of 50s
    // TODO: this loop is completely faulty and has to be redesigned
    // it repeats the same tracks over and over
    for (idx, track) in tracks_to_add.iter().enumerate() {
        // add the track to the list
        payload
            .variables
            .uris
            .push([constants::ID_PREFIX_TRACK, track].concat());

        // send the request to spotify for what is in the payload now
        // if the list is longer than a certain size or if it's the last pass
        // Spotify seems to be OK with 500 tracks at a time
        if idx % 99 == 0 || idx == tracks_to_add.len() - 1 {
            // ignore the response payload for now
            match execute_http_request::<models::IgnoredData, _>(
                auth_header_value,
                token_header_value,
                BUILD_POST_URL,
                Some(&payload),
                runtime,
            )
            .await
            {
                Ok(_) => {
                    log!("Added {} tracks", idx + 1);
                    tracks_added += payload.variables.uris.len();
                    // clear for the next lot of tracks
                    payload.variables.uris.clear();
                }
                Err(e) if matches!(e, RetryAfter::Never) => {
                    log!("Failed to add {} tracks", idx + 1);
                    tracks_missed += payload.variables.uris.len();
                    // clear the list to make room for the next lot of tracks
                    // it may build up until the request is too large if we don't clear it
                    payload.variables.uris.clear();
                    continue;
                }
                Err(_) => {
                    unimplemented!("Retries are not implemented");
                }
            };
        }
    }
    log!("All tracks added: {tracks_added}, missed: {tracks_missed}");

    tracks_added
}
