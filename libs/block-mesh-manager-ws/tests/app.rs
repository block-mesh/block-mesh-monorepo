//use block_mesh_manager_ws::app::app;
//use block_mesh_manager_ws::state::AppState;
//use dotenv::dotenv;
//use futures::SinkExt;
//use reqwest::Client;
//use reqwest_websocket::{Message, RequestBuilderExt, WebSocket};
//use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
//use testcontainers::runners::AsyncRunner;
//use testcontainers::ContainerAsync;
//use testcontainers_modules::postgres::Postgres;
//use tokio::net::TcpListener;
//
//struct TestApp {
//    address: SocketAddr,
//    postgres_container: ContainerAsync<Postgres>,
//}
//
//async fn test_app() -> TestApp {
//    dotenv().ok();
//    let postgres_container = Postgres::default().start().await.unwrap();
//    std::env::set_var(
//        "DATABASE_URL",
//        format!(
//            "postgres://postgres:postgres@{}:{}/postgres",
//            postgres_container.get_host().await.unwrap(),
//            postgres_container.get_host_port_ipv4(5432).await.unwrap(),
//        ),
//    );
//    let listener = TcpListener::bind(SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 0)))
//        .await
//        .unwrap();
//    let address = listener.local_addr().unwrap();
//    let state = AppState::new().await;
//    tokio::spawn(app(listener, state));
//
//    return TestApp {
//        postgres_container,
//        address,
//    };
//}
//
//impl TestApp {
//    async fn websocket_client(&self) -> WebSocket {
//        let response = Client::default()
//            .get(format!("ws://{}/ws", self.address.to_string()))
//            .upgrade()
//            .send()
//            .await
//            .unwrap();
//        response.into_websocket().await.unwrap()
//    }
//}
//
//#[ignore = "Using testcontainers"]
//#[tokio::test]
//async fn ws_ping() {
//    let app = test_app().await;
//    let mut ws = app.websocket_client().await;
//    ws.send(Message::Ping(vec![10, 10, 10])).await.unwrap();
//}
//
