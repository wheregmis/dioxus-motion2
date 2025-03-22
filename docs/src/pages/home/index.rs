use dioxus::prelude::*;
use dioxus_motion2::prelude::*;
use std::time::Duration;

use crate::components::animated_flower::AnimatedFlower;
use crate::components::cube_animation::SwingingCube;
use crate::{components::transformation_example::TransformAnimationShowcase, utils::router::Route};

#[component]
/// Renders the main landing page of the application.
///
/// This component initializes animated states for opacity, scale, and vertical positioning of key elements.
/// On mount, it triggers staggered spring and tween animations that animate the hero section, titles, and feature overlays,
/// creating a dynamic and engaging home page layout.
///
/// # Examples
///
/// ```
/// use dioxus::prelude::*;
/// // Adjust the import path below according to your project setup.
/// use your_crate::Home;
///
/// fn main() {
///     dioxus::web::launch(Home);
/// }
/// ```
pub fn Home() -> Element {
    // Animation values
    let title_y = use_motion(-50.0);
    let subtitle_y = use_motion(50.0);
    let hero_opacity = use_motion(0.0);
    let demo_scale = use_motion(0.8);
    let demo_opacity = use_motion(0.0);

    // Animate in on mount
    use_effect(move || {
        title_y
            .spring()
            .stiffness(100.0)
            .damping(15.0)
            .animate_to(0.0);

        subtitle_y
            .spring()
            .stiffness(100.0)
            .damping(15.0)
            .animate_to(0.0);

        hero_opacity
            .tween()
            .duration(Duration::from_millis(800))
            .animate_to(1.0);

        demo_scale
            .spring()
            .stiffness(100.0)
            .damping(15.0)
            .animate_to(1.0);

        demo_opacity
            .tween()
            .duration(Duration::from_millis(800))
            .animate_to(1.0);
    });

    rsx! {
        div { class: "min-h-screen bg-dark-50 text-text-primary",
            // Hero section
            section { class: "relative min-h-screen flex items-center justify-center overflow-hidden",
                // Background shapes
                div { class: "absolute inset-0 overflow-hidden",
                    AnimatedFlower {}
                    SwingingCube {}
                }

                // Content
                div { class: "relative z-10 text-center max-w-4xl mx-auto px-4",
                    h1 {
                        class: "text-4xl md:text-5xl lg:text-6xl font-bold mb-4",
                        style: "transform: translateY({title_y.get()}px)",
                        span { class: "text-gradient-primary", "Dioxus Motion2" }
                    }
                    p {
                        class: "text-lg md:text-xl text-text-secondary mb-8",
                        style: "transform: translateY({subtitle_y.get()}px)",
                        "Create beautiful animations with simple, powerful APIs"
                    }

                    // CTA buttons
                    div { class: "flex flex-col sm:flex-row justify-center gap-4",
                        Link {
                            to: Route::DocsLanding {},
                            class: "px-8 py-3 bg-primary/90 backdrop-blur-sm text-dark-50 rounded-xl
                                   font-semibold transition-all duration-300 hover:scale-105 
                                   shadow-lg shadow-primary/20 hover:shadow-primary/30",
                            "Get Started →"
                        }
                        a {
                            href: "https://github.com/wheregmis/dioxus-motion2",
                            target: "_blank",
                            class: "px-8 py-3 bg-dark-200/50 backdrop-blur-sm text-white/90 rounded-xl
                                   font-semibold transition-all duration-300 hover:scale-105 
                                   border border-white/10 hover:border-white/20",
                            "View on GitHub"
                        }
                    }
                }
            }

            // Demo section
            section { class: "py-20 bg-dark-100/50",
                div { class: "container mx-auto px-4",
                    h2 { class: "text-3xl font-bold text-center mb-12", "Basic Usage" }
                    div { class: "grid grid-cols-1 md:grid-cols-2 gap-8",
                        // Code example
                        div { class: "p-6 bg-dark-200/50 backdrop-blur-sm rounded-xl border border-primary/10",
                            h3 { class: "text-xl font-semibold mb-4", "Simple Spring Animation" }
                            pre { class: "bg-dark-300/50 p-4 rounded-lg overflow-x-auto",
                                code { class: "text-sm",
                                    {r#"let scale = use_motion(1.0);

// Animate on hover
let animate = move |_| {
    scale
        .spring()
        .stiffness(300.0)
        .damping(15.0)
        .animate_to(1.2);
};

rsx! {
    div {
        style: "transform: scale({scale.get()})",
        onmouseenter: animate,
        "Hover me!"
    }
}"#}
                                }
                            }
                        }
                        // Live demo
                        div {
                            class: "p-6 bg-dark-200/50 backdrop-blur-sm rounded-xl border border-primary/10",
                            style: "transform: scale({demo_scale.get()})",
                            h3 { class: "text-xl font-semibold mb-4", "Live Demo" }
                            TransformAnimationShowcase {}
                        }
                    }
                }
            }

            // Features section
            section { class: "py-20",
                div { class: "container mx-auto px-4",
                    h2 { class: "text-3xl font-bold text-center mb-12", "Features" }
                    div { class: "grid grid-cols-1 md:grid-cols-3 gap-8",
                        FeatureCard {
                            title: "Spring Physics",
                            description: "Create natural, physics-based animations with configurable springs.",
                            icon: "🌊",
                        }
                        FeatureCard {
                            title: "Transform API",
                            description: "Animate position, scale, and rotation with a simple, unified API.",
                            icon: "🎯",
                        }
                        FeatureCard {
                            title: "Custom Types",
                            description: "Implement Animatable for your own types to create custom animations.",
                            icon: "✨",
                        }
                    }
                }
            }

            // Footer
            footer { class: "py-8 border-t border-dark-200",
                div { class: "container mx-auto px-4",
                    div { class: "flex items-center justify-between",
                        div { class: "text-sm text-text-secondary",
                            "Built with ❤️ using Dioxus Motion2"
                        }
                        div { class: "flex items-center space-x-4 text-sm text-text-secondary",
                            a {
                                href: "https://github.com/wheregmis/dioxus-motion2",
                                target: "_blank",
                                class: "hover:text-text-primary transition-colors",
                                "GitHub"
                            }
                            span { "·" }
                            a {
                                href: "https://crates.io/crates/dioxus-motion2",
                                target: "_blank",
                                class: "hover:text-text-primary transition-colors",
                                "Crates.io"
                            }
                            span { "·" }
                            a {
                                href: "https://docs.rs/dioxus-motion2",
                                target: "_blank",
                                class: "hover:text-text-primary transition-colors",
                                "Documentation"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
/// Renders an animated feature card with a specified icon, title, and description.
///
/// This component displays a card that animates on hover by scaling up and shifting slightly upward,
/// then reverting to its original state when the mouse leaves. The animations are achieved using spring
/// dynamics to ensure smooth transitions.
///
/// # Arguments
///
/// * `title` - The title text displayed on the card.
/// * `description` - A brief description of the feature.
/// * `icon` - A static string representing the feature's icon (e.g., an emoji).
///
/// # Returns
///
/// A Dioxus `Element` representing the rendered feature card.
///
/// # Examples
///
/// ```
/// use dioxus::prelude::*;
///
/// fn app(cx: Scope) -> Element {
///     cx.render(rsx! {
///         FeatureCard {
///             title: "Spring Physics",
///             description: "Natural animations with customizable spring parameters",
///             icon: "🌊",
///         }
///     })
/// }
/// ```
fn FeatureCard(title: &'static str, description: &'static str, icon: &'static str) -> Element {
    let scale = use_motion(1.0);
    let y = use_motion(0.0);

    let animate_hover = move |_| {
        scale
            .spring()
            .stiffness(300.0)
            .damping(15.0)
            .animate_to(1.05);

        y.spring().stiffness(300.0).damping(15.0).animate_to(-5.0);
    };

    let animate_reset = move |_| {
        scale
            .spring()
            .stiffness(300.0)
            .damping(15.0)
            .animate_to(1.0);

        y.spring().stiffness(300.0).damping(15.0).animate_to(0.0);
    };

    let style = use_memo(move || {
        format!(
            "transform: scale({}) translateY({}px);",
            scale.get(),
            y.get()
        )
    });

    rsx! {
        div {
            class: "p-6 bg-dark-200/50 backdrop-blur-sm rounded-xl border border-primary/10
                   transition-all duration-300 hover:border-primary/20 hover:shadow-lg hover:shadow-primary/10",
            style: "{style}",
            onmouseenter: animate_hover,
            onmouseleave: animate_reset,
            div { class: "text-4xl mb-4", {icon} }
            h3 { class: "text-xl font-semibold mb-2", {title} }
            p { class: "text-text-secondary", {description} }
        }
    }
}
