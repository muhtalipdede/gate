use serde::{Serialize, Deserialize};
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Service {
    pub name: String,
    pub endpoints: Vec<String>,
    pub enable_cache: bool,
    pub cache_endpoints: Vec<String>,
    pub cache_duration: u64,
}