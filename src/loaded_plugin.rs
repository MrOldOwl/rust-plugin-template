use std::{ffi::OsStr, sync::Arc};

use libloading::{library_filename, Library};
use plugin_models::{plugin::{Plugin, PluginArgs}, plugin_result::CreateResult};


pub struct LoadedPlugin {
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