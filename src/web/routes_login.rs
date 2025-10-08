use crate::{web, Error, Result};
use axum::{routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}

pub fn routes() -> Router {
    Router::new()
    .route("/api/login",
    post(api_login))
}

async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("-->> {:<12} - api_login - {payload:?}", "HANDLER");
    //TODO: Implement actual login logic here
    if payload.username != "demo1" || payload.pwd != "welcome" {
        return Err(Error::LoginFail);
    }
    // FIXME: Implement reat auth-token generation/signature
    cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));
    //TODO: Set Cookies or JWT Token
    // Create the success body
    let body = Json(json!({
        "result": {
            "success": true,
        }
    }));

    Ok(body)
}

// #[derive(Debug, Deserialize)]
// struct LoginPayload {
//     username: String,
//     pwd: String,
// }