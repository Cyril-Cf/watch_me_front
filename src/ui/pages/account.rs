use crate::domain::user::{Gender, UpdateUser, User};
use crate::services::user::{get_user, update_user};
use crate::ui::state::app::AppState;
use dioxus::prelude::*;

#[component]
pub fn Account() -> Element {
    let state = use_context::<Signal<AppState>>();

    // Champs contrôlés
    let user = use_signal(|| None::<User>);
    let mut name = use_signal(|| String::new());
    let mut username = use_signal(|| String::new());
    let mut first_name = use_signal(|| String::new());
    let mut email = use_signal(|| String::new());
    let mut gender = use_signal(|| "Undefined".to_string());
    let mut birth_ts = use_signal(|| String::new());
    let loading = use_signal(|| false);
    let error = use_signal(|| "".to_string());
    let success = use_signal(|| false);

    // Elements conditionnels hors rsx!
    let loading_view = if *loading.read() {
        Some(rsx! { p { "Chargement..." } })
    } else {
        None
    };

    let error_view = if !error.read().is_empty() {
        Some(rsx! { p { class: "text-red-600", "{error}" } })
    } else {
        None
    };

    let success_view = if *success.read() {
        Some(rsx! { p { class: "text-green-600", "Mise à jour réussie !" } })
    } else {
        None
    };

    // Effect pour GET user au montage
    use_effect(move || {
        let state = state.clone();
        let mut user = user.clone();
        let mut name = name.clone();
        let mut username = username.clone();
        let mut first_name = first_name.clone();
        let mut email = email.clone();
        let mut gender = gender.clone();
        let mut birth_ts = birth_ts.clone();
        let mut loading = loading.clone();
        let mut error = error.clone();

        spawn(async move {
            if let Some(token) = &state().token {
                loading.set(true);
                match get_user(&state().api_base_url, token).await {
                    Some(fetched_user) => {
                        name.set(fetched_user.name().cloned().unwrap_or_default());
                        username.set(fetched_user.username().cloned().unwrap_or_default());
                        first_name.set(fetched_user.first_name().cloned().unwrap_or_default());
                        email.set(fetched_user.email().to_string());
                        gender.set(format!("{:?}", fetched_user.gender()));
                        birth_ts.set(
                            fetched_user
                                .birth_ts()
                                .map(|ts| ts.to_string())
                                .unwrap_or_default(),
                        );
                        user.set(Some(fetched_user));
                        error.set(String::new());
                    }
                    None => error.set("Impossible de récupérer les infos utilisateur".into()),
                }

                loading.set(false);
            }
        });
    });

    let on_submit = move |_| {
        let state = state.clone();
        let mut user = user.clone();
        let name = name.clone();
        let username = username.clone();
        let first_name = first_name.clone();
        let email = email.clone();
        let gender = gender.clone();
        let birth_ts = birth_ts.clone();
        let mut loading = loading.clone();
        let mut error = error.clone();
        let mut success = success.clone();

        spawn(async move {
            if let Some(token) = &state.read().token {
                loading.set(true);

                let body = UpdateUser::new(
                    Some(email()),
                    None, // password_hash
                    Some(name()),
                    Some(username()),
                    None,
                    Some(first_name()),
                    match gender().as_str() {
                        "Male" => Gender::Male,
                        "Female" => Gender::Female,
                        _ => Gender::Undefined,
                    },
                    birth_ts().parse().ok(),
                );
                match update_user(&state().api_base_url, token, &body).await {
                    Some(updated) => {
                        user.set(Some(updated));
                        success.set(true);
                        error.set("".into());
                    }
                    None => {
                        error.set("Échec de la mise à jour".into());
                        success.set(false);
                    }
                }

                loading.set(false);
            }
        });
    };

    rsx! {
        main {
            class: "min-h-screen bg-gray-50 px-6 py-16",

            div {
                class: "max-w-3xl mx-auto space-y-10",

                h1 { class: "text-3xl font-semibold text-gray-900", "Mon compte" },

                // Affichage conditionnel via variables hors rsx!
                {loading_view}
                {error_view}
                {success_view}

                div {
                    class: "bg-white rounded-2xl shadow p-8 space-y-8",

                    section {
                        class: "space-y-4",
                        h2 { class: "text-xl font-medium text-gray-800", "Identité" },

                        div {
                            class: "grid md:grid-cols-2 gap-4",

                            input {
                                placeholder: "Nom",
                                class: "input",
                                value: "{name()}",
                                oninput: move |e| name.set(e.value())
                            }

                            input {
                                placeholder: "Prénom",
                                class: "input",
                                value: "{first_name()}",
                                oninput: move |e| first_name.set(e.value())
                            }

                            input {
                                placeholder: "Nom d’utilisateur",
                                class: "input",
                                value: "{username()}",
                                oninput: move |e| username.set(e.value())
                            }
                        }
                    }

                    section {
                        class: "space-y-4",
                        h2 { class: "text-xl font-medium text-gray-800", "Compte" },

                        div {
                            class: "grid md:grid-cols-2 gap-4",

                            input {
                                r#type: "email",
                                placeholder: "Email",
                                class: "input",
                                value: "{email()}",
                                oninput: move |e| email.set(e.value())
                            }

                            select {
                                class: "input",
                                value: "{gender()}",
                                onchange: move |e| gender.set(e.value()),

                                option { value: "Undefined", "Genre non défini" }
                                option { value: "Male", "Homme" }
                                option { value: "Female", "Femme" }
                            }

                            input {
                                placeholder: "Date de naissance (timestamp)",
                                class: "input",
                                value: "{birth_ts()}",
                                oninput: move |e| birth_ts.set(e.value())
                            }
                        }
                    }

                    div {
                        class: "flex justify-end gap-4",

                        button {
                            class: "px-6 py-3 rounded-lg text-gray-700 border border-gray-300 hover:bg-gray-100 transition",
                            "Annuler"
                        }

                        button {
                            class: "px-6 py-3 rounded-lg font-medium text-white
                                    bg-gradient-to-r from-indigo-500 to-violet-500
                                    hover:from-indigo-600 hover:to-violet-600
                                    transition",
                            onclick: on_submit,
                            "Enregistrer"
                        }
                    }
                }
            }
        }
    }
}
