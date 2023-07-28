
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Photo {
    pub albumId: u8,
    pub id: u64,
    pub title: String,
    pub url: String,
    pub thumbnailUrl: String
}
