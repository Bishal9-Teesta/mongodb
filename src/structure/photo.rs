
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Photo {
    pub albumId: u8,
    pub id: u64,
    pub title: String,
    pub url: String,
    pub thumbnailUrl: String
}
