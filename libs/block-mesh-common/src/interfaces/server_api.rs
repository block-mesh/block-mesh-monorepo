use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct GetTaskResponse {
    pub id: Uuid,
    pub url: String,
    pub method: String,
    pub headers: Option<Value>,
    pub body: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetTaskRequest {
    pub email: String,
    pub api_token: Uuid,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubmitTaskRequest {
    pub email: String,
    pub api_token: Uuid,
    pub task_id: Uuid,
    pub response_code: Option<i32>,
    pub country: Option<String>,
    pub ip: Option<String>,
    pub asn: Option<String>,
    pub colo: Option<String>,
    pub response_time: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfirmEmailRequest {
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReportUptimeRequest {
    pub email: String,
    pub api_token: Uuid,
    pub ip: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReportUptimeResponse {
    pub status_code: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubmitTaskResponse {
    pub status_code: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetTokenRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetEmailViaTokenRequest {
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetEmailViaTokenResponse {
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CheckTokenRequest {
    pub email: String,
    pub api_token: Uuid,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetTokenResponse {
    pub api_token: Option<Uuid>,
    pub message: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetStatsRequest {
    pub email: String,
    pub api_token: Uuid,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Stat {
    pub day: NaiveDate,
    pub tasks_count: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetStatsResponse {
    pub stats: Vec<Stat>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResetPasswordForm {
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResendConfirmEmailForm {
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewPasswordQuery {
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewPasswordForm {
    pub email: String,
    pub token: String,
    pub password: String,
    pub password_confirm: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterForm {
    pub email: String,
    pub password: String,
    pub password_confirm: String,
    pub invite_code: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterResponse {
    pub status_code: u16,
    pub error: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetUserUptimeRequest {
    pub email: String,
    pub api_token: Uuid,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetLatestInviteCodeRequest {
    pub email: String,
    pub api_token: Uuid,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetUserUptimeResponse {
    pub user_id: Uuid,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub duration_seconds: Option<f64>,
    pub status_code: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetLatestInviteCodeResponse {
    pub invite_code: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReportBandwidthRequest {
    pub email: String,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct ReportBandwidthResponse {
    pub status_code: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DashboardResponse {
    pub points: f64,
    pub number_of_users_invited: i64,
    pub invite_code: String,
}
