//! Dioxus Motion - Animation library for Dioxus
//!
//! Provides smooth animations for web and native applications built with Dioxus.
//! Designed with an intuitive, fluent API that supports both spring physics and tween-based animations.
//!
//! # Features
//! - Spring physics with fluent configuration
//! - Tween animations with customizable easing
//! - Keyframe animations for complex sequences
//! - Animation groups for coordinated motion
//! - Staggered animations for sequential effects
//! - Color interpolation
//! - Transform animations
//!
#![deny(clippy::unwrap_used)]
#![deny(clippy::panic)]
#![deny(unused_variables)]
#![deny(unused_must_use)]
#![deny(unsafe_code)]
#![deny(clippy::unwrap_in_result)]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(clippy::modulo_arithmetic)]
#![deny(clippy::option_if_let_else)]

use dioxus::prelude::*;

pub use instant::Duration;

mod core;
mod platform;

// Animation type modules
mod animation;
mod animations;
mod properties;
mod traits;

// Re-exports for ease of use
pub use animation::{Animation, AnimationState, AnimationTiming};
pub use core::{AnimationEngine, MotionValue};
pub use platform::{MotionTime, TimeProvider};
pub use properties::{color::Color, transform::Transform};
pub use traits::animatable::Animatable;

/// Public prelude containing commonly used types and functions
pub mod prelude {
    pub use crate::Duration;
    pub use crate::animations::sequence;
    pub use crate::core::{AnimationEngine, MotionValue};
    pub use crate::properties::{color::Color, transform::Transform};
    pub use crate::traits::animatable::Animatable;
    pub use crate::use_motion;
}

/// Create a motion value with an initial value
///
/// This is the primary entry point for creating animations
///
pub fn use_motion<T: Animatable>(initial: T) -> MotionValue<T> {
    let animation_engine = AnimationEngine::new(initial);
    let mut signal = use_signal(|| animation_engine);

    use_future(move || async move {
        let mut last_frame = MotionTime::now();

        loop {
            let now = MotionTime::now();
            let dt = now.duration_since(last_frame).as_secs_f32();

            let is_active = signal.write().update(dt);

            // Adaptive frame rate based on activity
            let delay = if is_active {
                if dt > 0.064 {
                    Duration::from_millis(8)
                } else {
                    Duration::from_millis(16)
                }
            } else {
                Duration::from_millis(100)
            };

            last_frame = now;
            MotionTime::delay(delay).await;
        }
    });

    MotionValue::new(signal)
}
