use std::rc::Rc;

use dioxus::prelude::*;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;

#[derive(Clone, Deserialize)]
struct Item {
    id: i32,
    name: String,
}

#[derive(Serialize)]
struct AuthBody<'a> {
    username: &'a str,
    password: &'a str,
}

#[derive(Deserialize)]
struct AuthResp {
    token: String,
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let token = use_signal(|| None::<String>);
    let items = use_signal(Vec::<Item>::new);
    let loading = use_signal(|| false);
    let error = use_signal(|| None::<String>);

    let api_base = "https://watchme-cyril-cf3191-9segqxl7.leapcell.dev";

    // ---- Helper functions ----
    let fetch_all = {
        let token = token.clone();
        let items = items.clone();
        let loading = loading.clone();
        let error = error.clone();
        move || {
            let token = token.clone();
            let mut items = items.clone();
            let mut loading = loading.clone();
            let mut error = error.clone();
            let url = format!("{}/items", api_base);
            spawn_local(async move {
                loading.set(true);
                let mut req = Request::get(&url);
                if let Some(t) = token.read().as_ref() {
                    req = req.header("Authorization", &format!("Bearer {}", t));
                }
                match req.send().await {
                    Ok(resp) => match resp.json::<Vec<Item>>().await {
                        Ok(list) => items.set(list),
                        Err(e) => error.set(Some(format!("json error: {}", e))),
                    },
                    Err(e) => error.set(Some(format!("fetch error: {}", e))),
                }
                loading.set(false);
            });
        }
    };

    let login = {
        let token = token.clone();
        move |username: String, password: String| {
            let mut token = token.clone();
            let url = format!("{}/auth/login", api_base);
            spawn_local(async move {
                let body = AuthBody {
                    username: &username,
                    password: &password,
                };
                if let Ok(resp) = Request::post(&url).json(&body).unwrap().send().await {
                    if let Ok(auth) = resp.json::<AuthResp>().await {
                        token.set(Some(auth.token));
                    }
                }
            });
        }
    };

    let signup = {
        let token = token.clone();
        move |username: String, password: String| {
            let mut token = token.clone();
            let url = format!("{}/auth/signup", api_base);
            spawn_local(async move {
                let body = AuthBody {
                    username: &username,
                    password: &password,
                };
                if let Ok(resp) = Request::post(&url).json(&body).unwrap().send().await {
                    if let Ok(auth) = resp.json::<AuthResp>().await {
                        token.set(Some(auth.token));
                    }
                }
            });
        }
    };

    let create_item = {
        let token = token.clone();
        let fetch_all = fetch_all.clone();
        move |name: String| {
            let token = token.clone();
            let fetch_all = fetch_all.clone();
            let url = format!("{}/items", api_base);
            spawn_local(async move {
                if let Some(t) = token.read().as_ref() {
                    let _ = Request::post(&url)
                        .header("Authorization", &format!("Bearer {}", t))
                        .json(&serde_json::json!({ "name": name }))
                        .unwrap()
                        .send()
                        .await;
                    fetch_all();
                }
            });
        }
    };

    // let update_item = {
    //     let token = token.clone();
    //     let fetch_all = fetch_all.clone();
    //     move |id: i32, name: String| {
    //         let token = token.clone();
    //         let fetch_all = fetch_all.clone();
    //         let url = format!("{}/items/{}", api_base, id);
    //         spawn_local(async move {
    //             if let Some(t) = token.read().as_ref() {
    //                 let _ = Request::put(&url)
    //                     .header("Authorization", &format!("Bearer {}", t))
    //                     .json(&serde_json::json!({ "name": name }))
    //                     .unwrap()
    //                     .send()
    //                     .await;
    //                 fetch_all();
    //             }
    //         });
    //     }
    // };

    // let delete_item = {
    //     let token = token.clone();
    //     let fetch_all = fetch_all.clone();
    //     move |id: i32| {
    //         let token = token.clone();
    //         let fetch_all = fetch_all.clone();
    //         let url = format!("{}/items/{}", api_base, id);
    //         spawn_local(async move {
    //             if let Some(t) = token.read().as_ref() {
    //                 let _ = Request::delete(&url)
    //                     .header("Authorization", &format!("Bearer {}", t))
    //                     .send()
    //                     .await;
    //                 fetch_all();
    //             }
    //         });
    //     }
    // };

    // ---- JSX ----
    // avant le rsx!
    // prépare tout avant le rsx!
    let items_snapshot: Vec<_> = items.read().iter().cloned().collect();
    let fetch_all_rc = fetch_all.clone();

    rsx! {
        div {
            h2 { "Auth" }
            button {
                onclick: move |_| login("demo".to_string(), "demo".to_string()),
                "Login"
            }
            button {
                onclick: move |_| signup("demo".to_string(), "demo".to_string()),
                "Signup"
            }
            if let Some(_) = *token.read() {
                p { "Token set" }
            }
        }

        div {
            h2 { "Items" }
            button {
                onclick: {
                    let fetch_all = fetch_all_rc.clone();
                    move |_| fetch_all()
                },
                "Refresh All"
            }
            if *loading.read() { p { "Loading..." } }
            if let Some(err) = error.read().as_ref() { p { "{err}" } }

            // ul {
            //     for item in items_snapshot.iter() {
            //         // toutes les valeurs nécessaires clonées avant closure
            //         let item_id = item.id;
            //         let fetch_all = fetch_all_rc.clone();
            //         li {
            //             key: "{item.id}",
            //             "{item.name}"
            //             button {
            //                 onclick: move |_| update_item(&token, fetch_all.clone(), api_base, item_id, "updated".to_string()),
            //                 "PUT"
            //             }
            //             button {
            //                 onclick: move |_| delete_item(&token, fetch_all.clone(), api_base, item_id),
            //                 "DELETE"
            //             }
            //         }
            //     }
            // }

            // button {
            //     let fetch_all = fetch_all_rc.clone();
            //     onclick: move |_| create_item(&token, fetch_all, api_base, "created".to_string()),
            //     "POST New"
            // }
        }
    }
}

fn create_item(
    token: &Signal<Option<String>>,
    fetch_all: Rc<dyn Fn()>,
    api_base: &str,
    name: String,
) {
    let token = token.clone();
    let fetch_all = fetch_all.clone();
    let url = format!("{}/items", api_base);

    spawn_local(async move {
        if let Some(t) = token.read().as_ref() {
            // .get() pour Signal<T> en 0.7
            let _ = Request::post(&url)
                .header("Authorization", &format!("Bearer {}", t))
                .json(&serde_json::json!({ "name": name }))
                .unwrap()
                .send()
                .await;
            fetch_all(); // safe, Rc clone
        }
    });
}

fn update_item(
    token: &Signal<Option<String>>,
    fetch_all: Rc<dyn Fn()>, // Rc pour pouvoir le déplacer dans async
    api_base: &str,
    id: i32,
    name: String,
) {
    let token = token.clone();
    let fetch_all = fetch_all.clone();
    let url = format!("{}/items/{}", api_base, id);

    spawn_local(async move {
        if let Some(t) = token.read().as_ref() {
            let _ = Request::put(&url)
                .header("Authorization", &format!("Bearer {}", t))
                .json(&serde_json::json!({ "name": name }))
                .unwrap()
                .send()
                .await;
            fetch_all();
        }
    });
}
