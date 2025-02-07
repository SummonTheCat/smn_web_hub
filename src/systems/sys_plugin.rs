use hyper::{Body, Request, Response};
use crate::structs::struct_plugin::Plugin;

pub struct PluginManager {
    plugins: Vec<Box<dyn Plugin>>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }

    /// Add a plugin to the manager.
    pub fn apply_plugin(&mut self, plugin: Box<dyn Plugin>) {
        self.plugins.push(plugin);
    }

    /// Initialize all plugins.
    pub async fn init_plugins(&mut self) {
        for plugin in self.plugins.iter_mut() {
            plugin.plugin_init().await;
        }
    }

    /// Ask each plugin if it can handle the request; return the first response.
    pub async fn handle_request(&self, req: Request<Body>) -> Option<Response<Body>> {
        for plugin in self.plugins.iter() {
            if plugin.plugin_can_handle(&req) {
                // We assume plugin_handle returns Ok(Response) on success.
                return Some(plugin.plugin_handle(req).await.ok()?);
            }
        }
        None
    }
}
