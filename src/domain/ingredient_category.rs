use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct IngredientCategory {
    id: Uuid,
    name: String,
}
