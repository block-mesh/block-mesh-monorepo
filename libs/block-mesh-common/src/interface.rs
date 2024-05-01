use chrono::NaiveDate;
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
pub struct CheckTokenRequest {
    pub email: String,
    pub api_token: Uuid,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetTokenResponse {
    pub api_token: Uuid,
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
pub struct RegisterForm {
    pub email: String,
    pub password: String,
    pub password_confirm: String,
}
