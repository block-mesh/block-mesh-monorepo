use crate::interfaces::server_api::{GetTaskResponse, SubmitTaskRequest};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WsServerMessage {
    AssignTask(GetTaskResponse),
    RequestBandwidthReport,
    RequestUptimeReport,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WsClientMessage {
    CompleteTask(SubmitTaskRequest),
    ReportBandwidth,
    ReportUptime,
}
