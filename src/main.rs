use std::sync::Arc;

use smn_web_hub::{plugins::plugin_echo::EchoPlugin, systems::{sys_core::run_server, sys_plugin::PluginManager}};


#[tokio::main]
async fn main() {
    // Create and initialize PluginManager.
    let mut manager = PluginManager::new();
    manager.apply_plugin(Box::new(EchoPlugin::new()));
    manager.init_plugins().await;
    let manager = Arc::new(manager);

    // Run the server on port 3000.
    run_server(3000, manager).await;
}
