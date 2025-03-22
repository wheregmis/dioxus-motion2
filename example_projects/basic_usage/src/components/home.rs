use dioxus::prelude::*;
use dioxus_motion2::*;
use std::f32::consts::PI;

#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "h-[calc(100vh-4rem)] relative overflow-hidden",
            // Background layer (lowest)
            div { class: "absolute inset-0 z-0", AnimatedBackground {} }

            // Decorative elements layer
            div { class: "absolute inset-0 z-10", DecorativeCircle {} }

            // Orbital system layer
            div { class: "absolute inset-0 z-20", OrbitalSystem {} }

            // Content layer (highest)
            div { class: "relative z-30 h-full flex flex-col",
                // Main content container
                div { class: "flex-1 container mx-auto px-4 flex flex-col",
                    // Hero content
                    div { class: "pt-32 pb-16", HeroContent {} }

                    // Feature cards with adjusted background
                    div { class: "mt-auto pb-16", FeatureCards {} }

                    // Learn more button
                    LearnMore {}
                }

                Footer {}
            }
        }
    }
}

#[component]
fn AnimatedBackground() -> Element {
    let mut shape1_transform = use_motion(Transform::identity());
    let mut shape2_transform = use_motion(Transform::identity());
    let mut shape3_transform = use_motion(Transform::identity());

    use_effect(move || {
        shape1_transform
            .keyframes()
            .at(0.0, Transform::new(0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0))
            .at(
                0.5,
                Transform::new(20.0, -20.0, 1.1, 1.1, PI / 4.0, 0.0, 0.0),
            )
            .at(1.0, Transform::new(0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0))
            .duration(Duration::from_secs(6))
            .start(&mut shape1_transform);

        shape2_transform
            .keyframes()
            .at(0.0, Transform::new(-20.0, 10.0, 1.0, 1.0, 0.0, 0.0, 0.0))
            .at(
                0.5,
                Transform::new(0.0, -20.0, 1.2, 1.2, PI / 6.0, 0.0, 0.0),
            )
            .at(1.0, Transform::new(-20.0, 10.0, 1.0, 1.0, 0.0, 0.0, 0.0))
            .duration(Duration::from_secs(8))
            .start(&mut shape2_transform);

        shape3_transform
            .keyframes()
            .at(0.0, Transform::new(10.0, -10.0, 1.0, 1.0, 0.0, 0.0, 0.0))
            .at(
                0.5,
                Transform::new(-15.0, 15.0, 0.8, 0.8, -PI / 3.0, 0.0, 0.0),
            )
            .at(1.0, Transform::new(10.0, -10.0, 1.0, 1.0, 0.0, 0.0, 0.0))
            .duration(Duration::from_secs(7))
            .start(&mut shape3_transform);
    });

    rsx! {
        div { class: "absolute inset-0 overflow-hidden",
            div {
                class: "absolute w-64 h-64 bg-gradient-to-br from-orange-700/20 to-red-900/20 rounded-full blur-xl",
                style: "top: 10%; left: 5%; transform: {shape1_transform.get().to_css_string()}",
            }
            div {
                class: "absolute w-48 h-48 bg-gradient-to-tr from-zinc-700/20 to-orange-800/20 rounded-lg blur-lg",
                style: "top: 30%; right: 10%; transform: {shape2_transform.get().to_css_string()}",
            }
            div {
                class: "absolute w-56 h-56 bg-gradient-to-bl from-red-900/20 to-orange-700/20 rounded-lg blur-lg",
                style: "bottom: 15%; left: 15%; transform: {shape3_transform.get().to_css_string()}",
            }
        }
    }
}

