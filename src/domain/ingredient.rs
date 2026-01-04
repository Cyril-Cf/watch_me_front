use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct Ingredient {
    id: Uuid,
    name: String,
    measure_type: String,
    calories: f64,
    proteins: f64,
    extra_info: Option<String>,
    category_id: Uuid,
}
