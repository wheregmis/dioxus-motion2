use dioxus::prelude::*;
use dioxus_motion2::prelude::*;

#[component]
/// Renders a Dioxus component that showcases a hover-triggered transform animation effect.
///
/// When hovered, the component animates a card by translating, scaling, and rotating it using a spring-based motion,
/// while a glow effect provides additional visual feedback. The animation resets once the mouse leaves,
/// and any ongoing animations are halted when the component is dropped.
///
/// # Examples
///
/// ```
/// use dioxus::desktop::launch;
/// use transformation_example::TransformAnimationShowcase;
///
/// fn main() {
///     launch(TransformAnimationShowcase);
/// }
/// ```
pub fn TransformAnimationShowcase() -> Element {
    let transform = use_motion(Transform::identity());

    let animate_hover = move |_| {
        transform
            .spring()
            .stiffness(180.0)
            .damping(12.0)
            .animate_to(Transform::new(
                0.0,                                  // x
                -20.0,                                // y
                1.1,                                  // scale_x
                1.1,                                  // scale_y
                5.0 * (std::f32::consts::PI / 180.0), // rotation in radians
                0.0,                                  // skew_x
                0.0,                                  // skew_y
            ));
    };

    let animate_reset = move |_| {
        transform
            .spring()
            .stiffness(200.0)
            .damping(20.0)
            .animate_to(Transform::identity());
    };

    let style = use_memo(move || format!("transform: {};", transform.get().to_css_string()));

    rsx! {
        div {
            class: "w-48 h-48 bg-primary/20 backdrop-blur-sm rounded-xl border border-primary/20
                   shadow-lg shadow-primary/10 transition-all duration-300
                   hover:border-primary/30 hover:shadow-xl hover:shadow-primary/20",
            style: "{style}",
            onmouseenter: animate_hover,
            onmouseleave: animate_reset,
            div { class: "w-full h-full flex items-center justify-center text-primary font-medium",
                "Hover Me"
            }
        }
    }
}
