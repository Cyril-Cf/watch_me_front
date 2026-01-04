use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct WeightEntry {
    id: Uuid,
    user_id: Uuid,
    entry_ts: i64,
    weight: f64,
    body_fat: Option<f64>,
    waist: Option<f64>,
    belt: Option<f64>,
}
