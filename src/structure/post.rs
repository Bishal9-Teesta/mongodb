
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Post {
    pub userId: u64,
    pub id: u64,
    pub title: String,
    pub body: String,
}
