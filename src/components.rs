pub mod kernel;
pub mod memory;
pub mod network;

use anyhow::Result;

pub trait Collector: Send + Sync {
    fn name(&self) -> &'static str;
    fn collect(&self) -> Result<serde_json::Value>;
}
