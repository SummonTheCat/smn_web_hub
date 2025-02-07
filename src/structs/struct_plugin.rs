use std::convert::Infallible;
use async_trait::async_trait;
use hyper::{Body, Request, Response};

#[async_trait]
pub trait Plugin: Send + Sync {
    async fn plugin_init(&mut self);
    fn plugin_name(&self) -> &str;
    fn plugin_can_handle(&self, req: &Request<Body>) -> bool;
    async fn plugin_handle(&self, req: Request<Body>) -> Result<Response<Body>, Infallible>;
}
