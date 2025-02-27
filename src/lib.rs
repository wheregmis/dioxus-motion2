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

// Core module imports
mod animatable;
mod animation;
mod core;
mod platform;

// Animation type modules
mod color;
mod keyframe;
mod spring;
mod transform;
mod tween;

mod sequence;
mod stagger;

// Re-exports for ease of use
pub use animatable::Animatable;
pub use animation::{Animation, AnimationState};
pub use color::Color;
pub use core::{AnimationEngine, MotionValue};
pub use keyframe::KeyframeAnimation;
pub use platform::{MotionTime, TimeProvider};
pub use sequence::AnimationSequence;
pub use spring::Spring;
pub use stagger::StaggeredAnimation;
pub use transform::Transform;
pub use tween::Tween;

/// Public prelude containing commonly used types and functions
pub mod prelude {
    pub use crate::Duration;
    pub use crate::animatable::Animatable;
    pub use crate::animation::{Animation, AnimationState};
    pub use crate::color::Color;
    pub use crate::core::{AnimationEngine, MotionValue};
    pub use crate::keyframe::{KeyframeAnimation, keyframes};
    pub use crate::sequence::{AnimationSequence, sequence};
    pub use crate::spring::Spring;
    pub use crate::stagger::{StaggeredAnimation, stagger};
    pub use crate::transform::Transform;
    pub use crate::tween::Tween;
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
