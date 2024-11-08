use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Lot {
    pub id: Option<String>,
    pub limit_date: String,
    pub current_price: i32,
    pub status: String,
}