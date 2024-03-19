fn spawn_app() {
    let server = the_newsletter::run().expect("Failed to bind address");
    let _ = tokio::spawn(server);
}


#[tokio::test]
async fn health_check_works() {
    // arrange
    spawn_app();

    let client = reqwest::Client::new();

    // Act
    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed top execute request");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
