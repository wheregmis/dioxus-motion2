//! Keyframe animation implementation
//!
//! Provides support for animating through multiple keyframes with
//! custom timing and easing functions.

use easer::functions::{Easing, Linear};
use instant::Duration;
use ordered_float::OrderedFloat;
use std::collections::BTreeMap;

use crate::Animatable;
use crate::animation::{Animation, AnimationState, AnimationTiming};

/// Type alias for easing functions from the easer package
// In timing.rs
pub type EasingFunction = fn(f32, f32, f32, f32) -> f32;

/// A keyframe with value and optional easing function
#[derive(Clone)]
pub struct Keyframe<T: Animatable> {
    /// The value at this keyframe
    pub value: T,
    /// Optional easing function for interpolation from this keyframe to the next
    pub easing: Option<EasingFunction>,
}

impl<T: Animatable> Keyframe<T> {
    /// Create a new keyframe with a value
    pub fn new(value: T) -> Self {
        Self {
            value,
            easing: None,
        }
    }

    /// Create a new keyframe with a value and easing function
    pub fn with_easing(value: T, easing: EasingFunction) -> Self {
        Self {
            value,
            easing: Some(easing),
        }
    }
}

/// A keyframe animation with multiple time positions
pub struct KeyframeAnimation<T: Animatable> {
    /// Keyframes indexed by position (0.0 to 1.0)
    keyframes: BTreeMap<OrderedFloat<f32>, Keyframe<T>>,
    /// Total duration of the animation
    duration: Duration,
    /// Animation timing parameters
    timing: AnimationTiming,
    /// Current time in the animation
    current_time: Duration,
    /// Current value
    current: T,
    /// Current velocity (approximated)
    velocity: T,
    /// Previous update time
    prev_time: Duration,
    /// Previous value
    prev_value: T,
    /// Whether the animation is active
    is_active: bool,
}

impl<T: Animatable> KeyframeAnimation<T> {
    /// Create a new keyframe animation with specified duration
    pub fn new(duration: Duration) -> Self {
        // Create a default animation with 0% and 100% keyframes
        let mut keyframes = BTreeMap::new();
        let default_value = T::zero();

        keyframes.insert(OrderedFloat(0.0), Keyframe::new(default_value));
        keyframes.insert(OrderedFloat(1.0), Keyframe::new(default_value));

        Self {
            keyframes,
            duration,
            timing: AnimationTiming::default(),
            current_time: Duration::ZERO,
            current: default_value,
            velocity: T::zero(),
            prev_time: Duration::ZERO,
            prev_value: default_value,
            is_active: true,
        }
    }

    /// Add a keyframe at a specific position (0.0 to 1.0)
    pub fn add_keyframe(mut self, position: f32, value: T) -> Self {
        let position = position.clamp(0.0, 1.0);
        self.keyframes
            .insert(OrderedFloat(position), Keyframe::new(value));
        self
    }

    /// Add a keyframe with a custom easing function
    pub fn add_keyframe_with_easing(
        mut self,
        position: f32,
        value: T,
        easing: EasingFunction,
    ) -> Self {
        let position = position.clamp(0.0, 1.0);
        self.keyframes
            .insert(OrderedFloat(position), Keyframe::with_easing(value, easing));
        self
    }

    /// Add a keyframe with a cubic ease-in easing
    pub fn add_keyframe_ease_in(self, position: f32, value: T) -> Self {
        self.add_keyframe_with_easing(position, value, easer::functions::Cubic::ease_in)
    }

    /// Add a keyframe with a cubic ease-out easing
    pub fn add_keyframe_ease_out(self, position: f32, value: T) -> Self {
        self.add_keyframe_with_easing(position, value, easer::functions::Cubic::ease_out)
    }

    /// Add a keyframe with a cubic ease-in-out easing
    pub fn add_keyframe_ease_in_out(self, position: f32, value: T) -> Self {
        self.add_keyframe_with_easing(position, value, easer::functions::Cubic::ease_in_out)
    }

    /// Add a keyframe with a bounce ease-out easing
    pub fn add_keyframe_bounce(self, position: f32, value: T) -> Self {
        self.add_keyframe_with_easing(position, value, easer::functions::Bounce::ease_out)
    }

    /// Add a keyframe with an elastic ease-out easing
    pub fn add_keyframe_elastic(self, position: f32, value: T) -> Self {
        self.add_keyframe_with_easing(position, value, easer::functions::Elastic::ease_out)
    }

    /// Set the animation duration
    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    /// Set the animation timing parameters
    pub fn with_timing(mut self, timing: AnimationTiming) -> Self {
        self.timing = timing;
        self
    }

