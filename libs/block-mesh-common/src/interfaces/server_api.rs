use crate::constants::DeviceType;
use crate::interfaces::ws_api::WsServerMessage;
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::sync::Arc;
use std::time::Duration;
use time::{Date, OffsetDateTime};
use typeshare::typeshare;
use uuid::Uuid;

#[typeshare]
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GetTaskResponse {
    #[typeshare(serialized_as = "string")]
    pub id: Uuid,
    pub url: String,
    pub method: String,
    #[typeshare(serialized_as = "object")]
    pub headers: Option<Value>,
    #[typeshare(serialized_as = "object")]
    pub body: Option<Value>,
}

#[typeshare]
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GetTwitterData {
    #[typeshare(serialized_as = "string")]
    pub id: Uuid,
    pub twitter_username: String,
    pub since: Date,
    pub until: Date,
}

#[typeshare]
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SendTwitterData {
    #[typeshare(serialized_as = "string")]
    pub id: Uuid,
    pub results: Value,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug)]
pub struct GetTaskRequest {
    pub email: String,
    #[typeshare(serialized_as = "string")]
    pub api_token: Uuid,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubmitTaskRequest {
    pub email: String,
    #[typeshare(serialized_as = "string")]
    pub api_token: Uuid,
    #[typeshare(serialized_as = "string")]
    pub task_id: Uuid,
    pub response_code: Option<i32>,
    pub country: Option<String>,
    pub ip: Option<String>,
    pub asn: Option<String>,
    pub colo: Option<String>,
    pub response_time: Option<f64>,
    pub response_body: Option<String>,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug)]
pub struct ConfirmEmailRequest {
    pub token: String,
    pub email: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientsMetadata {
    pub depin_aggregator: Option<String>,
    #[typeshare(serialized_as = "string")]
    pub device_type: DeviceType,
    pub version: Option<String>,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReportUptimeRequest {
    pub email: String,
    #[typeshare(serialized_as = "string")]
    pub api_token: Uuid,
    pub ip: Option<String>,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug)]
pub struct DashboardRequest {
    pub email: String,
    #[typeshare(serialized_as = "string")]
    pub api_token: Uuid,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug)]
pub struct ReportUptimeResponse {
    pub status_code: u16,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug)]
pub struct SubmitTaskResponse {
    pub status_code: u16,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug)]
pub struct GetTokenRequest {
    pub email: String,
    pub password: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug)]
pub struct GetEmailViaTokenRequest {
    pub token: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug)]
pub struct GetEmailViaTokenResponse {
    pub email: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug)]
pub struct CheckTokenRequest {
    pub email: String,
    #[typeshare(serialized_as = "string")]
    pub api_token: Uuid,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetTokenResponse {
    #[typeshare(serialized_as = "string")]
    pub api_token: Option<Uuid>,
    pub message: Option<String>,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug)]
pub struct GetStatsRequest {
    pub email: String,
    #[typeshare(serialized_as = "string")]
    pub api_token: Uuid,
}
#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Stat {
    #[typeshare(serialized_as = "Date")]
    pub day: Date,
    #[typeshare(serialized_as = "number")]
    pub tasks_count: i64,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug)]
pub struct GetStatsResponse {
    pub stats: Vec<Stat>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SigArray(pub Vec<u8>);

#[typeshare]
#[derive(Serialize, Deserialize, Debug)]
pub struct LoginWalletForm {
    pub pubkey: String,
    pub signature: String,
    pub nonce: String,
    pub password: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug)]
pub struct ResetPasswordForm {
    pub email: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug)]
pub struct ResendConfirmEmailForm {
    pub email: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug)]
pub struct NewPasswordQuery {
    pub token: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug)]
