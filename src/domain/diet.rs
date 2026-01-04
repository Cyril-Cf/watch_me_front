use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct Diet {
    id: Uuid,
    user_id: Uuid,
    start_ts: i64,
    target_weight: f64,
    end_ts: Option<i64>,
}
