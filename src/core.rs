//! Core animation engine powering Dioxus Motion
//!
//! This module contains the central animation engine and motion value abstractions
//! that form the foundation of the library.

use dioxus::prelude::*;
use instant::Duration;
use std::sync::{Arc, RwLock};

use crate::animatable::Animatable;
use crate::animation::{Animation, AnimationState};
use crate::group::AnimationGroup;
use crate::keyframe::KeyframeAnimation;
use crate::sequence::AnimationSequence;
use crate::spring::Spring;
use crate::stagger::StaggeredAnimation;
use crate::tween::Tween;

/// Core animation engine that manages animations
pub struct AnimationEngine<T: Animatable> {
    /// Current value
    current: T,
    /// Current velocity (for physics-based animations)
    velocity: T,
    /// Current animation, if any
    animation: Option<Box<dyn Animation<Value = T>>>,
    /// Whether the engine is active
    is_active: bool,
}

impl<T: Animatable> AnimationEngine<T> {
    /// Create a new animation engine with initial value
    pub fn new(initial: T) -> Self {
        Self {
            current: initial,
            velocity: T::zero(),
            animation: None,
            is_active: false,
        }
    }

    /// Update the animation engine with time delta
    pub fn update(&mut self, dt: f32) -> bool {
        if !self.is_active {
            return false;
        }

        if let Some(animation) = &mut self.animation {
            let (state, value, velocity) = animation.update(dt);

            self.current = value;
            self.velocity = velocity;

            match state {
                AnimationState::Active => {
                    return true;
                }
                AnimationState::Completed => {
                    self.is_active = false;
                    self.animation = None;
                    self.velocity = T::zero();
                    return false;
                }
            }
        }

        false
    }

    /// Set the current value directly (without animation)
    pub fn set(&mut self, value: T) {
        self.current = value;
        self.velocity = T::zero();
        self.animation = None;
        self.is_active = false;
    }

    /// Get the current value
    pub fn get(&self) -> T {
        self.current
    }

    /// Check if the animation is active
    pub fn is_active(&self) -> bool {
        self.is_active
    }

    /// Start a spring animation
    pub fn spring_to(&mut self, target: T, spring: Spring) {
        self.animation = Some(Box::new(spring.create_animation(
            self.current,
            target,
            self.velocity,
        )));
        self.is_active = true;
    }

    /// Start a tween animation
    pub fn tween_to(&mut self, target: T, tween: Tween) {
        self.animation = Some(Box::new(tween.create_animation(self.current, target)));
        self.is_active = true;
    }

    /// Stop any active animation
    pub fn stop(&mut self) {
        self.animation = None;
        self.is_active = false;
    }

    /// Apply a keyframe animation
    pub fn apply_keyframes(&mut self, keyframes: KeyframeAnimation<T>) {
        self.animation = Some(Box::new(keyframes));
        self.is_active = true;
    }

    /// Apply an animation group
    pub fn apply_group(&mut self, group: AnimationGroup<T>) {
        self.animation = Some(Box::new(group));
        self.is_active = true;
    }

    /// Apply an animation sequence
    pub fn apply_sequence(&mut self, sequence: AnimationSequence<T>) {
        self.animation = Some(Box::new(sequence));
        self.is_active = true;
    }

    /// Apply a staggered animation
    pub fn apply_staggered<A: Animation<Value = T>>(
        &mut self,
        staggered: StaggeredAnimation<T, A>,
    ) {
        // Convert the generic StaggeredAnimation to a BoxedStaggeredAnimation
        // which implements Animation<Value = T> and can be stored in a Box<dyn Animation>
        let boxed_staggered = staggered;
        self.animation = Some(Box::new(boxed_staggered));
        self.is_active = true;
    }
}

/// A reactive motion value that can be animated
///
/// This is the main type that users interact with when creating animations.
/// It provides a fluent API for configuring and starting different animation types.
pub struct MotionValue<T: Animatable> {
    /// The underlying animation engine
    engine: Signal<AnimationEngine<T>>,
    /// Callback queue for animation completion
    callbacks: Arc<RwLock<Vec<Box<dyn FnOnce() + Send>>>>,
}

impl<T: Animatable> Clone for MotionValue<T> {
    fn clone(&self) -> Self {
        Self {
            engine: self.engine,
            callbacks: self.callbacks.clone(),
        }
    }
}

