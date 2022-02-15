use std::net::{SocketAddr, TcpListener};

use throwaway::build_app;

pub async fn start_test_server() -> SocketAddr {
    let app = build_app().await.unwrap();

    let socket_address = SocketAddr::from(([127, 0, 0, 1], 0));
    let listener = TcpListener::bind(socket_address).unwrap();

    let address = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::Server::from_tcp(listener)
            .unwrap()
            .serve(app.into_make_service())
            .await
            .unwrap();
    });

    address
}
