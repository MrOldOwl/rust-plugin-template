use std::sync::Arc;

pub struct PluginArgs;

pub trait Plugin {
    fn start(&mut self, args: Arc<PluginArgs>);
}
