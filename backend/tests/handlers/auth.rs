use axum::{
    body::Body,
    http::{Request, StatusCode}
};
use tower::ServiceExt;
use portfolio_manager::utils::test::init_app;

#[tokio::test]
// TC-001 - Authentication - Sign in : nominal scenario 
async fn test_authentication_sign_in_1() {
    let app = init_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/auth/login")
                .header("content-type", "application/json")
                .body(Body::from(r#"
                    {
                        "email": "john.doe@example.com",
                        "password": "password"
                    }
                    "#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
// TC-002 - Authentication - Sign in : wrong password 
async fn test_authentication_sign_in_2() {
    let app = init_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/auth/login")
                .header("content-type", "application/json")
                .body(Body::from(r#"
                    {
                        "email": "john.doe@example.com",
                        "password": "passwordpassword"
                    }
                    "#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}