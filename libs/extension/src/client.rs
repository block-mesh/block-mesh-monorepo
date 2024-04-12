use crate::BrowserRuntime;
use std::collections::HashSet;

// use std::time::Duration;
use crate::{
    api_wrappers::{
        add_tracks_to_playlist, fetch_album_tracks, fetch_lib_v3_items, fetch_playlist_tracks,
    },
    constants,
    constants::log,
    report_progress,
};
use rand::seq::SliceRandom;

/// Generates a random playlist of a predefined size out of everything stored in the user library.
pub(crate) async fn generate_random_playlist(
    auth_header_value: &str,
    token_header_value: &str,
    target_playlist_id: &str,
    user_uri: &str,
    number_of_tracks_to_add: usize,
    runtime: &BrowserRuntime,
) -> Result<String, String> {
    report_progress("Eclectic work started");
    report_progress("Fetching details of the target playlist");

    log!("User: {user_uri}, target playlist: {target_playlist_id}");

    // get details of the target playlist - tracks and the owner to see if we can add tracks to it
    // it used to be possible to add tracks to collaborative playlists, but I can't find how it's done now
    // spotify sucks.
    // TODO: retrieve playlist name for logging
    let (target_playlist_tracks, owner_uri) = match fetch_playlist_tracks(
        auth_header_value,
        token_header_value,
        target_playlist_id,
        1000,
        runtime,
    )
    .await
    {
        Some(v) => (v.tracks, v.owner_uri),
        None => {
            // cannot proceed if the target playlist does not exist
            return Err("Cannot fetch target playlist details from Spotify".to_owned());
        }
    };

    report_progress("Target playlist details fetched");

    // check if the playlis is owned by the current user
    if user_uri != owner_uri {
        log!("Playlist owner mismatch: {owner_uri}/{user_uri}");
        return Err("Cannot add tracks to someone else's playlist. Try again with a playlist you created yourself.".to_owned());
    }

    report_progress("Fetching target playlist tracks");

    let target_playlist_tracks = target_playlist_tracks
        .into_iter()
        .collect::<HashSet<String>>();
    report_progress(&format!(
        "Found {} tracks in the target playlist",
        target_playlist_tracks.len()
    ));

    report_progress("Fetching list of albums from My Library");

    // collect all album IDs
    let all_albums =
        fetch_lib_v3_items(auth_header_value, token_header_value, "Albums", runtime).await;

    // remove the repetitive prefix
    let mut all_albums = all_albums
        .into_iter()
        // .take(5) // uncomment for debugging to limit the number of albums
        .map(|v| v.replace(constants::ID_PREFIX_ALBUM, ""))
        .collect::<Vec<String>>();

    // log!("{}", all_albums.join("\n"));
    report_progress(&format!("Found {} albums in the library", all_albums.len()));
    report_progress("Fetching list of playlists from My Library");

    // repeat the same for playlists - collect all playlist IDs

    // request examples
    // https://api-partner.spotify.com/pathfinder/v1/query?operationName=libraryV3&variables=%7B%22filters%22%3A%5B%22Playlists%22%5D%2C%22order%22%3Anull%2C%22textFilter%22%3A%22%22%2C%22features%22%3A%5B%22LIKED_SONGS%22%2C%22YOUR_EPISODES%22%5D%2C%22limit%22%3A50%2C%22offset%22%3A6%2C%22flatten%22%3Afalse%2C%22expandedFolders%22%3A%5B%5D%2C%22folderUri%22%3Anull%2C%22includeFoldersWhenFlattening%22%3Atrue%2C%22withCuration%22%3Afalse%7D&extensions=%7B%22persistedQuery%22%3A%7B%22version%22%3A1%2C%22sha256Hash%22%3A%2217d801ba80f3a3d7405966641818c334fe32158f97e9e8b38f1a92f764345df9%22%7D%7D
    // operationName: libraryV3
    // variables: {"filters":["Playlists"],"order":null,"textFilter":"","features":["LIKED_SONGS","YOUR_EPISODES"],"limit":50,"offset":6,"flatten":false,"expandedFolders":[],"folderUri":null,"includeFoldersWhenFlattening":true,"withCuration":false}

    let all_playlists =
        fetch_lib_v3_items(auth_header_value, token_header_value, "Playlists", runtime).await;

    // remove the repetitive prefix and exclude the current playlist
    let mut all_playlists = all_playlists
        .into_iter()
        .filter_map(|v| {
            let v = v.replace(constants::ID_PREFIX_PLAYLIST, "");
            if v == target_playlist_id {
                None
            } else {
                Some(v)
            }
        })
        .collect::<Vec<String>>();

    // log!("{}", all_playlists.join("\n"));
    report_progress(&format!(
        "Found {} playlists in the library",
        all_playlists.len()
    ));

    // randomize the list of albums and playlists
    let mut rng = rand::thread_rng();
    all_playlists.shuffle(&mut rng);
    all_albums.shuffle(&mut rng);

    // a list of tracks selected for the addition to the new playlist
    let mut selected_tracks: HashSet<String> = HashSet::new();

    // a list of tracks per that were not selected
    let mut stashed_tracks: HashSet<String> = HashSet::new();

    // go thru all albums
    report_progress(&format!(
        "Selecting random tracks from {} albums",
        all_albums.len()
    ));
    for album_id in all_albums {
        // get album tracks, shuffle and add top N tracks to the selected list
        let mut album_tracks = fetch_album_tracks(
            auth_header_value,
            token_header_value,
            &album_id,
            50,
            runtime,
        )
        .await;

        if album_tracks.is_empty() {
            log!("Empty album {album_id}");
            continue;
        } else if album_tracks.len() <= constants::MIN_TRACKS_PER_ALBUM {
            // the album is too small and all tracks should be added
            log!(
                "Sel: {}, stash: {}, adding all {} tracks from album {album_id}",
                selected_tracks.len(),
                stashed_tracks.len(),
                album_tracks.len(),
            );
            let _: Vec<_> = album_tracks
                .into_iter()
                .map(|v| selected_tracks.insert(v))
                .collect();
        } else {
            // select N random tracks
            album_tracks.shuffle(&mut rng);
            let _: Vec<_> = album_tracks
                .drain(..constants::MIN_TRACKS_PER_ALBUM)
                .map(|v| selected_tracks.insert(v))
                .collect();

            // stash the remaining tracks
            let _: Vec<_> = album_tracks
                .into_iter()
                .map(|v| stashed_tracks.insert(v))
                .collect();
            log!(
                "Sel: {}, stash: {}, added {} tracks from album {album_id}",
                selected_tracks.len(),
                stashed_tracks.len(),
                constants::MIN_TRACKS_PER_ALBUM,
            );
        }

        // exit if there are enough tracks for the playlist
        if selected_tracks.len() >= number_of_tracks_to_add {
            break;
        }
    }

    log!(
        "Selected tracks: {}, stash tracks: {}",
        selected_tracks.len(),
        stashed_tracks.len(),
    );

    let selected_album_tracks_count = selected_tracks.len();
    report_progress(&format!(
        "Selected {selected_album_tracks_count} tracks from albums"
    ));

    // go thru all playlists
    // TODO: merge this with the album loop, but I do not know how to do it in terms of track priorities and what to pick from where in what order
    // to make it a more representative sample. Large playlists may dominate and skew the results.
    report_progress("Selecting random playlist tracks");
    for playlist_id in all_playlists {
        // get album tracks, shuffle and add top N tracks to the selected list
        let (mut tracks, owner_uri) = match fetch_playlist_tracks(
            auth_header_value,
            token_header_value,
            &playlist_id,
            constants::MAX_TRACKS_PER_PLAYLIST,
            runtime,
        )
        .await
        {
            Some(v) => (v.tracks, v.owner_uri),
            None => {
                // ignore the failure - not critical
                // TODO: add a counter
                continue;
            }
        };

        if tracks.is_empty() {
            log!("Empty playlist {playlist_id}");
            continue;
        } else if tracks.len() <= constants::MIN_TRACKS_PER_ALBUM {
            // the playlist is too small and all tracks should be added
            log!(
                "Sel: {}, stash: {}, adding all {} tracks from playlist {playlist_id}, owner: {owner_uri}",
                selected_tracks.len(),
                stashed_tracks.len(),
                tracks.len(),
            );
            let _: Vec<_> = tracks
                .into_iter()
                .map(|v| selected_tracks.insert(v))
                .collect();
        } else {
            // select N random tracks
            tracks.shuffle(&mut rng);
            let _: Vec<_> = tracks
                .drain(..constants::MIN_TRACKS_PER_ALBUM)
                .map(|v| selected_tracks.insert(v))
                .collect();

            // stash the remaining tracks
            let _: Vec<_> = tracks
                .into_iter()
                .map(|v| stashed_tracks.insert(v))
                .collect();
            log!(
                "Sel: {}, stash: {}, added {} tracks from playlist {playlist_id}, owner: {owner_uri}",
                selected_tracks.len(),
                stashed_tracks.len(),
                constants::MIN_TRACKS_PER_ALBUM,
            );
        }

        // exit if there are enough tracks for the playlist
        if selected_tracks.len() >= number_of_tracks_to_add {
            break;
        }
    }

    log!(
        "Selected tracks: {}, stashed tracks: {}",
        selected_tracks.len(),
        stashed_tracks.len()
    );

    // figure out how many tracks were added from playlists for reporting
    let selected_playlist_tracks_count = selected_tracks.len() - selected_album_tracks_count;
    report_progress(&format!(
        "Selected {selected_playlist_tracks_count} tracks from playlists"
    ));

    // log!("Selected tracks:");
    // log!(
    //     "{}",
    //     selected_tracks
    //         .clone()
    //         .into_iter()
    //         .collect::<Vec<String>>()
    //         .join("\n")
    // );

    // add tracks from the stash if the selected list is not long enough
    if selected_tracks.len() < number_of_tracks_to_add {
        let number_to_add_from_stash = number_of_tracks_to_add - selected_tracks.len();
        log!("Adding {number_to_add_from_stash} from stash");
        for stashed_track in stashed_tracks.into_iter().take(number_to_add_from_stash) {
            selected_tracks.insert(stashed_track);
        }
    }

    log!(
        "Selected tracks after adding from stash: {}",
        selected_tracks.len(),
    );

    // remove duplicates already present in the target playlist
    // TODO: this does not work because we get only a few tracks from a playlist and this list doesn't have them all
    let duplicate_tracks = target_playlist_tracks
        .intersection(&selected_tracks)
        .cloned()
        .collect::<Vec<String>>();
    log!(
        "Removing {} selected tracks already in the target playlist",
        duplicate_tracks.len()
    );
    for duplicate_track in duplicate_tracks {
        selected_tracks.remove(&duplicate_track);
    }

    let selected_tracks = selected_tracks.into_iter().collect::<Vec<String>>();

    // report the progress back to the user before writing to the playlist
    let msg = [
        "Adding ",
        &selected_tracks.len().to_string(),
        "tracks to the target playlist",
    ]
    .concat();
    report_progress(&msg);

    // add selected tracks to the back of the current playlist
    let tracks_added = add_tracks_to_playlist(
        auth_header_value,
        token_header_value,
        target_playlist_id,
        selected_tracks,
        runtime,
    )
    .await;

    Ok(format!("Done: added {tracks_added} tracks"))
}
