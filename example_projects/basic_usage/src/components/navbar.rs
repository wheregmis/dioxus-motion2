use crate::Route;
use dioxus::prelude::*;
use dioxus_motion2::*;

#[component]
pub fn NavBar() -> Element {
    let path: Route = use_route();
    let path_string = use_signal(|| path.to_string());

    // Animation values
    let logo_scale = use_motion(1.0);
    let nav_y = use_motion(-100.0);
    let links_opacity = use_motion(0.0);

    // Animate in on mount
    use_effect(move || {
        nav_y
            .spring()
            .stiffness(100.0)
            .damping(15.0)
            .animate_to(0.0);

        links_opacity
            .tween()
            .duration(Duration::from_millis(500))
            .animate_to(1.0);
    });

    // Logo hover animation
    let animate_logo = move |_| {
        logo_scale
            .spring()
            .stiffness(300.0)
            .damping(15.0)
            .animate_to(1.1);
    };

    let reset_logo = move |_| {
        logo_scale
            .spring()
            .stiffness(300.0)
            .damping(15.0)
            .animate_to(1.0);
    };

    let nav_style = use_memo(move || format!("transform: translateY({}px)", nav_y.get()));

    let logo_style = use_memo(move || format!("transform: scale({})", logo_scale.get()));

    let links_style = use_memo(move || format!("opacity: {}", links_opacity.get()));

    rsx! {
        div { class: "w-full h-full bg-gradient-to-b from-zinc-900/90 to-black/90",
            header {
                class: "fixed top-0 w-full z-50 h-16 backdrop-blur-md border-b border-orange-900/20",
                style: "{nav_style}",
                div { class: "container mx-auto h-full px-4",
                    div { class: "flex items-center justify-between h-full",
                        // Left side - Logo and name
                        div { class: "flex items-center space-x-3",
                            div {
                                class: "flex items-center gap-8 px-6 py-2 bg-zinc-800/50 backdrop-blur-sm border border-orange-700/20 rounded-full shadow-lg shadow-orange-900/20",
                                style: "{logo_style}",
                                onmouseenter: animate_logo,
                                onmouseleave: reset_logo,
                                Link {
                                    to: Route::Home {},
                                    onclick: move |evt: Event<MouseData>| async move {
                                        if path_string() == *"/" {
                                            evt.prevent_default();
                                        }
                                    },
                                    h1 { class: "text-lg font-bold flex items-center gap-2 whitespace-nowrap",
                                        span { class: "bg-gradient-to-r from-orange-500 to-red-500 bg-clip-text text-transparent hover:from-orange-400 hover:to-red-400 transition-all duration-300",
                                            "Dioxus Motion"
                                        }
                                        span { class: "animate-bounce", "🚀" }
                                    }
                                }
                                nav {
                                    class: "hidden md:flex items-center space-x-6",
                                    style: "{links_style}",
                                    for link in [("Guide", Route::AnimationGuide {}), ("Examples", Route::AnimationExamples {})]
                                        .iter()
                                    {
                                        Link {
                                            to: link.1.clone(),
                                            class: "text-sm text-zinc-400 hover:text-orange-400 transition-colors relative group cursor-pointer",
                                            "{link.0}"
                                            div { class: "absolute -bottom-1 left-0 h-[2px] w-0 bg-orange-500 transition-all group-hover:w-full" }
                                        }
                                    }
                                }
                            }
                        }
                        // Right side - External links
                        div {
                            class: "flex items-center gap-4",
                            style: "{links_style}",
                            // GitHub link
                            a {
                                href: "https://github.com/DioxusLabs/dioxus-motion",
                                target: "_blank",
                                rel: "noopener noreferrer",
                                class: "flex items-center gap-2 px-3 py-2 bg-zinc-800/50 backdrop-blur-sm border border-orange-700/20 rounded-full shadow-lg shadow-orange-900/20 hover:bg-zinc-700/50 transition-colors text-zinc-400 hover:text-orange-400",
                                svg {
                                    class: "w-5 h-5",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    fill: "currentColor",
                                    view_box: "0 0 24 24",
                                    path { d: "M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z" }
                                }
                            }
                            // Crates.io link
                            a {
                                href: "https://crates.io/crates/dioxus-motion2",
                                target: "_blank",
                                rel: "noopener noreferrer",
                                class: "flex items-center gap-2 px-3 py-2 bg-zinc-800/50 backdrop-blur-sm border border-orange-700/20 rounded-full shadow-lg shadow-orange-900/20 hover:bg-zinc-700/50 transition-colors text-zinc-400 hover:text-orange-400",
                                svg {
                                    class: "w-5 h-5",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    fill: "currentColor",
                                    view_box: "0 0 512 512",
                                    path { d: "M239.1 6.3l-208 78c-18.7 7-31.1 25-31.1 45v225.1c0 18.2 10.3 34.8 26.5 42.9l208 104c13.5 6.8 29.4 6.8 42.9 0l208-104c16.3-8.1 26.5-24.8 26.5-42.9V129.3c0-20-12.4-37.9-31.1-44.9l-208-78C262 2.2 250 2.2 239.1 6.3zM256 68.4l192 72v1.1l-192 78-192-78v-1.1l192-72zm32 356V275.5l160-65v133.9l-160 80z" }
                                }
                            }
                        }
                    }
                }
            }
            main { class: "pt-16", Outlet::<Route> {} }
        }
    }
}