pub struct NewPasswordForm {
    pub email: String,
    pub token: String,
    pub password: String,
    pub password_confirm: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterForm {
    pub email: String,
    pub password: String,
    pub password_confirm: String,
    pub invite_code: Option<String>,
    pub cftoken: Option<String>,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterWalletForm {
    pub pubkey: String,
    pub signature: String,
    pub nonce: String,
    pub password: String,
    pub password_confirm: String,
    pub invite_code: Option<String>,
    pub cftoken: Option<String>,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug)]
pub struct ResetPasswordWalletForm {
    pub pubkey: String,
    pub signature: String,
    pub nonce: String,
    pub password: String,
    pub password_confirm: String,
    pub cftoken: Option<String>,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterResponse {
    pub status_code: u16,
    pub error: Option<String>,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug)]
pub struct GetUserUptimeRequest {
    pub email: String,
    #[typeshare(serialized_as = "string")]
    pub api_token: Uuid,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug)]
pub struct GetLatestInviteCodeRequest {
    pub email: String,
    #[typeshare(serialized_as = "string")]
    pub api_token: Uuid,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug)]
pub struct GetUserUptimeResponse {
    #[typeshare(serialized_as = "string")]
    pub user_id: Uuid,
    #[typeshare(serialized_as = "Date")]
    pub start_time: Option<OffsetDateTime>,
    #[typeshare(serialized_as = "Date")]
    pub end_time: Option<OffsetDateTime>,
    pub duration_seconds: Option<f64>,
    pub status_code: u16,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetLatestInviteCodeResponse {
    pub invite_code: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReportBandwidthRequest {
    pub email: String,
    #[typeshare(serialized_as = "string")]
    pub api_token: Uuid,
    pub download_speed: f64,
    pub upload_speed: f64,
    pub latency: f64,
    pub city: String,
    pub country: String,
    pub ip: String,
    pub asn: String,
    pub colo: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug)]
pub struct ReportBandwidthResponse {
    pub status_code: u16,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct DashboardResponse {
    pub true_count: i64,
    pub false_count: i64,
    pub upload: f64,
    pub download: f64,
    pub latency: f64,
    pub uptime: f64,
    #[typeshare(serialized_as = "number")]
    pub tasks: i64,
    pub points: f64,
    #[typeshare(serialized_as = "number")]
    pub number_of_users_invited: i64,
    pub invite_code: String,
    pub connected: bool,
    pub daily_stats: Vec<DailyStatForDashboard>,
    pub perks: Vec<PerkUI>,
    pub calls_to_action: Vec<CallToActionUI>,
    pub referral_summary: ReferralSummary,
    pub verified_email: bool,
    // pub user_ips: Vec<UserIpInfo>,
    pub wallet_address: Option<String>,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct PerkUI {
    #[typeshare(serialized_as = "string")]
    pub id: Uuid,
    pub name: String,
    pub multiplier: f64,
    pub one_time_bonus: f64,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct CallToActionUI {
    #[typeshare(serialized_as = "string")]
    pub id: Uuid,
    pub name: String,
    pub status: bool,
}

#[typeshare]
#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DailyStatForDashboard {
    #[typeshare(serialized_as = "number")]
    pub tasks_count: i64,
    pub uptime: f64,
    #[typeshare(serialized_as = "Date")]
    pub day: Date,
    pub points: f64,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthStatusResponse {
    pub status_code: u16,
    pub logged_in: bool,
    pub wallet_address: Option<String>,
    pub email: Option<String>,
    pub enable_proof_of_humanity: bool,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RunTaskResponse {
    pub status: i32,
    pub raw: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug)]
pub struct ConnectWalletRequest {
    pub pubkey: String,
    pub message: String,
    pub signature: Vec<u8>,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug)]
pub struct ConnectWalletApiRequest {
    pub email: String,
    pub api_token: Uuid,
    pub pubkey: String,
    pub message: String,
    pub signature: Vec<u8>,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug)]
pub struct ConnectWalletResponse {
    pub status: i32,
    pub message: Option<String>,
}

#[typeshare]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Referral {
    pub email: String,
    #[typeshare(serialized_as = "Date")]
    pub created_at: OffsetDateTime,
    pub verified_email: bool,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug)]
