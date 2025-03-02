//! Tween animation module
//!
//! Provides time-based animation with customizable easing functions.
//! Supports duration and interpolation control for smooth animations.

use dioxus::signals::Writable;
use easer::functions::{Easing, Linear};
use instant::Duration;

use crate::animation::{Animation, AnimationState, AnimationTiming};
use crate::{Animatable, MotionValue};

/// Type alias for easing functions from the easer package
pub type EasingFunction = fn(f32, f32, f32, f32) -> f32;

/// Tween animation with configurable duration and easing
///
#[derive(Debug, Clone, Copy)]
pub struct Tween {
    /// Duration of the animation
    pub duration: Duration,
    /// Easing function for interpolation
    pub easing: EasingFunction,
}

impl Default for Tween {
    fn default() -> Self {
        Self {
            duration: Duration::from_millis(300),
            easing: Linear::ease_in_out,
        }
    }
}

impl Tween {
    /// Create a new tween with default parameters
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the animation duration
    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    /// Set the easing function
    pub fn easing(mut self, easing: EasingFunction) -> Self {
        self.easing = easing;
        self
    }

    /// Create a tween animation with the current configuration
    pub fn create_animation<T: Animatable>(&self, initial: T, target: T) -> TweenAnimation<T> {
        TweenAnimation::new(initial, target, *self, AnimationTiming::default())
    }
}

/// Tween animation implementation
pub struct TweenAnimation<T: Animatable> {
    /// Initial value
    initial: T,
    /// Current value
    current: T,
    /// Target value
    target: T,
    /// Tween configuration
    tween: Tween,
    /// Animation timing parameters
    timing: AnimationTiming,
    /// Elapsed time
    elapsed: Duration,
    /// Whether the animation is active
    is_active: bool,
}

impl<T: Animatable> TweenAnimation<T> {
    /// Create a new tween animation
    pub fn new(initial: T, target: T, tween: Tween, timing: AnimationTiming) -> Self {
        Self {
            initial,
            current: initial,
            target,
            tween,
            timing,
            elapsed: Duration::ZERO,
            is_active: true,
        }
    }
}

impl<T: Animatable> Animation for TweenAnimation<T> {
    type Value = T;

    fn update(&mut self, dt: f32) -> (AnimationState, Self::Value, Self::Value) {
        if !self.is_active {
            return (AnimationState::Completed, self.current, T::zero());
        }

        // Handle delay
        if !self.timing.handle_delay(dt) {
            return (AnimationState::Active, self.current, T::zero());
        }

        // Update elapsed time
        self.elapsed += Duration::from_secs_f32(dt);

        // Calculate progress (0.0 to 1.0)
        let duration = self.tween.duration.as_secs_f32();
        let mut progress = if duration > 0.0 {
            (self.elapsed.as_secs_f32() / duration).clamp(0.0, 1.0)
        } else {
            1.0
        };

        // Apply direction
        if self.timing.is_reverse() {
            progress = 1.0 - progress;
        }

        // Apply easing function with easer's standard parameters
        let eased_progress = (self.tween.easing)(progress, 0.0, 1.0, 1.0);

        // Update current value
        self.current = self.initial.interpolate(&self.target, eased_progress);

        // Calculate velocity (approximation)
        let velocity = if dt > 0.0 {
            let prev_progress = if duration > 0.0 {
                ((self.elapsed.as_secs_f32() - dt) / duration).clamp(0.0, 1.0)
            } else {
                1.0
            };

            let prev_eased = (self.tween.easing)(prev_progress, 0.0, 1.0, 1.0);
            let prev_value = self.initial.interpolate(&self.target, prev_eased);

            prev_value.sub(&self.current).scale(1.0 / dt)
        } else {
            T::zero()
        };

        // Check for completion
        let completed = if self.timing.is_reverse() {
            progress <= 0.0
        } else {
            progress >= 1.0
        };

        if completed {
            // Snap to the correct end value based on direction
            self.current = if self.timing.is_reverse() {
                self.initial
            } else {
                self.target
            };

            // Handle loop completion
            if self.timing.handle_loop_completion() {
                // Reset for next loop
                self.elapsed = Duration::ZERO;
                (AnimationState::Active, self.current, velocity)
            } else {
                self.is_active = false;
                (AnimationState::Completed, self.current, T::zero())
            }
        } else {
            (AnimationState::Active, self.current, velocity)
        }
    }

    fn value(&self) -> Self::Value {
        self.current
    }

    fn velocity(&self) -> Self::Value {
        // Velocity is approximated in update method
        T::zero()
    }

    fn reset(&mut self) {
        self.current = self.initial;
        self.elapsed = Duration::ZERO;
        self.timing.current_loop = 0;
        self.timing.delay_elapsed = false;
        self.is_active = true;
    }

    fn is_active(&self) -> bool {
        self.is_active
    }
}

/// Builder for tween animations
pub struct TweenBuilder<T: Animatable> {
    motion: MotionValue<T>,
    tween: Tween,
    target: Option<T>,
    completion_callback: Option<Box<dyn FnOnce() + Send>>,
}

impl<T: Animatable> TweenBuilder<T> {
    /// Create a new tween builder
    pub(crate) fn new(motion: MotionValue<T>) -> Self {
        Self {
            motion,
            tween: Tween::default(),
            completion_callback: None,
            target: None,
        }
    }

    /// Set tween duration
    pub fn duration(mut self, duration: Duration) -> Self {
        self.tween.duration = duration;
        self
    }

    /// Set easing function
    pub fn easing(mut self, easing: fn(f32, f32, f32, f32) -> f32) -> Self {
        self.tween.easing = easing;
        self
    }

    /// Add completion callback
    pub fn on_complete<F: FnOnce() + Send + 'static>(mut self, callback: F) -> Self {
        self.completion_callback = Some(Box::new(callback));
        self
    }

    /// Set the target value for the animation
    pub fn to(mut self, target: T) -> Self {
        self.target = Some(target);
        self
    }

    /// Build the animation for use in sequences or groups
    pub fn build(self) -> Box<dyn Animation<Value = T> + Send> {
        let target = self.target.unwrap_or_else(|| self.motion.get());
        Box::new(self.tween.create_animation(self.motion.get(), target))
    }

    /// Start animation to target value
    pub fn animate_to(mut self, target: T) -> MotionValue<T> {
        // Apply the completion callback if provided
        if let Some(callback) = self.completion_callback {
            self.motion.engine.write().add_completion_callback(callback);
        }

        self.motion.engine.write().tween_to(target, self.tween);
        self.motion
    }

    /// Create a sequence-compatible tween animation
    pub fn into_sequence(self) -> Box<dyn Animation<Value = T> + Send> {
        let target = self.target.unwrap_or_else(|| self.motion.get());
        Box::new(self.tween.create_animation(self.motion.get(), target))
    }
}
