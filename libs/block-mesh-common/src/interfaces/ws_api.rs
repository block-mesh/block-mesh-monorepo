use crate::constants::DeviceType;
use crate::interfaces::server_api::{
    GetTaskResponse, ReportBandwidthRequest, ReportUptimeRequest, SubmitTaskRequest,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WsMessage {
    // only for requests from client
    pub message_id: Uuid,
    pub email: Option<String>,
    pub device: Option<DeviceType>,
    pub message: WsMessageTypes,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum WsMessageTypes {
    SendTaskFromServer(GetTaskResponse),
    SubmitTaskToServer(SubmitTaskRequest),
    SendBandwidthReportFromServer,
    SubmitForBandwidthReportToServer(ReportBandwidthRequest),
    SendUptimeFromServer,
    SubmitUptimeToServer(ReportUptimeRequest),
}
//
// #[derive(Debug)]
// pub enum WsClientRequest {
//     ReportBandwith,
//     ReportUptime,
//
// }
//
// #[derive(Debug)]
// pub enum WsServerRequest {
//     ExecuteTask,
//
//
// }
