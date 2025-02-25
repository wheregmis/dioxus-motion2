//! Animation groups for synchronized animations
//!
//! Provides support for running multiple animations together in a coordinated manner,
//! with shared timing and completion handling.

use instant::Duration;
use std::sync::{Arc, Mutex};

use crate::animatable::Animatable;
use crate::animation::{Animation, AnimationState, AnimationTiming};

/// An animation that can be part of a group
pub trait GroupableAnimation: Animation + Send + 'static {
    /// Clone this animation
    fn clone_box(&self) -> Box<dyn GroupableAnimation<Value = Self::Value>>;
}

// Make groupable animations clonable
impl<T> Clone for Box<dyn GroupableAnimation<Value = T>>
where
    T: Animatable,
{
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

// Implement GroupableAnimation for any Animation
impl<T, A> GroupableAnimation for A
where
    T: Animatable,
    A: Animation<Value = T> + Clone + Send + 'static,
{
    fn clone_box(&self) -> Box<dyn GroupableAnimation<Value = Self::Value>> {
        Box::new(self.clone())
    }
}

/// A group of animations that run together
pub struct AnimationGroup<T: Animatable> {
    /// Animations in this group
    animations: Vec<Box<dyn Animation<Value = T>>>,
    /// Timing parameters
    timing: AnimationTiming,
    /// Current value (combined from all animations)
    current: T,
    /// Current velocity (combined from all animations)
    velocity: T,
    /// Whether the group is active
    is_active: bool,
}

impl<T: Animatable> AnimationGroup<T> {
    /// Create a new empty animation group
    pub fn new() -> Self {
        Self {
            animations: Vec::new(),
            timing: AnimationTiming::default(),
            current: T::zero(),
            velocity: T::zero(),
            is_active: false,
        }
    }

    /// Add an animation to the group
    pub fn add_animation<A: Animation<Value = T> + Send + 'static>(mut self, animation: A) -> Self {
        self.animations.push(Box::new(animation));
        self.is_active = true;
        self
    }

    /// Set the animation timing parameters
    pub fn with_timing(mut self, timing: AnimationTiming) -> Self {
        self.timing = timing;
        self
    }

    /// Set a completion callback
    pub fn on_complete<F: FnOnce() + Send + 'static>(mut self, callback: F) -> Self {
        self.timing.on_complete = Some(Arc::new(Mutex::new({
            let mut opt_callback = Some(callback);
            move || {
                if let Some(cb) = opt_callback.take() {
                    cb();
                }
            }
        })));
        self
    }

    /// Set a delay before the animation starts
    pub fn with_delay(mut self, delay: Duration) -> Self {
        self.timing = self.timing.with_delay(delay);
        self
    }
}

impl<T: Animatable> Animation for AnimationGroup<T> {
    type Value = T;

    fn update(&mut self, dt: f32) -> (AnimationState, Self::Value, Self::Value) {
        if !self.is_active {
            return (AnimationState::Completed, self.current, T::zero());
        }

        // Handle delay
        if !self.timing.handle_delay(dt) {
            return (AnimationState::Active, self.current, T::zero());
        }

        // Track if any animation is still active
        let mut any_active = false;
        let mut combined_value: Option<T> = None;
        let mut combined_velocity: Option<T> = None;

        // Update all animations
        for animation in &mut self.animations {
            let (state, value, velocity) = animation.update(dt);

            // Accumulate values and velocities
            if let Some(ref mut combined) = combined_value {
                *combined = combined.add(&value);
            } else {
                combined_value = Some(value);
            }

            if let Some(ref mut combined) = combined_velocity {
                *combined = combined.add(&velocity);
            } else {
                combined_velocity = Some(velocity);
            }

            if state == AnimationState::Active {
                any_active = true;
            }
        }

        // Update current values
        if let Some(value) = combined_value {
            self.current = value;
        }

        if let Some(velocity) = combined_velocity {
            self.velocity = velocity;
        }

        // Handle completion
        if !any_active {
            if self.timing.handle_loop_completion() {
                // Reset all animations
                for animation in &mut self.animations {
                    animation.reset();
                }
                return (AnimationState::Active, self.current, self.velocity);
            } else {
                self.is_active = false;
                return (AnimationState::Completed, self.current, T::zero());
            }
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
        for animation in &mut self.animations {
            animation.reset();
        }

        self.timing.current_loop = 0;
        self.timing.delay_elapsed = false;
        self.is_active = true;
    }

    fn is_active(&self) -> bool {
        self.is_active
    }
}

/// Helper function to create a new animation group
pub fn group<T: Animatable>() -> AnimationGroup<T> {
    AnimationGroup::new()
}
