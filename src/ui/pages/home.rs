use crate::Route;
use crate::ui::state::app::AppState;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let state = use_context::<Signal<AppState>>();
    let navigator = use_navigator();

    let content = if state().token.is_some() {
        // =======================
        // CONNECTED DASHBOARD
        // =======================
        rsx! {
            div {
                class: "space-y-10",

                h1 {
                    class: "text-3xl font-semibold text-gray-900",
                    "Dashboard santé"
                }

                // Measurements
                section {
                    class: "bg-white rounded-2xl shadow p-6 space-y-2",

                    h2 {
                        class: "text-xl font-medium text-gray-800",
                        "Suivi des mesures"
                    }

                    p {
                        class: "text-gray-500 text-sm",
                        "Poids, mensurations, évolution dans le temps."
                    }

                    div {
                        class: "h-32 rounded-lg border border-dashed border-gray-300 flex items-center justify-center text-gray-400",
                        "Aucune donnée pour le moment"
                    }
                }

                // Diet sessions
                section {
                    class: "bg-white rounded-2xl shadow p-6 space-y-2",

                    h2 {
                        class: "text-xl font-medium text-gray-800",
                        "Sessions de régime"
                    }

                    p {
                        class: "text-gray-500 text-sm",
                        "Historique et progression par période."
                    }

                    div {
                        class: "h-32 rounded-lg border border-dashed border-gray-300 flex items-center justify-center text-gray-400",
                        "Aucune session enregistrée"
                    }
                }

                // Daily nutrition
                section {
                    class: "bg-white rounded-2xl shadow p-6 space-y-2",

                    h2 {
                        class: "text-xl font-medium text-gray-800",
                        "Consommation quotidienne"
                    }

                    p {
                        class: "text-gray-500 text-sm",
                        "Calories et protéines par repas."
                    }

                    div {
                        class: "h-32 rounded-lg border border-dashed border-gray-300 flex items-center justify-center text-gray-400",
                        "Aucune déclaration aujourd’hui"
                    }
                }
            }
        }
    } else {
        // =======================
        // PUBLIC / NOT CONNECTED
        // =======================
        rsx! {
            div {
                class: "max-w-4xl mx-auto space-y-16",

                // Hero
                section {
                    class: "text-center space-y-6",

                    h1 {
                        class: "text-4xl font-semibold tracking-tight text-gray-900",
                        "Reprenez le contrôle de votre santé"
                    }

                    p {
                        class: "text-lg text-gray-600",
                        "Suivez vos mesures, votre poids et votre alimentation jour après jour,
                         avec une vision claire de votre progression."
                    }

                    div {
                        class: "flex justify-center gap-4",

                        Link {
                            to: Route::Register {},
                            class: "px-4 py-2 rounded-lg text-sm font-medium text-white
                                    bg-gradient-to-r from-indigo-500 to-violet-500
                                    transition shadow-sm",
                            "Inscription"
                        }

                        Link {
                            to: Route::Login {},
                            class: "px-4 py-2 rounded-lg text-sm font-medium text-gray-700 hover:bg-gray-100 transition",
                            "Connexion"
                        }
                    }
                }

                // Features
                section {
                    class: "grid md:grid-cols-3 gap-6",

                    div {
                        class: "bg-white rounded-2xl shadow p-6 space-y-2",

                        h3 { class: "font-medium text-gray-900", "Mesures & poids" }
                        p {
                            class: "text-sm text-gray-600",
                            "Visualisez votre évolution corporelle dans le temps."
                        }
                    }

                    div {
                        class: "bg-white rounded-2xl shadow p-6 space-y-2",

                        h3 { class: "font-medium text-gray-900", "Régimes & objectifs" }
                        p {
                            class: "text-sm text-gray-600",
                            "Structurez vos périodes alimentaires et suivez leur impact."
                        }
                    }

                    div {
                        class: "bg-white rounded-2xl shadow p-6 space-y-2",

                        h3 { class: "font-medium text-gray-900", "Nutrition quotidienne" }
                        p {
                            class: "text-sm text-gray-600",
                            "Déclarez chaque repas et suivez calories et protéines."
                        }
                    }
                }
            }
        }
    };

    rsx! {
        main {
            class: "min-h-screen bg-gray-50 px-6 py-16",
            {content}
        }
    }
}
