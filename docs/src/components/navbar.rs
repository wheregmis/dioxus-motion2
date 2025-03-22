use dioxus::prelude::*;
use dioxus_motion2::prelude::*;
use easer::functions::Easing;
use std::time::Duration;

use crate::utils::router::Route;

#[component]
/// Renders a responsive navigation bar with animated transitions.
///
/// On mount, the component animates into view by sliding down from an off-screen position and fading in.
/// It displays a logo, navigation links, and buttons for GitHub and Crates.io. On smaller viewports,
/// a mobile menu can be toggled to show similar navigation links with an overlay effect.
///
/// # Examples
///
/// ```rust
/// use dioxus::prelude::*;
///
/// fn main() {
///     dioxus::web::launch(app);
/// }
///
/// fn app(cx: Scope) -> Element {
///     cx.render(rsx! {
///         NavBar {}
///     })
/// }
/// ```
pub fn NavBar() -> Element {
    let mut nav_bg = use_motion(Transform::new(0.0, -100.0, 1.0, 1.0, 0.0, 0.0, 0.0));
    let mut nav_opacity = use_motion(0.0f32);
    let mut is_menu_open = use_signal(|| false);

    use_effect(move || {
        nav_bg
            .spring()
            .stiffness(100.0)
            .damping(20.0)
            .animate_to(Transform::new(0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0));

        nav_opacity
            .tween()
            .duration(Duration::from_millis(300))
            .animate_to(1.0);
    });

    rsx! {
        div { class: "w-full h-full bg-gradient-dark text-text-secondary",
            header {
                class: "fixed top-0 w-full z-50 h-16 backdrop-blur-md border-b border-primary/10 rust-accent",
                style: "
                    transform: translateY({nav_bg.get().y}px);
                    opacity: {nav_opacity.get()};
                ",
                // Background elements
                div { class: "absolute inset-0 overflow-hidden",
                    div { class: "absolute -top-1/2 -left-1/2 w-full h-full bg-primary/5 rounded-full blur-3xl" }
                    div { class: "absolute -bottom-1/2 -right-1/2 w-full h-full bg-secondary/5 rounded-full blur-3xl" }
                }
                // Content
                div { class: "relative z-10 container mx-auto h-full px-4",
                    div { class: "flex items-center justify-between h-full",
                        // Left side - Logo and navigation
                        div { class: "flex items-center space-x-3",
                            div { class: "flex items-center gap-8 px-6 py-2 bg-dark-200/50 backdrop-blur-sm
                                       border border-primary/10 rounded-full shadow-lg shadow-primary/5
                                       rust-accent",
                                // Rocket logo
                                span { class: "text-2xl performance-pulse", "🚀" }

                                // Logo/Home link
                                Link {
                                    to: Route::Home {},
                                    class: "text-lg font-semibold text-text-primary hover:text-primary transition-colors",
                                    "Dioxus Motion"
                                }

                                // Navigation links - Desktop
                                nav { class: "hidden md:flex items-center space-x-6",
                                    NavLink { to: Route::DocsLanding {}, "Documentation" }
                                }
                            }

                            // Mobile menu button
                            button {
                                class: "md:hidden p-2 rounded-lg transition-colors duration-300
                                       hover:bg-primary/10",
                                onclick: move |_| is_menu_open.toggle(),
                                if *is_menu_open.read() {
                                    span { class: "text-xl", "✕" }
                                } else {
                                    span { class: "text-xl", "☰" }
                                }
                            }
                        }

                        // Right side - Theme toggle and GitHub
                        div { class: "flex items-center space-x-4",
                            // GitHub link
                            a {
                                class: "hidden sm:flex items-center px-4 py-2 rounded-lg
                                       bg-dark-200/50 backdrop-blur-sm hover:bg-dark-200/70
                                       text-text-secondary hover:text-text-primary
                                       border border-primary/10 transition-all duration-300
                                       rust-accent",
                                href: "https://github.com/wheregmis/dioxus-motion",
                                target: "_blank",
                                rel: "noopener",
                                "GitHub"
                                span { class: "ml-2 px-2 py-1 text-xs rounded-full
                                           bg-primary/10 text-primary",
                                    "★ Star"
                                }
                            }

                            // Crates.io badge
                            a {
                                class: "hidden sm:flex items-center px-4 py-2 rounded-lg
                                       bg-dark-200/50 backdrop-blur-sm hover:bg-dark-200/70
                                       text-text-secondary hover:text-text-primary
                                       border border-primary/10 transition-all duration-300
                                       rust-accent",
                                href: "https://crates.io/crates/dioxus-motion",
                                target: "_blank",
                                rel: "noopener",
                                "Crates.io"
                                span { class: "ml-2 px-2 py-1 text-xs rounded-full
                                           bg-primary/10 text-primary",
                                    "0.3.1"
                                }
                            }
                        }
                    }
                }
            }

            // Mobile menu - only render when open
            {
                (*is_menu_open.read())
                    .then(|| {
                        rsx! {
                            // Mobile menu overlay
                            div { class: "fixed inset-0 z-40 bg-dark-200/95 backdrop-blur-lg transition-all duration-300 rust-accent",
                                // Background elements for mobile menu
                                div { class: "absolute inset-0 overflow-hidden",
                                    div { class: "absolute -top-1/2 -left-1/2 w-full h-full bg-primary/5 rounded-full blur-3xl" }
                                    div { class: "absolute -bottom-1/2 -right-1/2 w-full h-full bg-secondary/5 rounded-full blur-3xl" }
                                }
                                // Mobile menu content
                                div { class: "relative z-10 container mx-auto px-4 pt-24",
                                    div { class: "flex flex-col space-y-6",
                                        NavLink { to: Route::DocsLanding {}, "Documentation" }
                                        a {
                                            class: "flex items-center px-6 py-3 rounded-xl
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               bg-dark-200/50 backdrop-blur-sm hover:bg-dark-200/70
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               text-text-secondary hover:text-text-primary
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               border border-primary/10 transition-all duration-300
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               rust-accent",
                                            href: "https://github.com/wheregmis/dioxus-motion",
                                            target: "_blank",
                                            rel: "noopener",
                                            "GitHub"
                                            span { class: "ml-2 px-2 py-1 text-xs rounded-full
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               bg-primary/10 text-primary",
                                                "★ Star"
                                            }
                                        }
                                        a {
                                            class: "flex items-center px-6 py-3 rounded-xl
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               bg-dark-200/50 backdrop-blur-sm hover:bg-dark-200/70
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               text-text-secondary hover:text-text-primary
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               border border-primary/10 transition-all duration-300
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               rust-accent",
                                            href: "https://crates.io/crates/dioxus-motion",
                                            target: "_blank",
                                            rel: "noopener",
                                            "Crates.io"
                                            span { class: "ml-2 px-2 py-1 text-xs rounded-full
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               bg-primary/10 text-primary",
                                                "0.3.1"
                                            }
                                        }
                                    }
                                }
                            }
                            // Close menu when clicking outside
                            div {
                                class: "fixed inset-0 z-30 bg-black/20 backdrop-blur-sm",
                                onclick: move |_| is_menu_open.set(false),
                            }
                        }
                    })
            }

            AnimatedOutlet::<Route> {}
        }
    }
}

