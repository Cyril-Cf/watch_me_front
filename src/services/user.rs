use crate::domain::user::{UpdateUser, User};
use gloo_net::http::Request;

pub async fn get_user(api_base: &str, token: &str) -> Option<User> {
    let url = format!("{}/api/account/user", api_base);

    let resp = Request::get(&url)
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await
        .ok()?;

    resp.json::<User>().await.ok()
}

pub async fn update_user(api_base: &str, token: &str, body: &UpdateUser) -> Option<User> {
    let url = format!("{}/api/account/user", api_base);

    let resp = Request::put(&url)
        .header("Authorization", &format!("Bearer {}", token))
        .json(body)
        .ok()?
        .send()
        .await
        .ok()?;

    resp.json::<User>().await.ok()
}
