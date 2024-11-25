use crate::interfaces::server_api::{
    GetTaskResponse, ReportBandwidthRequest, ReportUptimeRequest, SubmitTaskRequest,
};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WsServerMessage {
    Ping,
    AssignTask(GetTaskResponse),
    RequestBandwidthReport,
    RequestUptimeReport,
    CloseConnection,
}

impl Display for WsServerMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ping => write!(f, "Ping"),
            Self::RequestBandwidthReport => write!(f, "RequestBandwidthReport"),
            Self::RequestUptimeReport => write!(f, "RequestUptimeReport"),
            Self::CloseConnection => write!(f, "CloseConnection"),
            Self::AssignTask(_response) => write!(f, "AssignTask"),
        }
    }
}

impl TryFrom<String> for WsServerMessage {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "Ping" => Ok(Self::Ping),
            "ping" => Ok(Self::Ping),
            "AssignTask" => Ok(Self::AssignTask(
                serde_json::from_str(&value).map_err(|_| ())?,
            )),
            "RequestBandwidthReport" => Ok(Self::RequestBandwidthReport),
            "RequestUptimeReport" => Ok(Self::RequestUptimeReport),
            "CloseConnection" => Ok(Self::CloseConnection),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WsClientMessage {
    Ping,
    CompleteTask(SubmitTaskRequest),
    ReportBandwidth(ReportBandwidthRequest),
    ReportUptime(ReportUptimeRequest),
}

impl Display for WsClientMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ping => write!(f, "Ping"),
            Self::CompleteTask(_request) => write!(f, "CompleteTask"),
            Self::ReportBandwidth(_request) => write!(f, "ReportBandwidth"),
            Self::ReportUptime(_request) => write!(f, "ReportUptime"),
        }
    }
}
