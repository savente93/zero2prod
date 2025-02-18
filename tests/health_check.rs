use std::net::TcpListener;

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn health_test_works() {
    spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get("http://127.0.0.1.:8000/health_check")
        .send()
        .await
        .expect("Failed to exeecute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