    /// Find the surrounding keyframes for a given position
    fn find_surrounding_keyframes(
        &self,
        position: f32,
    ) -> (
        Option<(&OrderedFloat<f32>, &Keyframe<T>)>,
        Option<(&OrderedFloat<f32>, &Keyframe<T>)>,
    ) {
        let mut prev = None;
        let mut next = None;

        for (pos, keyframe) in &self.keyframes {
            if *pos <= ordered_float::OrderedFloat(position) {
                // This keyframe is before or at our position
                prev = Some((pos, keyframe));
            } else {
                // This keyframe is after our position
                next = Some((pos, keyframe));
                break;
            }
        }

        (prev, next)
    }
}

impl<T: Animatable> Animation for KeyframeAnimation<T> {
    type Value = T;

    fn update(&mut self, dt: f32) -> (AnimationState, Self::Value, Self::Value) {
        if !self.is_active {
            return (AnimationState::Completed, self.current, T::zero());
        }

        // Handle delay
        if !self.timing.handle_delay(dt) {
            return (AnimationState::Active, self.current, T::zero());
        }

        // Update timing
        self.prev_time = self.current_time;
        self.prev_value = self.current;
        self.current_time += Duration::from_secs_f32(dt);

        // Check if we've reached the end
        let is_completed = if self.timing.is_reverse() {
            self.current_time >= self.duration
        } else {
            self.current_time >= self.duration
        };

        if is_completed {
            // Handle completion
            if self.timing.handle_loop_completion() {
                // Reset for next loop
                self.current_time = Duration::ZERO;
                self.prev_time = Duration::ZERO;

                // Set to first or last keyframe depending on direction
                if self.timing.is_reverse() {
                    if let Some((_, keyframe)) = self.keyframes.iter().next_back() {
                        self.current = keyframe.value;
                        self.prev_value = keyframe.value;
                    }
                } else if let Some((_, keyframe)) = self.keyframes.iter().next() {
                    self.current = keyframe.value;
                    self.prev_value = keyframe.value;
                }

                return (AnimationState::Active, self.current, T::zero());
            } else {
                // Animation is done
                self.is_active = false;

                // Set to final keyframe
                if self.timing.is_reverse() {
                    if let Some((_, keyframe)) = self.keyframes.iter().next() {
                        self.current = keyframe.value;
                    }
                } else if let Some((_, keyframe)) = self.keyframes.iter().next_back() {
                    self.current = keyframe.value;
                }

                return (AnimationState::Completed, self.current, T::zero());
            }
        }

        // Calculate current position in animation (0.0 to 1.0)
        let mut position =
            (self.current_time.as_secs_f32() / self.duration.as_secs_f32()).clamp(0.0, 1.0);

        // Apply direction
        if self.timing.is_reverse() {
            position = 1.0 - position;
        }

        // Find surrounding keyframes
        let (prev_keyframe, next_keyframe) = self.find_surrounding_keyframes(position);

        // Interpolate between keyframes
        if let (Some((prev_pos, prev_kf)), Some((next_pos, next_kf))) =
            (prev_keyframe, next_keyframe)
        {
            let segment_length = next_pos - prev_pos;
            let segment_position = if segment_length > ordered_float::OrderedFloat(0.0) {
                (position - **prev_pos) / *segment_length
            } else {
                0.0
            };

            // Apply easing if specified
            let eased_position = prev_kf.easing.map_or_else(
                || Linear::ease_in_out(segment_position, 0.0, 1.0, 1.0),
                |easing| easing(segment_position, 0.0, 1.0, 1.0),
            );

            // Interpolate value
            self.current = prev_kf.value.interpolate(&next_kf.value, eased_position);
        } else if let Some((_, kf)) = prev_keyframe {
            // We're past the last keyframe or before the first one
            self.current = kf.value;
        }

        // Calculate velocity
        let dt_duration = Duration::from_secs_f32(dt);
        if dt_duration > Duration::ZERO {
            self.velocity = self.prev_value.sub(&self.current).scale(dt);
        }

        (AnimationState::Active, self.current, self.velocity)
    }

    fn value(&self) -> Self::Value {
        self.current
    }

    fn velocity(&self) -> Self::Value {
        self.velocity
    }

    fn reset(&mut self) {
        self.current_time = Duration::ZERO;
        self.prev_time = Duration::ZERO;

        // Reset to initial value
        if let Some((_, keyframe)) = self.keyframes.iter().next() {
            self.current = keyframe.value;
            self.prev_value = keyframe.value;
        }

        self.velocity = T::zero();
        self.timing.current_loop = 0;
        self.timing.delay_elapsed = false;
        self.is_active = true;
    }

    fn is_active(&self) -> bool {
        self.is_active
    }
}
