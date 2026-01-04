use crate::ui::state::app::AppState;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct AuthBody<'a> {
    email: &'a str,
    password: &'a str,
}

#[derive(Deserialize)]
pub struct AuthResp {
    token: String,
}

pub async fn login(api_base: &str, email: &str, password: &str) -> Option<String> {
    let url = format!("{}/auth/login", api_base);
    let body = AuthBody { email, password };

    let resp = Request::post(&url).json(&body).ok()?.send().await.ok()?;

    let auth = resp.json::<AuthResp>().await.ok()?;
    Some(auth.token)
}

pub async fn signup(api_base: &str, email: &str, password: &str) -> Option<String> {
    let url = format!("{}/auth/signup", api_base);
    let body = AuthBody { email, password };

    let resp = Request::post(&url).json(&body).ok()?.send().await.ok()?;

    let auth = resp.json::<AuthResp>().await.ok()?;
    Some(auth.token)
}

pub fn logout(state: &mut AppState) {
    state.token = None;
}
