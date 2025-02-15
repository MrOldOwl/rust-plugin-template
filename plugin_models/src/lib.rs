use plugin_traits::plugin::Plugin;
mod plugin_traits;
mod enums;

pub use plugin_traits::{*};
pub use enums::{*};

#[no_mangle]
pub fn drop_plugin(_args: Box<dyn Plugin>) {
    // Nothing
}
