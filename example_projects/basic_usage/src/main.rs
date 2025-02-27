use std::f32::consts::PI;

use dioxus::prelude::*;
use dioxus_logger::tracing::Level;
use dioxus_motion2::prelude::*;
use easer::functions::Easing;

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus_logger::init(Level::DEBUG).expect("failed to init logger");
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        AnimationExamples {}
    }
}

/// Example component showcasing various animation types in Dioxus Motion
#[component]
pub fn AnimationExamples() -> Element {
    // Container for examples
    rsx! {
        div { class: "p-8 max-w-4xl mx-auto",
            h1 { class: "text-3xl font-bold mb-8", "Dioxus Motion Animation Examples" }

            // Spring animation example
            SpringExample {}

            // Tween animation example
            TweenExample {}

            // Keyframe animation example
            KeyframeExample {}

            // Transform animation example
            TransformExample {}

            // Color animation example
            ColorExample {}

            // Sequence animation example
            SequenceExample {}

            // Staggered animation example
            StaggeredExample {}

        }
    }
}

/// Basic spring animation example
#[component]
fn SpringExample() -> Element {
    // Create a spring-animated value starting at 0.0
    let position = use_motion(0.0);

    let start_animation = move |_: Event<MouseData>| {
        // Configure and start a spring animation
        position
            .spring()
            .stiffness(180.0) // Higher = stronger spring
            .damping(12.0) // Higher = less bouncy
            .mass(1.0) // Higher = more inertia
            .on_complete(|| {
                println!("Spring animation completed!");
            })
            .animate_to(200.0);
    };

    let reset_animation = move |_| {
        position.spring().stiffness(150.0).animate_to(0.0);
    };

    // Generate the style based on the animated value
    let box_style = use_memo(move || {
        format!(
            "transform: translateX({}px); transition: background-color 0.3s;",
            position.get()
        )
    });

    rsx! {
        section { class: "mb-12 border-b pb-8",
            h2 { class: "text-2xl font-semibold mb-4", "Spring Animation" }
            p { class: "mb-4",
                "Spring animations use physics to create natural motion with configurable stiffness, damping, and mass."
            }

            div { class: "my-6 relative h-24",
                div {
                    class: "absolute top-0 left-0 w-16 h-16 bg-blue-500 rounded shadow-md flex items-center justify-center text-white",
                    style: "{box_style.read()}",
                    "Box"
                }
            }

            div { class: "flex gap-4",
                button {
                    class: "px-4 py-2 bg-blue-500 hover:bg-blue-600 text-white rounded",
                    onclick: start_animation,
                    "Start Spring"
                }
                button {
                    class: "px-4 py-2 bg-gray-300 hover:bg-gray-400 rounded",
                    onclick: reset_animation,
                    "Reset"
                }
            }

            pre { class: "mt-4 p-4 bg-gray-100 rounded overflow-x-auto text-sm",
                code {
                   { r#"// Create a spring-animated value
        let position = use_motion(0.0);

        // Start a spring animation
        position.spring()
            .stiffness(180.0)  // Higher = stronger spring
            .damping(12.0)     // Higher = less bouncy
            .mass(1.0)         // Higher = more inertia
            .on_complete(|| {
                println!("Animation completed!");
            })
            .animate_to(200.0);"#}
                }
            }
        }
    }
}

/// Basic tween animation example
#[component]
fn TweenExample() -> Element {
    // Create a tween-animated value for opacity
    let opacity = use_motion(0.0);

    // Button click handlers for animation

    let start_animation = move |_| {
        opacity
            .tween()
            .duration(Duration::from_millis(800))
            .easing(easer::functions::Cubic::ease_in_out)
            .animate_to(1.0);
    };

    let reset_animation = move |_| {
        opacity
            .tween()
            .duration(Duration::from_millis(500))
            .easing(easer::functions::Cubic::ease_out)
            .animate_to(0.0);
    };

    rsx! {
            section { class: "mb-12 border-b pb-8",
                h2 { class: "text-2xl font-semibold mb-4", "Tween Animation" }
                p { class: "mb-4", "Tween animations use time-based interpolation with various easing functions." }

                div { class: "my-6 relative h-24",
                    div {
                        class: "w-16 h-16 bg-purple-500 rounded shadow-md flex items-center justify-center text-white",
                        style: "opacity: {opacity.get()};",
                        "Box"
                    }
                }

                div { class: "flex gap-4",
                    button {
                        class: "px-4 py-2 bg-purple-500 hover:bg-purple-600 text-white rounded",
                        onclick: start_animation,
                        "Fade In"
                    }
                    button {
                        class: "px-4 py-2 bg-gray-300 hover:bg-gray-400 rounded",
                        onclick: reset_animation,
                        "Fade Out"
                    }
                }

                pre { class: "mt-4 p-4 bg-gray-100 rounded overflow-x-auto text-sm",
                    code {
    {r#"// Create a tween-animated value
let opacity = use_motion(0.0);

// Start a tween animation
opacity.tween()
    .duration(Duration::from_millis(800))
    .easing(easer::functions::Cubic::ease_in_out)
    .animate_to(1.0);"#}
                    }
                }
            }
        }
}

/// Keyframe animation example
#[component]
fn KeyframeExample() -> Element {
    // Create a keyframe-animated value for position
    let position = use_motion(0.0);

    // Button click handlers for animation
    let start_animation = move |_| {
        // Configure and start a keyframe animation
        position
            .keyframes()
            .keyframe(0.0, 0.0)
            .keyframe_with_easing(0.3, 150.0, easer::functions::Cubic::ease_out)
            .keyframe_with_easing(0.7, 50.0, easer::functions::Bounce::ease_out)
            .keyframe(1.0, 100.0)
            .duration(Duration::from_millis(2000))
            .start();
    };

    let reset_animation = move |_| {
        position
            .tween()
            .duration(Duration::from_millis(500))
            .animate_to(0.0);
    };

    // Generate the style based on the animated value
    let box_style = use_memo(move || format!("transform: translateX({}px);", position.get()));

    rsx! {
            section { class: "mb-12 border-b pb-8",
                h2 { class: "text-2xl font-semibold mb-4", "Keyframe Animation" }
                p { class: "mb-4", "Keyframe animations let you define multiple points with different easing functions between them." }

                div { class: "my-6 relative h-24",
                    div {
                        class: "absolute top-0 left-0 w-16 h-16 bg-green-500 rounded shadow-md flex items-center justify-center text-white",
                        style: "{box_style.read()}",
                        "Box"
                    }
                }

                div { class: "flex gap-4",
                    button {
                        class: "px-4 py-2 bg-green-500 hover:bg-green-600 text-white rounded",
                        onclick: start_animation,
                        "Play Keyframes"
                    }
                    button {
                        class: "px-4 py-2 bg-gray-300 hover:bg-gray-400 rounded",
                        onclick: reset_animation,
                        "Reset"
                    }
                }

                pre { class: "mt-4 p-4 bg-gray-100 rounded overflow-x-auto text-sm",
                    code {
    {r#"// Create a keyframe-animated value
let position = use_motion(0.0);

// Start a keyframe animation
position.keyframes()
    .keyframe(0.0, 0.0)
    .keyframe_with_easing(0.3, 150.0, easer::functions::Cubic::ease_out)
    .keyframe_with_easing(0.7, 50.0, easer::functions::Bounce::ease_out)
    .keyframe(1.0, 100.0)
    .duration(Duration::from_millis(2000))
    .start();"#}
                    }
                }
            }
        }
}

/// Transform animation example
#[component]
fn TransformExample() -> Element {
    // Create a motion value for transform
    let transform = use_motion(Transform::identity());

    // Button click handlers for animation

    let start_animation = move |_| {
        // Animate multiple transform properties
        transform
            .spring()
            .stiffness(120.0)
            .damping(10.0)
            .animate_to(Transform::new(
                100.0,    // x translation
                -30.0,    // y translation
                1.5,      // x scale
                1.5,      // y scale
                PI / 8.0, // rotation (radians)
                0.0,      // x skew
                0.0,      // y skew
            ));
    };

    let reset_animation = move |_| {
        transform
            .spring()
            .stiffness(150.0)
            .damping(20.0)
            .animate_to(Transform::identity());
    };

    // Generate the style based on the transform
    let box_style = use_memo(move || format!("transform: {};", transform.get().to_css_string()));

    rsx! {
            section { class: "mb-12 border-b pb-8",
                h2 { class: "text-2xl font-semibold mb-4", "Transform Animation" }
                p { class: "mb-4", "Transform animations let you animate multiple properties (translate, scale, rotate, skew) simultaneously." }

                div { class: "my-6 relative h-32",
                    div {
                        class: "absolute top-8 left-0 w-16 h-16 bg-indigo-500 rounded shadow-md flex items-center justify-center text-white",
                        style: "{box_style.read()}",
                        "Box"
                    }
                }

                div { class: "flex gap-4",
                    button {
                        class: "px-4 py-2 bg-indigo-500 hover:bg-indigo-600 text-white rounded",
                        onclick: start_animation,
                        "Animate Transform"
                    }
                    button {
                        class: "px-4 py-2 bg-gray-300 hover:bg-gray-400 rounded",
                        onclick: reset_animation,
                        "Reset"
                    }
                }

                pre { class: "mt-4 p-4 bg-gray-100 rounded overflow-x-auto text-sm",
                    code {
    {r#"// Create a transform-animated value
let transform = use_motion(Transform::identity());

// Animate multiple transform properties
transform.spring()
    .stiffness(120.0)
    .damping(10.0)
    .animate_to(Transform::new(
        100.0,     // x translation
        -30.0,     // y translation
        1.5,       // x scale
        1.5,       // y scale
        PI / 8.0,  // rotation (radians)
        0.0,       // x skew
        0.0        // y skew
    ));"#}
                    }
                }
            }
        }
}

/// Color animation example
#[component]
fn ColorExample() -> Element {
    // Create a motion value for color
    let color = use_motion(Color::from_rgba(100, 100, 100, 255));

    // Button click handlers for animation
    let animate_red = move |_| {
        color
            .tween()
            .duration(Duration::from_millis(500))
            .easing(easer::functions::Cubic::ease_out)
            .animate_to(Color::from_rgba(220, 50, 50, 255));
    };

    let animate_green = move |_| {
        color
            .tween()
            .duration(Duration::from_millis(500))
            .easing(easer::functions::Cubic::ease_out)
            .animate_to(Color::from_rgba(50, 180, 50, 255));
    };

    let animate_blue = move |_| {
        color
            .tween()
            .duration(Duration::from_millis(500))
            .easing(easer::functions::Cubic::ease_out)
            .animate_to(Color::from_rgba(50, 50, 220, 255));
    };

    let reset_animation = move |_| {
        color
            .tween()
            .duration(Duration::from_millis(500))
            .animate_to(Color::from_rgba(100, 100, 100, 255));
    };

    // Generate the style based on the color
    let box_style = use_memo(move || format!("background-color: {};", color.get().to_css_string()));

    rsx! {
        section { class: "mb-12 border-b pb-8",
            h2 { class: "text-2xl font-semibold mb-4", "Color Animation" }
            p { class: "mb-4", "Color animations provide smooth transitions between colors with proper RGB interpolation." }

            div { class: "my-6 flex justify-center",
                div {
                    class: "w-24 h-24 rounded shadow-md flex items-center justify-center text-white",
                    style: "{box_style.read()}",
                    "Color"
                }
            }

            div { class: "flex gap-2 flex-wrap",
                button {
                    class: "px-4 py-2 bg-red-500 hover:bg-red-600 text-white rounded",
                    onclick: animate_red,
                    "Red"
                }
                button {
                    class: "px-4 py-2 bg-green-500 hover:bg-green-600 text-white rounded",
                    onclick: animate_green,
                    "Green"
                }
                button {
                    class: "px-4 py-2 bg-blue-500 hover:bg-blue-600 text-white rounded",
                    onclick: animate_blue,
                    "Blue"
                }
                button {
                    class: "px-4 py-2 bg-gray-300 hover:bg-gray-400 rounded",
                    onclick: reset_animation,
                    "Reset"
                }
            }

            pre { class: "mt-4 p-4 bg-gray-100 rounded overflow-x-auto text-sm",
                code {
r#"// Create a color-animated value
let color = use_motion(Color::from_rgba(100, 100, 100, 255));

// Animate to a new color
color.tween()
    .duration(Duration::from_millis(500))
    .easing(easer::functions::Cubic::ease_out)
    .animate_to(Color::from_rgba(220, 50, 50, 255));"#
                }
            }
        }
    }
}

/// Sequence animation example
#[component]
fn SequenceExample() -> Element {
    // Create a motion value for position
    let position = use_motion(0.0);

    // Button click handler for sequence animation
    let start_animation = move |_| {
        position
            .sequence()
            .then(
                position
                    .spring()
                    .stiffness(180.0)
                    .damping(12.0)
                    .to(150.0)
                    .build(),
            )
            .then(
                position
                    .spring()
                    .stiffness(180.0)
                    .damping(12.0)
                    .to(50.0)
                    .build(),
            )
            .then(
                position
                    .spring()
                    .stiffness(200.0)
                    .damping(10.0)
                    .to(200.0)
                    .build(),
            )
            .on_complete(|| {
                println!("Sequence completed!");
            })
            .start();
    };

    let reset_animation = move |_| {
        position.tween().animate_to(0.0);
    };

    // Generate the style based on the animated value
    let box_style = use_memo(move || format!("transform: translateX({}px);", position.get()));

    rsx! {
            section { class: "mb-12 border-b pb-8",
                h2 { class: "text-2xl font-semibold mb-4", "Sequence Animation" }
                p { class: "mb-4", "Sequence animations let you chain multiple animations to run one after another." }

                div { class: "my-6 relative h-24",
                    div {
                        class: "absolute top-0 left-0 w-16 h-16 bg-amber-500 rounded shadow-md flex items-center justify-center text-white",
                        style: "{box_style.read()}",
                        "Box"
                    }
                }

                div { class: "flex gap-4",
                    button {
                        class: "px-4 py-2 bg-amber-500 hover:bg-amber-600 text-white rounded",
                        onclick: start_animation,
                        "Start Sequence"
                    }
                    button {
                        class: "px-4 py-2 bg-gray-300 hover:bg-gray-400 rounded",
                        onclick: reset_animation,
                        "Reset"
                    }
                }

                pre { class: "mt-4 p-4 bg-gray-100 rounded overflow-x-auto text-sm",
                    code {
    {r#"// Create a motion value
let position = use_motion(0.0);

// Create and start an animation sequence
position
    .sequence()
    .then(
        position.spring()
            .stiffness(180.0)
            .damping(12.0)
            .to(150.0)
            .build()
    )
    .then(
        position.tween()
            .duration(Duration::from_millis(500))
            .easing(easer::functions::Bounce::ease_out)
            .to(50.0)
            .build()
    )
    .then(
        position.spring()
            .stiffness(200.0)
            .damping(10.0)
            .to(200.0)
            .build()
    )
    .on_complete(|| {
        println!("Sequence completed!");
    })
    .start();"#}
                    }
                }
            }
        }
}

#[component]
fn StaggeredExample() -> Element {
    // Create multiple motion values for staggered animation
    let items = (0..5).collect::<Vec<_>>();
    let motion_values = Signal::new(items.iter().map(|_| use_motion(0.0)).collect::<Vec<_>>());

    let start_animation = move |_: Event<MouseData>| {
        for (i, motion) in motion_values.peek().iter().enumerate() {
            let delay = Duration::from_millis(i as u64 * 1000);

            motion
                .tween()
                .duration(delay)
                .easing(easer::functions::Back::ease_out)
                .animate_to(150.0);
        }
    };

    let reset_animation = move |_: Event<MouseData>| {
        for motion in motion_values.peek().iter() {
            motion
                .tween()
                .duration(Duration::from_millis(300))
                .animate_to(0.0);
        }
    };

    rsx! {
            section { class: "mb-12",
                h2 { class: "text-2xl font-semibold mb-4", "Staggered Animation" }
                p { class: "mb-4", "Staggered animations create cascading effects by starting animations with sequential delays." }

                div { class: "my-6 relative min-h-[200px]",
                    // Render items with staggered animations
                    {items.iter().enumerate().map(|(i, _)| {
                        // Get the position from the motion value
                        let position = motion_values.read()[i].get();

                        // Apply different vertical position for each box
                        rsx! {
                            div {
                                key: "{i}",
                                class: "mb-4 w-16 h-16 bg-teal-500 rounded shadow-md flex items-center justify-center text-white",
                                style: "transform: translateX({position}px);",
                                "Box {i}"
                            }
                        }
                    })}
                }

                div { class: "flex gap-4",
                    button {
                        class: "px-4 py-2 bg-teal-500 hover:bg-teal-600 text-white rounded",
                        onclick: start_animation,
                        "Start Staggered"
                    }
                    button {
                        class: "px-4 py-2 bg-gray-300 hover:bg-gray-400 rounded",
                        onclick: reset_animation,
                        "Reset"
                    }
                }

                pre { class: "mt-4 p-4 bg-gray-100 rounded overflow-x-auto text-sm",
                    code {
    {                    r#"// Create multiple motion values
let items = (0..5).collect::<Vec<_>>();
let motion_values = use_signal(|| {
    items.iter().map(|_| use_motion(0.0)).collect::<Vec<_>>()
});

// Start a staggered animation
for (i, motion) in motion_values.read().iter().enumerate() {
    // Staggered animation with delay based on index
    motion.tween()
        .duration(Duration::from_millis(600))
        .easing(easer::functions::Back::ease_out)
        .animate_to(150.0);
}"#}
                    }
                }
            }
        }
}
