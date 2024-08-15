use crate::constants::DeviceType;
use crate::interfaces::server_api::{
    GetTaskResponse, ReportBandwidthRequest, ReportUptimeRequest, SubmitTaskRequest,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WsMessage {
    pub message_id: Uuid,
    pub email: Option<String>,
    pub device: Option<DeviceType>,
    pub message: WsMessageTypes,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum WsMessageTypes {
    SendTaskToNode(GetTaskResponse),
    SubmitTaskFromNode(SubmitTaskRequest),
    SendBandwidthReportToNode,
    SubmitForBandwidthReportFromNode(ReportBandwidthRequest),
    SendUptimeToNode,
    SubmitUptimeFromNode(ReportUptimeRequest),
}
