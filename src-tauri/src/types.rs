mod types;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QueryParams {
    limit: usize,
}

impl Default for QueryParams {
    fn default() -> Self {
        QueryParams { limit: 100 }
    }
}
