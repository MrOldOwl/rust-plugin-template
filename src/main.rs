mod loaded_plugin;

use loaded_plugin::LoadedPlugin;

use std::sync::Arc;
use plugin_models::plugin::PluginArgs;

fn main() {
    let mut plugins = Vec::new();
    let args = Arc::new(PluginArgs);
    if let Some(plugin) = LoadedPlugin::load("plugins/plugin_math", args.clone()) {
        plugins.push(plugin);
    }

    if !plugins.is_empty() {
        plugins.remove(0);
    }
}