pub struct EditInviteCodeForm {
    pub new_invite_code: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug)]
pub struct EditEmailForm {
    pub new_email: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug)]
pub struct CallToActionForm {
    pub name: String,
    pub status: bool,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DailyLeaderboard {
    #[typeshare(serialized_as = "Date")]
    pub day: Date,
    pub leaderboard_users: Vec<LeaderBoardUser>,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LeaderBoardUser {
    pub email: String,
    pub points: Option<f64>,
    // pub ips: Option<i64>,
}

impl PartialEq<Self> for LeaderBoardUser {
    fn eq(&self, other: &Self) -> bool {
        self.points == other.points
    }
}

impl PartialOrd<Self> for LeaderBoardUser {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for LeaderBoardUser {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Eq for LeaderBoardUser {}

#[typeshare]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserIpInfo {
    pub ip: String,
    pub country: Option<String>,
    #[typeshare(serialized_as = "Date")]
    pub updated_at: OffsetDateTime,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum HandlerMode {
    Http,
    WebSocket,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum GetTokenResponseEnum {
    GetTokenResponse(GetTokenResponse),
    UserNotFound,
    PasswordMismatch,
    ApiTokenNotFound,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum CheckTokenResponseEnum {
    GetTokenResponse(GetTokenResponse),
    UserNotFound,
    ApiTokenMismatch,
    ApiTokenNotFound,
}

pub type GetTokenResponseMap = Arc<DashMap<(String, String), GetTokenResponseEnum>>;
pub type CheckTokenResponseMap = Arc<DashMap<(String, Uuid), CheckTokenResponseEnum>>;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CronReportSettings {
    pub period: Option<Duration>,
    pub messages: Option<Vec<WsServerMessage>>,
    pub window_size: Option<usize>,
}

impl CronReportSettings {
    pub fn new(
        period: Option<Duration>,
        messages: Option<impl Into<Vec<WsServerMessage>>>,
        window_size: Option<usize>,
    ) -> Self {
        Self {
            period,
            messages: messages.map(|m| m.into()),
            window_size,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CronReportAggregateEntry {
    pub period: Duration,
    pub messages: Vec<WsServerMessage>,
    pub window_size: usize,
    pub used_window_size: Option<usize>,
    pub queue_size: usize,
}

impl Default for CronReportAggregateEntry {
    fn default() -> Self {
        Self::new(
            Duration::from_secs(10),
            vec![
                WsServerMessage::RequestUptimeReport,
                WsServerMessage::RequestBandwidthReport,
            ],
            10,
            0,
        )
    }
}

impl CronReportAggregateEntry {
    #[tracing::instrument(name = "new", skip_all)]
    pub fn new(
        period: Duration,
        messages: Vec<WsServerMessage>,
        window_size: usize,
        queue_size: usize,
    ) -> Self {
        Self {
            period,
            messages,
            window_size,
            used_window_size: None,
            queue_size,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CaptchaResp {
    pub status: u16,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VpsResp {
    pub status: u16,
    pub ip: String,
    pub message: String,
    pub asn: Option<u64>,
    pub is_datacenter: Option<bool>,
    pub is_vps: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OptCreds {
    pub email: Option<String>,
    pub api_token: Option<Uuid>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FeedElement {
    pub origin: String,
    pub user_name: String,
    pub link: String,
    pub id: String,
    pub raw: String,
    pub reply: Option<u32>,
    pub retweet: Option<u32>,
    pub like: Option<u32>,
    pub tweet: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetFeedElementUserNameIdLin {
    pub user_name: String,
    pub link: String,
    pub id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DigestDataRequest {
    pub email: String,
    pub api_token: Uuid,
    pub data: FeedElement,
    pub pubkey: Option<String>,
    pub signature: Option<String>,
    pub msg: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DigestDataResponse {
    pub status_code: u16,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SendEmail {
    pub code: String,
    pub user_id: Uuid,
    pub email_type: String,
    pub email_address: String,
    pub nonce: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TurnStile {
    pub secret: String,
    pub response: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug)]
pub struct ProofOfHumanForm {
    pub cftoken: String,
    pub recaptcha_v2: String,
    pub hcaptcha: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ReCaptchaV2 {
    pub secret: String,
    pub response: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HCaptcha {
    pub secret: String,
    pub response: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthStatusParams {
    pub perks_page: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq)]
pub struct ReferralSummary {
    pub total_invites: i64,
    pub total_verified_email: i64,
    pub total_verified_human: i64,
    pub total_eligible: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TmpReferralSummary {
    pub total_invites: Option<i64>,
    pub total_verified_email: Option<i64>,
    pub total_verified_human: Option<i64>,
    pub total_eligible: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserAffiliate {
    pub user_id: Uuid,
    pub email: String,
    pub invited_by: Uuid,
    pub verified_email: bool,
    pub proof_of_humanity: bool,
    pub level: i32,
    pub uptime: f64,
    pub tasks_count: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TmpUserAffiliate {
    pub user_id: Option<Uuid>,
    pub email: Option<String>,
    pub invited_by: Option<Uuid>,
    pub verified_email: Option<bool>,
    pub proof_of_humanity: Option<bool>,
    pub level: Option<i32>,
    pub uptime: Option<f64>,
    pub tasks_count: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AdminReferral {
    pub email: String,
    pub code: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateTwitterTask {
    pub code: String,
    pub username: String,
    pub since: Date,
    pub until: Date,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateBulkTwitterTask {
    pub code: String,
    pub username: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetTwitterProfileDetails {
    pub code: String,
    pub username: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DebugEndpoint {
    pub code: String,
    pub method: String,
    pub user_id: Uuid,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IntractResp {
    pub message: String,
    pub data: IntractRespData,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IntractError {
    pub message: String,
    pub name: String,
}

#[allow(clippy::large_enum_variant)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum IntractResponses {
    IntractRespData(IntractRespData),
    IntractError(IntractError),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[allow(non_snake_case)]
pub struct IntractRespData {
    pub evmAddress: Option<String>,
    pub userId: String,
    pub twitterId: Option<String>,
    pub twitterUsername: Option<String>,
    pub discordId: Option<String>,
    pub discordUsername: Option<String>,
    pub solAddress: Option<String>,
    pub telegramId: Option<String>,
    pub telegramUsername: Option<String>,
    pub email: Option<String>,
    pub pohMintStatus: bool,
    pub kyc: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[allow(non_snake_case)]
pub struct IntractParams {
    pub identity: String,
    pub identityType: IntractIdentityType,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum IntractIdentityType {
    Twitter,
    Discord,
    Email,
    Telegram,
    IntractUser,
    // EVM::EVM, SOLANA::SOLANA
}

impl Display for IntractIdentityType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Twitter => write!(f, "Twitter"),
            Self::Discord => write!(f, "Discord"),
            Self::Email => write!(f, "Email"),
            Self::Telegram => write!(f, "Telegram"),
            Self::IntractUser => write!(f, "IntractUser"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PerkResponse {
    pub error: bool,
    pub message: Option<String>,
    pub cached: bool,
    pub name: String,
    pub multiplier: f64,
    pub one_time_bonus: f64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LivenessRequest {
    pub email: String,
    pub api_token: Uuid,
    pub signature: String,
    pub msg: String,
    pub timestamp: i64,
    pub pubkey: String,
    pub uuid: Uuid,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LivenessResponse {
    pub timestamp: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IdRequest {
    pub email: String,
    pub api_token: String,
    pub fp: String,
    pub fp2: Option<String>,
    pub fp3: Option<String>,
    pub fp4: Option<String>,
    pub signature: Option<String>,
    pub msg: Option<String>,
    pub timestamp: Option<i64>,
    pub pubkey: Option<String>,
    pub uuid: Option<Uuid>,
}
