use axum::{Json, Router, routing::post};
use serde::Deserialize;
use serde_json::{Value, json};
use tower_cookies::{Cookies, cookie, Cookie};

use crate::{Error, Result, web};

pub fn routes() -> Router {
    Router::new()
        .route(
            "/api/login",
            post(api_login),
        )
}

async fn api_login(cookies: Cookies, payload: Json<LoginPayLoad>) -> Result<Json<Value>> {
    println!("->> {:12} - api_login", "HANDLER");

    // TODO: IMplement login logic
    if payload.username != "demo1" || payload.pwd != "welcome" {
        return Err(Error::LoginFail);
    }

    // FIXME: Implement real auth-token generation/signature
    cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));


    // Create the success body.
    let body = Json(json!(
        {
            "result": {
                "success": true
            }
        }
    ));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayLoad {
    username: String,
    pwd: String,
}
