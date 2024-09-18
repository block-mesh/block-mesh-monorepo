use crate::interfaces::server_api::{
    GetTaskResponse, ReportBandwidthRequest, ReportUptimeRequest, SubmitTaskRequest,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WsServerMessage {
    Ping,
    AssignTask(GetTaskResponse),
    RequestBandwidthReport,
    RequestUptimeReport,
    CloseConnection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WsClientMessage {
    CompleteTask(SubmitTaskRequest),
    ReportBandwidth(ReportBandwidthRequest),
    ReportUptime(ReportUptimeRequest),
}
