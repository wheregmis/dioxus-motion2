//! Animation groups for synchronized animations
//!
//! Provides support for running multiple animations together in a coordinated manner,
//! with shared timing and completion handling.

use instant::Duration;
use std::marker::PhantomData;
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

pub struct GroupItem<T: Animatable> {
    /// The animation to run
    pub animation: Box<dyn Animation<Value = T> + Send>,
    /// Whether this animation has completed
    pub completed: bool,
}

/// A group of animations that run in parallel
pub struct AnimationGroup<T: Animatable> {
    /// The animations in this group
    pub animations: Vec<GroupItem<T>>,
    /// Whether the group is active
    pub is_active: bool,
    /// Completion callback
    pub on_complete: Option<Arc<Mutex<dyn FnMut() + Send>>>,
    /// Phantom data for the animation value type
    _phantom: PhantomData<T>,
}

impl<T: Animatable> Default for AnimationGroup<T> {
    fn default() -> Self {
        Self {
            animations: Vec::new(),
            is_active: false,
            on_complete: None,
            _phantom: PhantomData,
        }
    }
}

impl<T: Animatable> AnimationGroup<T> {
    /// Create a new empty animation group
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an animation to the group
    pub fn add_animation<A: Animation<Value = T> + Send + 'static>(mut self, animation: A) -> Self {
        self.animations.push(GroupItem {
            animation: Box::new(animation),
            completed: false,
        });
        self
    }

    /// Set a completion callback
    pub fn on_complete<F: FnMut() + Send + 'static>(mut self, callback: F) -> Self {
        self.on_complete = Some(Arc::new(Mutex::new(callback)));
        self
    }

    /// Start the animation group
    pub fn start(mut self) -> Self {
        if !self.animations.is_empty() {
            self.is_active = true;
            // Mark all animations as not completed
            for anim in &mut self.animations {
                anim.completed = false;
            }
        }
        self
    }

    /// Build an animation for use with a MotionValue
    pub fn build(self) -> Box<dyn Animation<Value = T> + Send + 'static> {
        Box::new(self)
    }
}

impl<T: Animatable> Animation for AnimationGroup<T> {
    type Value = T;

    fn update(&mut self, dt: f32) -> (AnimationState, Self::Value, Self::Value) {
        if !self.is_active || self.animations.is_empty() {
            return (AnimationState::Completed, T::zero(), T::zero());
        }

        let mut all_completed = true;
        let mut combined_value = T::zero();
        let mut combined_velocity = T::zero();

        // Update all animations
        for anim in &mut self.animations {
            if !anim.completed {
                let (state, value, velocity) = anim.animation.update(dt);

                // Combine values and velocities
                combined_value = combined_value.add(&value);
                combined_velocity = combined_velocity.add(&velocity);

                if state == AnimationState::Completed {
                    anim.completed = true;
                } else {
                    all_completed = false;
                }
            }
        }

        // Check if all animations are completed
        if all_completed {
            self.is_active = false;

            // Call completion callback if provided
            if let Some(callback) = &self.on_complete {
                if let Ok(mut callback) = callback.lock() {
                    (callback)();
                }
            }

            return (AnimationState::Completed, combined_value, combined_velocity);
        }

        (AnimationState::Active, combined_value, combined_velocity)
    }

    fn value(&self) -> Self::Value {
        // Combine values from all animations
        let mut combined_value = T::zero();
        for anim in &self.animations {
            combined_value = combined_value.add(&anim.animation.value());
        }
        combined_value
    }

    fn velocity(&self) -> Self::Value {
        // Combine velocities from all animations
        let mut combined_velocity = T::zero();
        for anim in &self.animations {
            combined_velocity = combined_velocity.add(&anim.animation.velocity());
        }
        combined_velocity
    }

    fn reset(&mut self) {
        // Reset all animations
        for anim in &mut self.animations {
            anim.animation.reset();
            anim.completed = false;
        }
        self.is_active = !self.animations.is_empty();
    }

    fn is_active(&self) -> bool {
        self.is_active
    }
}

/// Helper function to create an animation group
pub fn group<T: Animatable>() -> AnimationGroup<T> {
    AnimationGroup::new()
}
