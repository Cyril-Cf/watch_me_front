use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct FoodLog {
    id: Uuid,
    user_id: Uuid,
    ingredient_id: Uuid,
    quantity: f64,
    entry_ts: i64,
    meal: String,
    target_calories: f64,
    target_proteins: f64,
}
