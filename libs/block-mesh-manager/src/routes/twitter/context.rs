use twitter_v2::authorization::{Oauth2Client, Oauth2Token};
use twitter_v2::oauth2::{CsrfToken, PkceCodeVerifier};
use uuid::Uuid;

pub struct Oauth2Ctx {
    pub client: Oauth2Client,
    pub verifier: Option<PkceCodeVerifier>,
    pub state: Option<CsrfToken>,
    pub token: Option<Oauth2Token>,
    pub user_nonce: Option<String>,
    pub user_id: Option<Uuid>,
}
