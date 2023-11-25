
#![allow(unused_imports)] // For beginning only.

use std::net::SocketAddr;
use axum::{Router, middleware, Json};
use axum::extract::{Query, Path};
use axum::routing::{get, get_service};
use axum::response::{Html, IntoResponse, Response};
use serde::Deserialize;
use serde_json::json;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use uuid::Uuid;

use crate::model::ModelController;

pub use self::error::{Error, Result};
mod error;
mod web;
mod model;
mod ctx; // Context, named this way to avoid conflicting.

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize model controller
    let mc = ModelController::new().await?;

    let routes_api = web::routes_tickets::routes(mc.clone())
        .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));
    // We use route layer to only apply to this route, meaning that routes_hello, routes_login, and routes_static are not affected.

    // Important note, it is executed in order from top to bottom.
    let routes_all = Router::new()
    .merge(routes_hello())
    .merge(web::routes_login::routes())
    .nest("/api",  routes_api) // Could just pass, but cloning for now.
    .layer(middleware::map_response(main_response_mapper))
    .layer(middleware::from_fn_with_state(mc.clone(), web::mw_auth::mw_ctx_resolver))
    .layer(CookieManagerLayer::new())
    .fallback_service(routes_static());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();

    Ok(())
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}
fn routes_hello() -> Router {
    Router::new()
        .route(
            "/hello",
            get(handler_hello),
        )
        .route(
            "/hello2/:name",
            get(handler_hello2),
        )
}

// e.g, "/hello?name=John Doe"
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World");
    Html(format!("<h1>Hello, {name}!</h1>"))
}

// e.g, "/hello2/John Doe"
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello2 - {name:?}", "HANDLER");

    Html(format!("<h1>Hello, {name}!</h1>"))
}

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RESP_MAPPER");
    println!();

    let uuid = Uuid::new_v4();

    // Get response error.
    let service_error = res.extensions().get::<Error>();
    let client_status_error = service_error.map(|e| e.client_status_and_error());

    // Build a new response
    let error_response = client_status_error.as_ref().map(|(status, client_error)| {
        let body = json!({
            "error": {
                "type": client_error.as_ref(),
                "req_uuid": uuid.to_string(),
            }
        });

        println!("->> {:<12} - {body:?}", "RESP_MAPPER");

        // Build the new response.
        (*status, Json(body)).into_response()
    });

    // TODO: Build and log the error response.
    println!("->> server log line: {uuid} - Error: {service_error:?}");

    error_response.unwrap_or(res)
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}