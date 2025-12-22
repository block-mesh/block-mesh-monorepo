use askama::Template;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Extension;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;

use block_mesh_common::constants::{
    BLOCK_MESH_APP_SERVER, BLOCK_MESH_CHROME_EXTENSION_LINK, BLOCK_MESH_GITBOOK, BLOCK_MESH_GITHUB,
    BLOCK_MESH_LANDING_PAGE_IMAGE, BLOCK_MESH_SUPPORT_CHAT, BLOCK_MESH_SUPPORT_EMAIL,
    BLOCK_MESH_TWITTER, PCN_LOGO,
};

use crate::database::uptime_report::world_map_markers::world_map_markers;
use crate::errors::error::Error;
use crate::startup::application::AppState;

#[allow(dead_code)]
#[derive(Template)]
#[template(path = "map.html")]
struct MapTemplate {
    pub chrome_extension_link: String,
    pub app_server: String,
    pub github: String,
    pub twitter: String,
    pub gitbook: String,
    pub logo: String,
    pub image: String,
    pub support: String,
    pub chat: String,
    pub mapbox_api: String,
    pub markers: Vec<MapMarker>,
    pub cf_site_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapMarkerProperties {
    name: String,
    count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapMarkerGeometry {
    r#type: String,
    coordinates: Vec<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapMarker {
    r#type: String,
    properties: MapMarkerProperties,
    geometry: MapMarkerGeometry,
}

pub async fn handler(
    State(state): State<Arc<AppState>>,
    Extension(pool): Extension<PgPool>,
) -> Result<impl IntoResponse, Error> {
    let mut transaction = pool.begin().await.map_err(Error::from)?;
    let markers = world_map_markers(&mut transaction)
        .await
        .map_err(Error::from)?
        .into_iter()
        .map(|marker| MapMarker {
            r#type: "Feature".to_string(),
            properties: MapMarkerProperties {
                name: marker.city.unwrap_or_default(),
                count: marker.count.unwrap_or_default(),
            },
            geometry: MapMarkerGeometry {
                r#type: "Point".to_string(),
                coordinates: vec![
                    marker.longitude.unwrap_or_default(),
                    marker.latitude.unwrap_or_default(),
                ],
            },
        })
        .collect();
    transaction.commit().await.map_err(Error::from)?;

    Ok(MapTemplate {
        cf_site_key: state.cf_site_key.to_string(),
        mapbox_api: std::env::var("MAPBOX").unwrap_or_default(),
        chrome_extension_link: BLOCK_MESH_CHROME_EXTENSION_LINK.to_string(),
        app_server: BLOCK_MESH_APP_SERVER.to_string(),
        github: BLOCK_MESH_GITHUB.to_string(),
        twitter: BLOCK_MESH_TWITTER.to_string(),
        gitbook: BLOCK_MESH_GITBOOK.to_string(),
        logo: PCN_LOGO.to_string(),
        image: BLOCK_MESH_LANDING_PAGE_IMAGE.to_string(),
        support: BLOCK_MESH_SUPPORT_EMAIL.to_string(),
        chat: BLOCK_MESH_SUPPORT_CHAT.to_string(),
        markers,
    })
}
