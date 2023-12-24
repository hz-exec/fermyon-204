use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;

/// A simple Spin HTTP component.
#[http_component]
fn handle_fermyon204(req: Request) -> anyhow::Result<impl IntoResponse> {
    println!("Handling request to {:?}", req.header("spin-full-url"));
    Ok(Response::builder()
        .status(204)
        .header("Content-Length", "0")
        .header("Cross-Origin-Resource-Policy", "cross-origin")
        .body("")
        .build())
}
