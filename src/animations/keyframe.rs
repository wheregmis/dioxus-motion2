//! Keyframe animation implementation
//!
//! Provides support for animating through multiple keyframes with
//! custom timing and easing functions.

use dioxus::signals::Writable;
use easer::functions::{Easing, Linear};
use instant::Duration;
use ordered_float::OrderedFloat;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use crate::animation::{Animation, AnimationState, AnimationTiming, LoopMode, PlaybackDirection};
use crate::{Animatable, MotionValue};

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

impl<T: Animatable> Default for KeyframeAnimation<T> {
    fn default() -> Self {
        Self {
            keyframes: BTreeMap::new(),
            duration: Duration::from_millis(300),
            timing: AnimationTiming::default(),
            current_time: Duration::ZERO,
            current: T::zero(),
            velocity: T::zero(),
            prev_time: Duration::ZERO,
            prev_value: T::zero(),
            is_active: false,
        }
    }
}

impl<T: Animatable> KeyframeAnimation<T> {
    /// Create a new keyframe animation
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a keyframe at position (0.0 to 1.0)
    pub fn at(mut self, position: f32, value: T) -> Self {
        self.keyframes
            .insert(OrderedFloat(position.clamp(0.0, 1.0)), Keyframe::new(value));
        self
    }

    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    /// Add a keyframe with easing
    pub fn at_with_easing(mut self, position: f32, value: T, easing: EasingFunction) -> Self {
        self.keyframes.insert(
            OrderedFloat(position.clamp(0.0, 1.0)),
            Keyframe::with_easing(value, easing),
        );
        self
    }

    /// Set animation duration
    pub fn for_duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    /// Set loop mode
    pub fn looping(mut self, mode: LoopMode) -> Self {
        self.timing.loop_mode = mode;
        self
    }

    /// Set playback direction
    pub fn direction(mut self, direction: PlaybackDirection) -> Self {
        self.timing.direction = direction;
        self
    }

    /// Set initial delay
    pub fn delay(mut self, delay: Duration) -> Self {
        self.timing.delay = delay;
        self
    }

    pub fn timing(mut self, timing: AnimationTiming) -> Self {
        self.timing = timing;
        self
    }

    /// Set completion callback
    pub fn on_complete<F>(mut self, f: F) -> Self
    where
        F: FnMut() + Send + 'static,
    {
        self.timing.on_complete = Some(Arc::new(Mutex::new(f)));
        self
    }

    /// Start the animation
    pub fn start(self, motion: &mut MotionValue<T>) -> MotionValue<T> {
        motion.engine.write().apply_keyframes(self);
        *motion
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

impl<T: Animatable> MotionValue<T> {
    /// Start a new keyframe animation
    pub fn keyframes(&self) -> KeyframeAnimation<T> {
        KeyframeAnimation::new()
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
        let is_completed = self.current_time >= self.duration;

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
