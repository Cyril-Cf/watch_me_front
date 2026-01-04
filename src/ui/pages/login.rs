use crate::{Route, services::auth::login, ui::state::app::AppState};
use dioxus::prelude::*;

#[component]
pub fn Login() -> Element {
    let mut state = use_context::<Signal<AppState>>();
    let navigator = use_navigator();

    let mut email = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());

    let error_view = if !state().error.is_empty() {
        rsx! {
            p {
                class: "text-sm text-red-600 text-center",
                "{state().error[0]}"
            }
        }
    } else {
        rsx! {}
    };

    rsx! {
        div {
            class: "min-h-screen flex items-center justify-center bg-gradient-to-br from-gray-50 to-gray-100 px-4",

            div {
                class: "w-full max-w-md bg-white rounded-2xl shadow-lg p-8 space-y-6",

                h1 {
                    class: "text-2xl font-semibold text-gray-900 text-center",
                    "Connexion"
                }

                div {
                    class: "space-y-4",

                    input {
                        r#type: "email",
                        placeholder: "Email",
                        class: "w-full px-4 py-3 rounded-lg border border-gray-300
                                focus:outline-none focus:ring-2 focus:ring-indigo-500",
                        value: "{email()}",
                        oninput: move |e| email.set(e.value())
                    }

                    input {
                        r#type: "password",
                        placeholder: "Mot de passe",
                        class: "w-full px-4 py-3 rounded-lg border border-gray-300
                                focus:outline-none focus:ring-2 focus:ring-indigo-500",
                        value: "{password()}",
                        oninput: move |e| password.set(e.value())
                    }
                }

                button {
                    class: "w-full py-3 rounded-lg font-medium text-white
                            bg-gradient-to-r from-indigo-500 to-violet-500
                            hover:from-indigo-600 hover:to-violet-600
                            transition",

                    onclick: move |_| {
                        let email = email();
                        let password = password();
                        let api = state().api_base_url.clone();
                        let mut state = state.clone();
                        let navigator = navigator.clone();

                        spawn(async move {
                            state.write().loading = true;

                            if let Some(token) = login(&api, &email, &password).await {
                                state.write().token = Some(token);
                                state.write().error.clear();
                                state.write().loading = false;

                                navigator.push(Route::Home {});
                            } else {
                                state.write().error = vec!["Échec de connexion".into()];
                                state.write().loading = false;
                            }
                        });
                    },

                    "Se connecter"
                }

                {error_view}
            }
        }
    }
}