#[component]
fn HeroContent() -> Element {
    let title_y = use_motion(-50.0);
    let title_opacity = use_motion(0.0);
    let subtitle_opacity = use_motion(0.0);
    let cards_scale = use_motion(0.95);
    let cards_opacity = use_motion(0.0);

    use_effect(move || {
        title_y
            .spring()
            .stiffness(100.0)
            .damping(15.0)
            .animate_to(0.0);

        title_opacity
            .tween()
            .duration(Duration::from_millis(800))
            .animate_to(1.0);

        subtitle_opacity
            .tween()
            .duration(Duration::from_millis(800))
            .animate_to(1.0);

        cards_scale
            .spring()
            .stiffness(150.0)
            .damping(20.0)
            .animate_to(1.0);

        cards_opacity
            .tween()
            .duration(Duration::from_millis(800))
            .animate_to(1.0);
    });

    let title_style = use_memo(move || {
        format!(
            "transform: translateY({}px); opacity: {};",
            title_y.get(),
            title_opacity.get()
        )
    });

    let subtitle_style = use_memo(move || format!("opacity: {}", subtitle_opacity.get()));

    rsx! {
        div { class: "flex-1 relative container mx-auto px-4 pt-32 pb-16 flex flex-col",
            h1 {
                class: "text-5xl font-bold text-center mb-6",
                style: "{title_style}",
                span { class: "bg-gradient-to-r from-orange-500 to-red-500 bg-clip-text text-transparent",
                    "Smooth Animations for Rust"
                }
            }
            p {
                class: "text-xl text-center text-zinc-400 mb-12 max-w-2xl mx-auto",
                style: "{subtitle_style}",
                "High-performance, type-safe animations forged with Rust's powerful systems programming capabilities."
            }
            FeatureCards {}
        }
    }
}

#[component]
fn DecorativeCircle() -> Element {
    let mut circle_scale = use_motion(0.8);
    let mut circle_rotate = use_motion(0.0);

    use_effect(move || {
        circle_scale
            .keyframes()
            .at(0.0, 0.8)
            .at(0.5, 1.2)
            .at(1.0, 0.8)
            .duration(Duration::from_secs(4))
            .start(&mut circle_scale);

        circle_rotate
            .keyframes()
            .at(0.0, 0.0)
            .at(1.0, 2.0 * PI)
            .duration(Duration::from_secs(8))
            .start(&mut circle_rotate);
    });

    let style = use_memo(move || {
        format!(
            "transform: scale({}) rotate({}rad);",
            circle_scale.get(),
            circle_rotate.get()
        )
    });

    rsx! {
        div {
            class: "absolute left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 w-96 h-96 border-2 border-orange-500/20 rounded-full",
            style: "{style}",
        }
    }
}

#[component]
fn OrbitalSystem() -> Element {
    let system_opacity = use_motion(0.0);
    let mut rust_scale = use_motion(1.0);
    let mut rust_rotate = use_motion(0.0);

    // Create orbiting crates
    let crates = [
        ("Cargo", 60.0, 8.0),    // Package manager
        ("Tokio", 100.0, 12.0),  // Async runtime
        ("Dioxus", 140.0, 10.0), // UI framework
        ("Serde", 180.0, 9.0),   // Serialization
        ("Actix", 220.0, 11.0),  // Web framework
        ("Rayon", 260.0, 8.0),
    ];

    let mut crate_positions =
        Signal::new(crates.iter().map(|_| use_motion(0.0)).collect::<Vec<_>>());
    let mut crate_rotations =
        Signal::new(crates.iter().map(|_| use_motion(0.0)).collect::<Vec<_>>());

    use_effect(move || {
        // Fade in the entire system
        system_opacity
            .tween()
            .duration(Duration::from_millis(1500))
            .animate_to(1.0);

        // Animate the Rust logo (center)
        rust_scale
            .keyframes()
            .at(0.0, 0.8)
            .at(0.5, 1.2)
            .at(1.0, 0.8)
            .duration(Duration::from_secs(4))
            .start(&mut rust_scale);

        rust_rotate
            .keyframes()
            .at(0.0, 0.0)
            .at(1.0, 2.0 * PI)
            .duration(Duration::from_secs(20))
            .start(&mut rust_rotate);

        // Animate each crate orbit
        for (i, (motion, rot)) in crate_positions
            .write()
            .iter_mut()
            .zip(crate_rotations.write().iter_mut())
            .enumerate()
        {
            let orbit_duration = Duration::from_secs(10 + i as u64 * 2);
            let delay = Duration::from_millis(i as u64 * 200);

            motion
                .keyframes()
                .at(0.0, 0.0)
                .at(0.25, PI / 2.0)
                .at(0.5, PI)
                .at(0.75, 3.0 * PI / 2.0)
                .at(1.0, 2.0 * PI)
                .duration(orbit_duration)
                .delay(delay)
                .start(motion);

            rot.keyframes()
                .at(0.0, 0.0)
                .at(1.0, 2.0 * PI)
                .duration(Duration::from_secs(5))
                .delay(delay)
                .start(rot);
        }
    });

    rsx! {
        div {
            class: "absolute inset-0 pointer-events-none",
            style: "opacity: {system_opacity.get()}",
            // Rust logo (center)
            div {
                class: "absolute left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2",
                style: "transform: scale({rust_scale.get()}) rotate({rust_rotate.get()}rad);",
                div { class: "w-16 h-16 bg-orange-500 rounded-full flex items-center justify-center",
                    span { class: "text-2xl font-bold text-white", "R" }
                }
            }

            // Orbiting crates
            {
                crates
                    .iter()
                    .enumerate()
                    .map(|(i, (name, orbit_radius, size))| {
                        let angle = crate_positions.read()[i].get();
                        let rotation = crate_rotations.read()[i].get();
                        let x = orbit_radius * angle.cos();
                        let y = orbit_radius * angle.sin();
                        rsx! {
                            // Orbit path
                            div {
                                key: "orbit-{i}",
                                class: "absolute left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 rounded-full border border-orange-500/20",
                                style: "width: {orbit_radius * 2.0}px; height: { orbit_radius * 2.0}px;",
                            }
                            div {
                                key: "crate-{i}",
                                class: "absolute left-1/2 top-1/2 bg-gradient-to-br from-orange-400 to-red-500 rounded-lg flex items-center justify-center shadow-lg",
                                style: "transform: translate({x}px, {y}px) rotate({rotation}rad); width: {size}px; height: {size}px;",
                                span { class: "text-[8px] font-semibold text-white", "{name}" }
                            }
                        }
                    })
            }
        }
    }
}

