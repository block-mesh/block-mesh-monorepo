use crate::interfaces::server_api::{
    GetTaskResponse, ReportBandwidthRequest, ReportUptimeRequest, RunTaskResponse,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WsServerMessage {
    AssignTask(GetTaskResponse),
    RequestBandwidthReport,
    RequestUptimeReport,
    CloseConnection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WsClientMessage {
    CompleteTask(RunTaskResponse),
    ReportBandwidth(ReportBandwidthRequest),
    ReportUptime(ReportUptimeRequest),
}
