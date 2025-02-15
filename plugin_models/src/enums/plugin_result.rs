use crate::plugin::Plugin;

pub type CreateResult = Result<Box<dyn Plugin>, String>;
