use crate::interfaces::server_api::{
    GetTaskResponse, GetTwitterData, ReportBandwidthRequest, ReportUptimeRequest, SendTwitterData,
    SubmitTaskRequest,
};
use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(clippy::large_enum_variant)]
pub enum WsServerMessage {
    Ping,
    AssignTask(GetTaskResponse),
    RequestBandwidthReport,
    RequestUptimeReport,
    CloseConnection,
    RequestTwitterCreds,
    GetTwitterData(GetTwitterData),
}

impl Display for WsServerMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ping => write!(f, "Ping"),
            Self::RequestBandwidthReport => write!(f, "RequestBandwidthReport"),
            Self::RequestUptimeReport => write!(f, "RequestUptimeReport"),
            Self::CloseConnection => write!(f, "CloseConnection"),
            Self::AssignTask(_response) => write!(f, "AssignTask"),
            Self::RequestTwitterCreds => write!(f, "RequestTwitterCreds"),
            Self::GetTwitterData(_t) => write!(f, "GetTwitterData"),
        }
    }
}

impl TryFrom<String> for WsServerMessage {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "Ping" => Ok(Self::Ping),
            "ping" => Ok(Self::Ping),
            "RequestBandwidthReport" => Ok(Self::RequestBandwidthReport),
            "RequestUptimeReport" => Ok(Self::RequestUptimeReport),
            "CloseConnection" => Ok(Self::CloseConnection),
            "RequestTwitterCreds" => Ok(Self::RequestTwitterCreds),
            other => {
                let json: Value = serde_json::from_str(other)?;
                Ok(Self::try_from(json)?)
            }
        }
    }
}

impl TryFrom<Value> for WsServerMessage {
    type Error = anyhow::Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        if let Some(json) = value.get("AssignTask") {
            let v: GetTaskResponse = serde_json::from_value(json.clone())?;
            return Ok(WsServerMessage::AssignTask(v));
        }
        if let Some(json) = value.get("GetTwitterData") {
            let v: GetTwitterData = serde_json::from_value(json.clone())?;
            return Ok(WsServerMessage::GetTwitterData(v));
        }
        Err(anyhow!("unsupported type {:#?}", value))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WsClientMessage {
    Ping,
    CompleteTask(SubmitTaskRequest),
    ReportBandwidth(ReportBandwidthRequest),
    ReportUptime(ReportUptimeRequest),
    ReportTwitterCreds,
    SendTwitterData(SendTwitterData),
}

impl Display for WsClientMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ping => write!(f, "Ping"),
            Self::CompleteTask(_request) => write!(f, "CompleteTask"),
            Self::ReportBandwidth(_request) => write!(f, "ReportBandwidth"),
            Self::ReportUptime(_request) => write!(f, "ReportUptime"),
            Self::ReportTwitterCreds => write!(f, "RequestTwitterCreds"),
            Self::SendTwitterData(_t) => write!(f, "SendTwitterData"),
        }
    }
}
