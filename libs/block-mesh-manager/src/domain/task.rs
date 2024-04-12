use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::{Json, JsonValue};
use std::fmt::Display;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Method {
    GET,
    POST,
}

impl From<&str> for Method {
    fn from(s: &str) -> Self {
        match s {
            "GET" => Method::GET,
            "POST" => Method::POST,
            _ => Method::GET,
        }
    }
}

impl Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Method::GET => write!(f, "GET"),
            Method::POST => write!(f, "POST"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Status {
    Pending,
    Assigned,
    Completed,
    Failed,
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Pending => write!(f, "Pending"),
            Status::Assigned => write!(f, "Assigned"),
            Status::Completed => write!(f, "Completed"),
            Status::Failed => write!(f, "Failed"),
        }
    }
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: Uuid,
    pub user_id: Uuid,
    pub url: String,
    pub method: Method,
    pub headers: Option<Json<JsonValue>>,
    pub body: Option<Json<JsonValue>>,
    pub assigned_user_id: Option<Uuid>,
    pub status: Status,
    pub response_code: Option<i32>,
    pub response_raw: Option<String>,
    pub created_at: DateTime<Utc>,
}
