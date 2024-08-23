use crate::database::perks::add_perk_to_user::add_perk_to_user;
use crate::domain::perk::PerkName;
use crate::errors::error::Error;
use crate::notification::notification_redirect::NotificationRedirect;
use crate::routes::twitter::context::Oauth2Ctx;
use axum::extract::Query;
use axum::response::Redirect;
use axum::Extension;
use block_mesh_common::constants::BLOCKMESH_TWITTER_USER_ID;
use block_mesh_common::routes_enum::RoutesEnum;
use http::StatusCode;
use reqwest::ClientBuilder;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::PgPool;
use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use twitter_v2::oauth2::{AuthorizationCode, CsrfToken};
use twitter_v2::TwitterApi;

#[derive(Deserialize)]
pub struct CallbackParams {
    code: AuthorizationCode,
    state: CsrfToken,
}

pub async fn callback(
    Extension(ctx): Extension<Arc<Mutex<Oauth2Ctx>>>,
    Extension(pool): Extension<PgPool>,
    Query(CallbackParams { code, state }): Query<CallbackParams>,
) -> Result<Redirect, Error> {
    let (client, verifier) = {
        let mut ctx = ctx.lock().await;
        // get previous state from ctx (see login)
        let saved_state = ctx
            .state
            .take()
            .ok_or_else(|| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "No previous state found".to_string(),
                )
            })
            .map_err(|_| Error::InternalServer)?;
        // // check state returned to see if it matches, otherwise throw an error
        if state.secret() != saved_state.secret() {
            ctx.state = None;
            ctx.user_id = None;
            ctx.user_nonce = None;
            return Err(Error::InternalServer);
        }
        // // get verifier from ctx
        let verifier = ctx
            .verifier
            .take()
            .ok_or_else(|| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "No PKCE verifier found".to_string(),
                )
            })
            .map_err(|_| Error::InternalServer)?;
        let client = ctx.client.clone();
        (client, verifier)
    };

    // request oauth2 token
    let token = client
        .request_token(code.clone(), verifier)
        .await
        .map_err(|_| Error::InternalServer)?;
    // set context for use with twitter API
    ctx.lock().await.token = Some(token);

    let (mut oauth_token, oauth_client) = {
        let ctx = ctx.lock().await;
        let token = ctx
            .token
            .as_ref()
            .ok_or_else(|| (StatusCode::UNAUTHORIZED, "User not logged in!".to_string()))
            .map_err(|_| Error::InternalServer)?
            .clone();
        let client = ctx.client.clone();
        (token, client)
    };
    // refresh oauth token if expired
    if oauth_client
        .refresh_token_if_expired(&mut oauth_token)
        .await
        .map_err(|_| Error::InternalServer)?
    {
        // save oauth token if refreshed
        ctx.lock().await.token = Some(oauth_token.clone());
    }

    let mut ctx = ctx.lock().await;
    if ctx.user_id.is_none() || ctx.user_nonce.is_none() {
        return Err(Error::InternalServer);
    }
    let user_id = ctx.user_id;
    ctx.state = None;
    ctx.user_id = None;
    ctx.user_nonce = None;
    ctx.token = None;

    let api = TwitterApi::new(oauth_token);
    if let Ok(user) = api.get_users_me().send().await {
        let mut transaction = pool.begin().await.map_err(Error::from)?;
        let data = user.into_data().unwrap();
        let follow_data = get_following(data.id.as_u64()).await?;
        if follow_data.following {
            add_perk_to_user(
                &mut transaction,
                user_id.unwrap(),
                PerkName::Twitter,
                1.0,
                500.0,
                serde_json::to_value(&follow_data).unwrap(),
            )
            .await?;
            transaction.commit().await.map_err(Error::from)?;
            Ok(NotificationRedirect::redirect(
                "Success",
                "Twitter perk added",
                &format!("/ui{}", RoutesEnum::Static_Auth_Dashboard),
            ))
        } else {
            Ok(Error::redirect(
                500,
                "ERROR",
                "You're not following @blockmesh_xyz",
                &format!("/ui{}", RoutesEnum::Static_Auth_Dashboard),
            ))
        }
    } else {
        Ok(Error::redirect(
            500,
            "ERROR",
            "Failed verify Twitter account, please contact support",
            &format!("/ui{}", RoutesEnum::Static_Auth_Dashboard),
        ))
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TwitterApiSource {
    id_str: String,
    screen_name: String,
    following: bool,
    #[serde(flatten)]
    extra_fields: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TwitterApiRelationShip {
    source: TwitterApiSource,
    #[serde(flatten)]
    extra_fields: HashMap<String, Value>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct TwitterApiData {
    relationship: TwitterApiRelationShip,
    #[serde(flatten)]
    extra_fields: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TwitterApiResponse {
    data: TwitterApiData,
    #[serde(flatten)]
    extra_fields: HashMap<String, Value>,
}

async fn get_following(user_id: u64) -> anyhow::Result<TwitterApiSource> {
    #[derive(Serialize, Deserialize, Debug)]
    #[allow(non_snake_case)]
    struct Params {
        target_id: u64,
        source_id: u64,
        apiKey: String,
        resFormat: String,
    }

    let client = ClientBuilder::new()
        .timeout(Duration::from_millis(3_000))
        .build()
        .unwrap();
    let value: TwitterApiResponse = client
        .get(env::var("TWITTER_API_URL").expect("could not find TWITTER_API_URL"))
        .query(&Params {
            apiKey: env::var("TWITTER_API_TOKEN").expect("could not find TWITTER_API_TOKEN"),
            target_id: BLOCKMESH_TWITTER_USER_ID,
            source_id: user_id,
            resFormat: "json".to_string(),
        })
        .header(
            "x-rapidapi-host",
            env::var("TWITTER_API_HOST").expect("could not find TWITTER_API_HOST"),
        )
        .header(
            "x-rapidapi-key",
            env::var("TWITTER_API_TOKEN_TOKEN").expect("could not find TWITTER_API_TOKEN_TOKEN"),
        )
        .send()
        .await?
        .json()
        .await?;
    Ok(value.data.relationship.source)
}
