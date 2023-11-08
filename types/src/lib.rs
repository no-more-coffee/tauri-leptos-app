use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QueryParams<'a> {
    pub limit: usize,
    pub title: Option<&'a str>,
    pub artist: Option<&'a str>,
    pub bpm: Option<i64>,
    pub location: Option<&'a str>,
}

impl<'a> Default for QueryParams<'a> {
    fn default() -> Self {
        QueryParams {
            limit: 100,
            title: None,
            artist: None,
            bpm: None,
            location: None,
        }
    }
}
