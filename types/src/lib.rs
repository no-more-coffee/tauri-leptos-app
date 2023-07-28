use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QueryParams<'a> {
    pub limit: usize,
    pub title: Option<&'a str>,
}

impl<'a> Default for QueryParams<'a> {
    fn default() -> Self {
        QueryParams {
            limit: 100,
            title: None,
        }
    }
}
