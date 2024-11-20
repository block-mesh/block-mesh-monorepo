use crate::constants::DeviceType;
use crate::interfaces::ws_api::WsServerMessage;
use chrono::{DateTime, NaiveDate, Utc};
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::cmp::Ordering;
use std::sync::Arc;
use std::time::Duration;
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
    pub day: NaiveDate,
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
    pub invite_code: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterWalletForm {
    pub pubkey: String,
    pub signature: String,
    pub nonce: String,
    pub password: String,
    pub password_confirm: String,
    pub invite_code: String,
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
    pub start_time: Option<DateTime<Utc>>,
    #[typeshare(serialized_as = "Date")]
    pub end_time: Option<DateTime<Utc>>,
    pub duration_seconds: Option<f64>,
    pub status_code: u16,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug)]
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
    pub referrals: Vec<Referral>,
    pub verified_email: bool,
    pub user_ips: Vec<UserIpInfo>,
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
    pub day: NaiveDate,
    pub points: f64,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthStatusResponse {
    pub status_code: u16,
    pub logged_in: bool,
    pub wallet_address: Option<String>,
    pub email: Option<String>,
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
pub struct ConnectWalletResponse {
    pub status: i32,
}

#[typeshare]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Referral {
    pub email: String,
    #[typeshare(serialized_as = "Date")]
    pub created_at: DateTime<Utc>,
    pub verified_email: bool,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug)]
pub struct EditInviteCodeForm {
    pub new_invite_code: String,
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
    pub day: NaiveDate,
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
    pub updated_at: DateTime<Utc>,
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

#[derive(Deserialize, Serialize, Debug)]
pub struct CaptchaResp {
    pub status: u16,
    pub message: String,
}
