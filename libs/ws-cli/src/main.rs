use block_mesh_common::interfaces::server_api::{GetTokenRequest, GetTokenResponse, RegisterForm};
use block_mesh_common::interfaces::ws_api::WsClientMessage;
use block_mesh_common::routes_enum::RoutesEnum;
use futures::{SinkExt, StreamExt};
use inquire::Select;
use reqwest::Client;
use reqwest_websocket::{Message, RequestBuilderExt};
use serde::Serialize;
use std::fmt::{Display, Formatter};
use uuid::Uuid;

async fn auth(client: &Client, email: &str, addr: &str) -> Uuid {
    let password = "ASdasdasldkjasdSAD#21";
    let _response = client
        .post(format!(
            "http://{addr}{}",
            RoutesEnum::Static_UnAuth_Register
        ))
        .form(&RegisterForm {
            email: email.to_string(),
            password: password.to_string(),
            password_confirm: password.to_string(),
            invite_code: "123".to_string(),
        })
        .send()
        .await
        .unwrap();
    let url = format!("http://{addr}/api{}", RoutesEnum::Api_GetToken);
    println!("{url}");
    let response = client
        .post(url)
        .json(&GetTokenRequest {
            email: email.to_string(),
            password: password.to_string(),
        })
        .send()
        .await
        .unwrap();

    let json: GetTokenResponse = response.json::<GetTokenResponse>().await.unwrap();
    json.api_token.unwrap()
}

#[tokio::main]
async fn main() {
    let client = Client::default();
    let email = "hi@gmail.com";
    let addr = "localhost:8000";
    let api_token = auth(&client, email, addr).await;
    let response = client
        .get(format!(
            "ws://{addr}/ws?email={email}&api_token={api_token}",
        ))
        .upgrade()
        .send()
        .await
        .unwrap();
    let (mut tx, mut _rx) = response.into_websocket().await.unwrap().split();
    let ws_client_opts = vec![WsMessage::ReportBandwidth, WsMessage::ReportUptime];
    loop {
        let sel = Select::new("Send WS message", ws_client_opts.clone());
        let selection: WsClientMessage = sel.clone().prompt().unwrap().into();
        tx.send(Message::Text(serde_json::to_string(&selection).unwrap()))
            .await
            .unwrap();
    }
}

#[derive(Serialize, Clone)]
enum WsMessage {
    ReportBandwidth,
    ReportUptime,
}

impl From<WsMessage> for WsClientMessage {
    fn from(value: WsMessage) -> Self {
        match value {
            WsMessage::ReportBandwidth => WsClientMessage::ReportBandwidth,
            WsMessage::ReportUptime => WsClientMessage::ReportUptime,
        }
    }
}

impl Display for WsMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::ReportBandwidth => "report bandwidth",
                Self::ReportUptime => "report uptime",
            }
        )
    }
}