#[component]
fn LearnMore() -> Element {
    let scale = use_motion(0.95);
    let opacity = use_motion(0.0);
    let mut rotate = use_motion(0.0);

    use_effect(move || {
        scale
            .spring()
            .stiffness(100.0)
            .damping(15.0)
            .animate_to(1.0);

        opacity
            .tween()
            .duration(Duration::from_millis(800))
            .animate_to(1.0);

        rotate
            .keyframes()
            .at(0.0, 0.0)
            .at(0.5, PI / 12.0)
            .at(1.0, 0.0)
            .duration(Duration::from_secs(20))
            .start(&mut rotate);
    });

    let style = use_memo(move || {
        format!(
            "transform: scale({}) rotate({}rad); opacity: {};",
            scale.get(),
            rotate.get(),
            opacity.get()
        )
    });

    rsx! {
        div { class: "text-center mb-8", style: "{style}",
            button { class: "px-6 py-3 bg-gradient-to-r from-orange-500 to-red-500 rounded-full text-white font-semibold",
                "Learn More"
            }
        }
    }
}

#[component]
fn FeatureCards() -> Element {
    rsx! {
        div { class: "grid grid-cols-1 md:grid-cols-3 gap-8 max-w-5xl mx-auto",
            // Individual cards with adjusted transparency
            div { class: "bg-zinc-800/20 rounded-xl p-6 backdrop-blur-sm border border-zinc-700/30 hover:bg-zinc-800/30 transition-colors",
                h3 { class: "text-xl font-semibold mb-3", "Spring Physics" }
                p { class: "text-zinc-400",
                    "Natural-feeling animations powered by spring physics simulation."
                }
            }

            div { class: "bg-zinc-800/20 rounded-xl p-6 backdrop-blur-sm border border-zinc-700/30 hover:bg-zinc-800/30 transition-colors",
                h3 { class: "text-xl font-semibold mb-3", "Type Safe" }
                p { class: "text-zinc-400", "Leverage Rust's type system for reliable animations." }
            }

            div { class: "bg-zinc-800/20 rounded-xl p-6 backdrop-blur-sm border border-zinc-700/30 hover:bg-zinc-800/30 transition-colors",
                h3 { class: "text-xl font-semibold mb-3", "Performant" }
                p { class: "text-zinc-400", "Optimized animation engine with minimal overhead." }
            }
        }
    }
}

#[component]
fn Footer() -> Element {
    rsx! {
        footer { class: "py-8 text-center text-zinc-500",
            p { "Built with Rust + Dioxus" }
        }
    }
}
