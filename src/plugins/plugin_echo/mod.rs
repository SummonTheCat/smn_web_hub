use std::convert::Infallible;
use async_trait::async_trait;
use hyper::{Body, Request, Response};
use crate::structs::struct_plugin::Plugin;

pub struct EchoPlugin;

impl EchoPlugin {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Plugin for EchoPlugin {
    async fn plugin_init(&mut self) {
        println!("{} initialized", self.plugin_name());
    }

    fn plugin_name(&self) -> &str {
        "EchoPlugin"
    }

    // This plugin handles requests where the path is "/echo"
    fn plugin_can_handle(&self, req: &Request<Body>) -> bool {
        req.uri().path() == "/echo"
    }

    async fn plugin_handle(&self, req: Request<Body>) -> Result<Response<Body>, Infallible> {
        // Echo back the request body.
        let body_bytes = hyper::body::to_bytes(req.into_body()).await.unwrap();
        let body_text = String::from_utf8_lossy(&body_bytes);
        let response_body = format!("Echo: {}", body_text);
        Ok(Response::new(Body::from(response_body)))
    }
}
