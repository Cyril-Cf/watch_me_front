use crate::{Route, ui::state::app::AppState};
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    let mut open = use_signal(|| false);
    let mut state = use_context::<Signal<AppState>>();
    let navigator = use_navigator();

    let is_auth = state().token.is_some();

    // =======================
    // DESKTOP ACTIONS
    // =======================
    let desktop_actions = if is_auth {
        rsx! {
            div {
                class: "hidden md:flex items-center gap-4",

                // Profile icon
                Link {
                    to: Route::Account {},
                    class: "p-2 rounded-full hover:bg-gray-100 transition",

                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        class: "w-6 h-6 text-gray-700",

                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "2",
                            d: "M5.121 17.804A9 9 0 1118.364 4.56
                                M15 11a3 3 0 11-6 0 3 3 0 016 0z"
                        }
                    }
                }

                button {
                    class: "px-4 py-2 rounded-lg text-sm font-medium text-gray-700 hover:bg-gray-100 transition",
                    onclick: move |_| {
                        state.write().token = None;
                        navigator.push(Route::Home {});
                    },
                    "Déconnexion"
                }
            }
        }
    } else {
        rsx! {
            div {
                class: "hidden md:flex items-center gap-3",

                Link {
                    to: Route::Login {},
                    class: "px-4 py-2 rounded-lg text-sm font-medium text-gray-700 hover:bg-gray-100 transition",
                    "Connexion"
                }

                Link {
                    to: Route::Register {},
                    class: "px-4 py-2 rounded-lg text-sm font-medium text-white
                            bg-gradient-to-r from-indigo-500 to-violet-500
                            transition shadow-sm",
                    "Inscription"
                }
            }
        }
    };

    // =======================
    // MOBILE MENU
    // =======================
    let mobile_menu = if open() {
        if is_auth {
            rsx! {
                div {
                    class: "md:hidden bg-white border-t border-gray-200 shadow-sm",
                    div {
                        class: "px-6 py-4 flex flex-col gap-2",

                        Link {
                            to: Route::Home {},
                            class: "px-4 py-2 rounded-lg text-sm font-medium text-gray-700 hover:bg-gray-100 transition",
                            onclick: move |_| open.set(false),
                            "Profil"
                        }

                        button {
                            class: "px-4 py-2 rounded-lg text-sm font-medium text-gray-700 hover:bg-gray-100 transition text-left",
                            onclick: move |_| {
                                state.write().token = None;
                                open.set(false);
                                navigator.push(Route::Home {});
                            },
                            "Déconnexion"
                        }
                    }
                }
            }
        } else {
            rsx! {
                div {
                    class: "md:hidden bg-white border-t border-gray-200 shadow-sm",
                    div {
                        class: "px-6 py-4 flex flex-col gap-2",

                        Link {
                            to: Route::Login {},
                            class: "px-4 py-2 rounded-lg text-sm font-medium text-gray-700 hover:bg-gray-100 transition",
                            onclick: move |_| open.set(false),
                            "Connexion"
                        }

                        Link {
                            to: Route::Register {},
                            class: "px-4 py-2 rounded-lg text-sm font-medium text-white
                                    bg-gradient-to-r from-indigo-500 to-violet-500
                                    transition shadow-sm",
                            onclick: move |_| open.set(false),
                            "Inscription"
                        }
                    }
                }
            }
        }
    } else {
        rsx! {}
    };

    rsx! {
        nav {
            class: "w-full h-16 bg-white/80 backdrop-blur border-b border-gray-200 fixed top-0 z-50",

            div {
                class: "max-w-7xl mx-auto h-full flex items-center justify-between px-6",

                Link {
                    to: Route::Home {},
                    class: "text-xl font-semibold tracking-tight text-gray-900",
                    "MyApp"
                }

                {desktop_actions}

                button {
                    class: "md:hidden p-2 rounded-lg hover:bg-gray-100 transition",
                    onclick: move |_| open.set(!open()),

                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        class: "w-6 h-6",

                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "2",
                            d: "M4 6h16M4 12h16M4 18h16"
                        }
                    }
                }
            }

            {mobile_menu}
        }

        Outlet::<Route> {}
    }
}
