use std::{ffi::OsStr, sync::Arc};

use libloading::{Library, library_filename};
use plugin_models::{plugin::{Plugin, PluginArgs}, plugin_result::CreateResult};

struct LoadedPlugin {
    lib: Library,
    plugin: Option<Box<dyn Plugin>>
}

impl LoadedPlugin {
    pub fn load<T>(path: T, args: Arc<PluginArgs>) -> Option<Self>
        where T: AsRef<OsStr> 
    {
        unsafe {
            let path = library_filename(path);
            if let Ok(lib) = Library::new(path) {
                if let Ok(func) = lib.get::<fn() -> CreateResult>(b"create_plugin") {
                    if let Ok(mut plugin) = func() {
                        plugin.start(args);
                        return Some(Self { lib, plugin: Some(plugin) });
                    }
                }
            }
        }
        None
    }
}

impl Drop for LoadedPlugin {
    fn drop(&mut self) {
        unsafe {
            if let Ok(func) = self.lib.get::<fn(Box<dyn Plugin>)>(b"drop_plugin") {
                if let Some(x) = self.plugin.take() {
                    func(x);
                }
            }   
        }
    }
}

fn main() {
    let mut plugins = Vec::<LoadedPlugin>::new();
    let args = Arc::new(PluginArgs);
    if let Some(plugin) = LoadedPlugin::load("plugins/plugin_math", args.clone()) {
        plugins.push(plugin);
    }

    if !plugins.is_empty() {
        plugins.remove(0);
    }
}
