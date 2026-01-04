use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    let mut open = use_signal(|| false);

    let mobile_menu = if open() {
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
