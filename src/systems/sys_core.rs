use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;

use crate::systems::sys_plugin::PluginManager;

/// Internal handler function that logs the request and defers to the PluginManager.
async fn handle_request(
    req: Request<Body>,
    plugin_manager: Arc<PluginManager>,
) -> Result<Response<Body>, Infallible> {
    // Log method, URI, and headers.
    println!("Received {} request for {}", req.method(), req.uri());
    for (name, value) in req.headers().iter() {
        println!("Header: {}: {:?}", name, value);
    }

    // Ask the plugin manager to handle the request.
    if let Some(response) = plugin_manager.handle_request(req).await {
        return Ok(response);
    }

    // Fallback response.
    Ok(Response::new(Body::from("No Plugin Found")))
}

/// Run the server on the given port using the provided PluginManager.
pub async fn run_server(port: u16, plugin_manager: Arc<PluginManager>) {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    let make_svc = make_service_fn(move |_conn| {
        let manager = plugin_manager.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                let manager = manager.clone();
                async move { handle_request(req, manager).await }
            }))
        }
    });

    let server = Server::bind(&addr).serve(make_svc);
    println!("Server running on http://{}", addr);
    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}
