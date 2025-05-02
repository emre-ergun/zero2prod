//! tests/api/health_check.rs

use crate::helpers::spawn_app;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = app.get_health_check().await;

    // Assert
    assert!(response.status().is_success());
    assert!(Some(0) == response.content_length());
}