impl<T: Animatable> MotionValue<T> {
    /// Create a new motion value from an animation engine signal
    pub fn new(engine: Signal<AnimationEngine<T>>) -> Self {
        Self {
            engine,
            callbacks: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Get the current value
    pub fn get(&self) -> T {
        self.engine.read().get()
    }

    /// Set the value directly (without animation)
    pub fn set(&mut self, value: T) {
        self.engine.write().set(value);
    }

    /// Create a spring animation builder
    pub fn spring(&self) -> SpringBuilder<T> {
        SpringBuilder::new(self.clone())
    }

    /// Create a tween animation builder
    pub fn tween(&self) -> TweenBuilder<T> {
        TweenBuilder::new(self.clone())
    }

    /// Create a keyframe animation builder
    pub fn keyframes(&self) -> KeyframeBuilder<T> {
        KeyframeBuilder::new(self.clone())
    }

    /// Directly animate to a value with default spring physics
    pub fn animate_to(&mut self, target: T) -> &Self {
        self.engine.write().spring_to(target, Spring::default());
        self
    }

    /// Stop any running animation
    pub fn stop(&mut self) -> &Self {
        self.engine.write().stop();
        self
    }

    /// Check if there's an active animation
    pub fn is_animating(&self) -> bool {
        self.engine.read().is_active()
    }

    /// Add a completion callback
    fn add_completion_callback<F: FnOnce() + Send + 'static>(&self, callback: F) {
        if let Ok(mut callbacks) = self.callbacks.write() {
            callbacks.push(Box::new(callback));
        }
    }
}

/// Builder for spring animations
pub struct SpringBuilder<T: Animatable> {
    motion: MotionValue<T>,
    spring: Spring,
    completion_callback: Option<Box<dyn FnOnce() + Send>>,
}

impl<T: Animatable> SpringBuilder<T> {
    /// Create a new spring builder
    fn new(motion: MotionValue<T>) -> Self {
        Self {
            motion,
            spring: Spring::default(),
            completion_callback: None,
        }
    }

    /// Set spring stiffness
    pub fn stiffness(mut self, stiffness: f32) -> Self {
        self.spring.stiffness = stiffness;
        self
    }

    /// Set spring damping
    pub fn damping(mut self, damping: f32) -> Self {
        self.spring.damping = damping;
        self
    }

    /// Set spring mass
    pub fn mass(mut self, mass: f32) -> Self {
        self.spring.mass = mass;
        self
    }

    /// Set initial velocity
    pub fn velocity(mut self, velocity: T) -> Self {
        self.spring.initial_velocity = Some(velocity.magnitude());
        self
    }

    /// Add completion callback
    pub fn on_complete<F: FnOnce() + Send + 'static>(mut self, callback: F) -> Self {
        self.completion_callback = Some(Box::new(callback));
        self
    }

    /// Start animation to target value
    pub fn animate_to(mut self, target: T) -> MotionValue<T> {
        // Apply the completion callback if provided
        if let Some(callback) = self.completion_callback {
            self.motion.add_completion_callback(callback);
        }

        self.motion.engine.write().spring_to(target, self.spring);
        self.motion
    }
}

/// Builder for tween animations
pub struct TweenBuilder<T: Animatable> {
    motion: MotionValue<T>,
    tween: Tween,
    completion_callback: Option<Box<dyn FnOnce() + Send>>,
}

impl<T: Animatable> TweenBuilder<T> {
    /// Create a new tween builder
    fn new(motion: MotionValue<T>) -> Self {
        Self {
            motion,
            tween: Tween::default(),
            completion_callback: None,
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

    /// Start animation to target value
    pub fn animate_to(mut self, target: T) -> MotionValue<T> {
        // Apply the completion callback if provided
        if let Some(callback) = self.completion_callback {
            self.motion.add_completion_callback(callback);
        }

        self.motion.engine.write().tween_to(target, self.tween);
        self.motion
    }
}

/// Builder for keyframe animations
pub struct KeyframeBuilder<T: Animatable> {
    motion: MotionValue<T>,
    keyframes: Vec<(f32, T, Option<fn(f32, f32, f32, f32) -> f32>)>,
    duration: Duration,
    completion_callback: Option<Box<dyn FnOnce() + Send>>,
}

impl<T: Animatable> KeyframeBuilder<T> {
    /// Create a new keyframe builder
    fn new(motion: MotionValue<T>) -> Self {
        Self {
            motion,
            keyframes: Vec::new(),
            duration: Duration::from_millis(300),
            completion_callback: None,
        }
    }

    /// Add a keyframe at position (0.0 to 1.0)
    pub fn keyframe(mut self, position: f32, value: T) -> Self {
        self.keyframes.push((position.clamp(0.0, 1.0), value, None));
        self
    }

    /// Add a keyframe with custom easing
    pub fn keyframe_with_easing(
        mut self,
        position: f32,
        value: T,
        easing: fn(f32, f32, f32, f32) -> f32,
    ) -> Self {
        self.keyframes
            .push((position.clamp(0.0, 1.0), value, Some(easing)));
        self
    }

    /// Set animation duration
    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    /// Add completion callback
    pub fn on_complete<F: FnOnce() + Send + 'static>(mut self, callback: F) -> Self {
        self.completion_callback = Some(Box::new(callback));
        self
    }

    /// Start keyframe animation
    pub fn start(mut self) -> MotionValue<T> {
        // Create keyframe animation
        let mut keyframe_animation = crate::keyframe::KeyframeAnimation::new(self.duration);

        // Add keyframes
        for (position, value, easing) in self.keyframes {
            if let Some(easing) = easing {
                // Convert the easing function to the format expected by KeyframeAnimation
                keyframe_animation =
                    keyframe_animation.add_keyframe_with_easing(position, value, easing);
            } else {
                keyframe_animation = keyframe_animation.add_keyframe(position, value);
            }
        }

        // Apply the completion callback if provided
        if let Some(callback) = self.completion_callback {
            self.motion.add_completion_callback(callback);
        }

        // Start the animation
        self.motion
            .engine
            .write()
            .apply_keyframes(keyframe_animation);

        self.motion
    }
}
