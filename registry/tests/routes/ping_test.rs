use crate::support::TestApp;

#[actix_rt::test]
async fn get_ping() {
    let app = TestApp::new().await;
    let response = app.get("/ping").await.expect("Failed to execute request.");

    assert!(response.status().is_success());
}
