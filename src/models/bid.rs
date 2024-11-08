use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

#[derive(Debug, Serialize, Deserialize)]
pub struct Bid {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub offer_id: ObjectId,
    pub user_id: String,
    pub bid_amount: i32,
    pub volume: Option<i32>,
}