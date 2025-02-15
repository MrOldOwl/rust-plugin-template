use std::sync::Arc;
use plugin_models::{plugin::{Plugin, PluginArgs}, plugin_result::CreateResult};

pub struct PluginMath;

#[no_mangle]
pub fn create_plugin() -> CreateResult {
    Ok(Box::new(PluginMath))
}

impl Plugin for PluginMath {
    fn start(&mut self, _: Arc<PluginArgs>) {
        let sum = add(2, 3);
        println!("2 + 3 = {}", sum);
    }
}


fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
