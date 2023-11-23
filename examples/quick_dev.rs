#![allow(unused)]

use anyhow::Result;
use serde_json::json;

#[tokio::main]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000/")?;

    //hc.do_get("hello2/Billy Bob The Fourth").await?.print().await?;

    //hc.do_get("/src/main.rs").await?.print().await?; //- 404

    let req_login = hc.do_post(
        "api/login",
        json!({
            "username": "demo1",
            "pwd": "welcome"
        })
    );

    req_login.await?.print().await?;


    let req_create_ticket = hc.do_post(
        "api/tickets",
        json!({
            "title": "Ticket 2",
        }),
    );

    req_create_ticket.await?.print().await?;

    hc.do_delete("api/tickets/1").await?.print().await?;

    hc.do_get("api/tickets").await?.print().await?;

    Ok(())
}

fn main() {
    if let Err(e) = quick_dev() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}