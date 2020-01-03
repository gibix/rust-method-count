#[macro_use]
extern crate log;
extern crate syn;

pub mod associated_method;
pub mod cognitive_complexity;

pub use associated_method::*;
pub use cognitive_complexity::*;

use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct AggregatedMetrics {
    pub cc: HashMap<String, u64>,
    pub amf: HashMap<String, ItemCount>,
}
