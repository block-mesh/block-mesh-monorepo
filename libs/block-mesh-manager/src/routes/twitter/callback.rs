use crate::database::aggregate::update_aggregate::update_aggregate;
use crate::database::perks::add_perk_to_user::add_perk_to_user;
use crate::errors::error::Error;
use crate::notification::notification_redirect::NotificationRedirect;
use crate::routes::twitter::context::{Oauth2Ctx, Oauth2CtxPg};
use crate::routes::twitter::helper::TwitterProfile;
use anyhow::{anyhow, Context};
use askama_axum::IntoResponse;
use axum::extract::Query;
use axum::Extension;
use block_mesh_common::constants::DeviceType;
use block_mesh_common::reqwest::http_client;
use block_mesh_common::routes_enum::RoutesEnum;
use block_mesh_manager_database_domain::domain::get_or_create_aggregate_by_user_and_name::get_or_create_aggregate_by_user_and_name;
use http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::PgPool;
use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;
use twitter_v2::oauth2::{AuthorizationCode, CsrfToken};
use twitter_v2::TwitterApi;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CallbackParams {
    code: AuthorizationCode,
    state: CsrfToken,
}

#[tracing::instrument(name = "twitter_callback", skip(ctx, pool))]
pub async fn callback(
    Extension(ctx): Extension<Arc<Mutex<Oauth2Ctx>>>,
    Extension(pool): Extension<PgPool>,
    Query(CallbackParams { code, state }): Query<CallbackParams>,
) -> Result<impl IntoResponse, Error> {
    let parts: Vec<&str> = state.secret().split("___").collect();
    let state = CsrfToken::new(parts[0].to_string());
    let user_id = Uuid::parse_str(parts[1]).map_err(|e| anyhow!("Cannot parse UUID - {}", e))?;
    let target = parts[2]
        .to_string()
        .parse::<u64>()
        .map_err(|e| anyhow!("Cannot parse target - {}", e))?;
    let twitter_profile = match TwitterProfile::new(target) {
        Ok(t) => t,
        Err(_) => {
            return Ok(Error::redirect(
                500,
                "ERROR",
                "Invalid follow target",
                &format!("/ui{}", RoutesEnum::Static_Auth_Dashboard),
            ));
        }
    };
    let mut transaction = pool.begin().await?;
    let twitter_agg =
        get_or_create_aggregate_by_user_and_name(&mut transaction, twitter_profile.name, &user_id)
            .await?;
    let mut pg =
        serde_json::from_value::<Oauth2CtxPg>(twitter_agg.value).context("Cannot deserialize")?;
    let pg_state = pg.state.clone().ok_or(anyhow!("No state"))?;
    let saved_parts: Vec<&str> = pg_state.secret().split("___").collect();
    let saved_state = CsrfToken::new(saved_parts[0].to_string());

    let (client, verifier) = {
        let ctx = ctx.lock().await;
        if state.secret() != saved_state.secret() {
            update_aggregate(&mut transaction, &twitter_agg.id, &Value::Null).await?;
            return Ok(Error::redirect(
                500,
                "ERROR",
                "State error",
                &format!("/ui{}", RoutesEnum::Static_Auth_Dashboard),
            ));
        }
        // // get verifier from ctx
        let verifier = match pg.verifier.take().ok_or_else(|| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "No PKCE verifier found".to_string(),
            )
        }) {
            Ok(v) => v,
            Err(_) => {
                return Ok(Error::redirect(
                    500,
                    "ERROR",
                    "No PKCE verifier found",
                    &format!("/ui{}", RoutesEnum::Static_Auth_Dashboard),
                ));
            }
        };
        let client = ctx.client.clone();
        (client, verifier)
    };

    // request oauth2 token
    let oauth_token = match client.request_token(code.clone(), verifier).await {
        Ok(o) => o,
        Err(_) => {
            return Ok(Error::redirect(
                500,
                "ERROR",
                "No Oauth token",
                &format!("/ui{}", RoutesEnum::Static_Auth_Dashboard),
            ));
        }
    };
    // set context for use with twitter API
    // ctx.lock().await.token = Some(token);

    let user_id = pg.user_id;
    update_aggregate(&mut transaction, &twitter_agg.id, &Value::Null).await?;

    let api = TwitterApi::new(oauth_token);
    if let Ok(user) = api.get_users_me().send().await {
        let data = user.into_data().unwrap();
        tracing::info!("target = {}", target);
        let twitter_profile = TwitterProfile::new(target)
            .map_err(|_| Error::Auth("Bad follow target".to_string()))?;
        let follow_data = get_following(data.id.as_u64(), target).await?;
        if follow_data.following {
            add_perk_to_user(
                &mut transaction,
                user_id.unwrap(),
                twitter_profile.perk,
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
                &format!("You're not following @{}", twitter_profile.name),
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

async fn get_following(user_id: u64, target: u64) -> anyhow::Result<TwitterApiSource> {
    #[derive(Serialize, Deserialize, Debug)]
    #[allow(non_snake_case)]
    struct Params {
        target_id: u64,
        source_id: u64,
        apiKey: String,
        resFormat: String,
    }
    let client = http_client(DeviceType::AppServer);
    let value: Value = client
        .get(env::var("TWITTER_API_URL").expect("could not find TWITTER_API_URL"))
        .query(&Params {
            apiKey: env::var("TWITTER_API_TOKEN").expect("could not find TWITTER_API_TOKEN"),
            target_id: target,
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
    let value: TwitterApiResponse = serde_json::from_value(value)?;
    // let value: TwitterApiResponse = client
    //     .get(env::var("TWITTER_API_URL").expect("could not find TWITTER_API_URL"))
    //     .query(&Params {
    //         apiKey: env::var("TWITTER_API_TOKEN").expect("could not find TWITTER_API_TOKEN"),
    //         target_id: target,
    //         source_id: user_id,
    //         resFormat: "json".to_string(),
    //     })
    //     .header(
    //         "x-rapidapi-host",
    //         env::var("TWITTER_API_HOST").expect("could not find TWITTER_API_HOST"),
    //     )
    //     .header(
    //         "x-rapidapi-key",
    //         env::var("TWITTER_API_TOKEN_TOKEN").expect("could not find TWITTER_API_TOKEN_TOKEN"),
    //     )
    //     .send()
    //     .await?
    //     .json()
    //     .await?;
    Ok(value.data.relationship.source)
}