#[component]
/// Renders a navigation link with an animated underline that highlights when active.
///
/// This function generates a clickable link styled based on whether it matches the current route.
/// When the link's destination (`to`) equals the current route, an active style is applied to emphasize it.
/// The link also features an animated underline that expands on hover.
///
/// # Arguments
///
/// * `to` - The destination route for the link.
/// * `children` - The content displayed within the link.
///
/// # Examples
///
/// ```
/// use dioxus::prelude::*;
///
/// // Example route enum for demonstration.
/// #[derive(PartialEq, Eq)]
/// enum Route {
///     Home,
///     About,
/// }
///
/// // An example component utilizing NavLink.
/// fn App(cx: Scope) -> Element {
///     cx.render(rsx! {
///         NavLink(Route::Home, rsx! { "Home" })
///     })
/// }
/// ```
fn NavLink(to: Route, children: Element) -> Element {
    let current_route = use_route::<Route>();
    let is_active = current_route == to;

    rsx! {
        Link {
            to,
            class: {
                let base_classes = "relative group text-sm text-text-secondary hover:text-text-primary transition-all duration-300";
                if is_active {
                    format!("{} text-primary", base_classes)
                } else {
                    base_classes.to_string()
                }
            },
            span { class: "relative z-10", {children} }
            // Animated underline
            div { class: "absolute -bottom-1 left-0 h-[2px] w-0 bg-primary
                       transition-all duration-300 group-hover:w-full" }
        }
    }
}
