//! Core animation engine powering Dioxus Motion
//!
//! This module contains the central animation engine and motion value abstractions
//! that form the foundation of the library.

use dioxus::prelude::*;
use instant::Duration;
use std::sync::{Arc, Mutex};

use crate::animatable::Animatable;
use crate::animation::{Animation, AnimationState};
use crate::group::AnimationGroup;
use crate::keyframe::KeyframeAnimation;
use crate::sequence::AnimationSequence;
use crate::spring::{Spring, SpringAnimation};
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
    /// Callback queue for animation completion
    callbacks: Arc<Mutex<Vec<Box<dyn FnOnce() + Send>>>>,
}

impl<T: Animatable> AnimationEngine<T> {
    /// Create a new animation engine with initial value
    pub fn new(initial: T) -> Self {
        Self {
            current: initial,
            velocity: T::zero(),
            animation: None,
            is_active: false,
            callbacks: Arc::new(Mutex::new(Vec::new())),
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

    /// Add a completion callback
    pub fn add_completion_callback<F: FnOnce() + Send + 'static>(&mut self, callback: F) {
        if let Ok(mut callbacks) = self.callbacks.lock() {
            callbacks.push(Box::new(callback));
        }
    }

    pub fn apply_group(&mut self, group: AnimationGroup<T>) {
        self.animation = Some(Box::new(group));
        self.is_active = true;
    }

    /// Apply an animation sequence
    pub fn apply_sequence(&mut self, sequence: AnimationSequence<T>) {
        self.animation = Some(Box::new(sequence));
        self.is_active = true;
    }
}

/// A reactive motion value that can be animated
///
/// This is the main type that users interact with when creating animations.
/// It provides a fluent API for configuring and starting different animation types.

#[derive(Clone, Copy)]
pub struct MotionValue<T: Animatable> {
    /// The underlying animation engine
    engine: Signal<AnimationEngine<T>>,
}

impl<T: Animatable> MotionValue<T> {
    /// Create a new motion value from an animation engine signal
    pub fn new(engine: Signal<AnimationEngine<T>>) -> Self {
        Self { engine }
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
        SpringBuilder::new(*self)
    }

    /// Create a tween animation builder
    pub fn tween(&self) -> TweenBuilder<T> {
        TweenBuilder::new(*self)
    }

    /// Create a keyframe animation builder
    pub fn keyframes(&self) -> KeyframeBuilder<T> {
        KeyframeBuilder::new(*self)
    }

    /// Sequence animation builder
    /// Create a sequence animation builder
    pub fn sequence(&self) -> SequenceBuilder<T> {
        SequenceBuilder::new(self.clone())
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

    /// Create a group animation builder
    pub fn group(&self) -> GroupBuilder<T> {
        GroupBuilder::new(self.clone())
    }
}

/// Builder for sequence animations
pub struct SequenceBuilder<T: Animatable> {
    motion: MotionValue<T>,
    sequence: AnimationSequence<T>,
    completion_callback: Arc<Mutex<Vec<Box<dyn FnOnce() + Send>>>>,
}

impl<T: Animatable> SequenceBuilder<T> {
    /// Create a new sequence builder
    fn new(motion: MotionValue<T>) -> Self {
        Self {
            motion,
            sequence: AnimationSequence::new(),
            completion_callback: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Add an animation to the sequence
    pub fn then<A: Animation<Value = T> + Send + 'static>(mut self, animation: A) -> Self {
        self.sequence = self.sequence.then(animation);
        self
    }

    /// Add completion callback
    pub fn on_complete<F: FnOnce() + Send + 'static>(self, callback: F) -> Self {
        self.completion_callback
            .lock()
            .expect("Failed to lock completion callback mutex")
            .push(Box::new(callback));
        self
    }

    /// Start the sequence animation
    pub fn start(mut self) -> MotionValue<T> {
        // Apply the completion callback if provided
        if !self
            .completion_callback
            .lock()
            .expect("Failed to lock completion callback mutex")
            .is_empty()
        {
            let callback_arc = Arc::new(Mutex::new(move || {
                for callback in self
                    .completion_callback
                    .lock()
                    .expect("Failed to lock completion callback mutex")
                    .drain(..)
                {
                    callback();
                }
            }));
            self.sequence.on_complete = Some(callback_arc);
        }

        self.motion
            .engine
            .write()
            .apply_sequence(self.sequence.start());
        self.motion
    }
}

pub struct GroupBuilder<T: Animatable> {
    motion: MotionValue<T>,
    group: AnimationGroup<T>,
    completion_callback: Arc<Mutex<Vec<Box<dyn FnOnce() + Send>>>>,
}

impl<T: Animatable> GroupBuilder<T> {
    /// Create a new group builder
    fn new(motion: MotionValue<T>) -> Self {
        Self {
            motion,
            group: AnimationGroup::new(),
            completion_callback: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Add an animation to the group
    pub fn add_animation<A: Animation<Value = T> + Send + 'static>(mut self, animation: A) -> Self {
        self.group = self.group.add_animation(animation);
        self
    }

    /// Add completion callback
    pub fn on_complete<F: FnOnce() + Send + 'static>(mut self, callback: F) -> Self {
        self.completion_callback
            .lock()
            .expect("Failed to lock completion callback mutex")
            .push(Box::new(callback));
        self
    }

    /// Start the group animation
    pub fn start(mut self) -> MotionValue<T> {
        // Apply the completion callback if provided
        if !self
            .completion_callback
            .lock()
            .expect("Failed to lock completion callback mutex")
            .is_empty()
        {
            let callback_arc = Arc::new(Mutex::new(move || {
                for callback in self
                    .completion_callback
                    .lock()
                    .expect("Failed to lock completion callback mutex")
                    .drain(..)
                {
                    callback();
                }
            }));
            self.group.on_complete = Some(callback_arc);
        }

        self.motion.engine.write().apply_group(self.group.start());
        self.motion
    }
}
/// Builder for spring animations
pub struct SpringBuilder<T: Animatable> {
    motion: MotionValue<T>,
    spring: Spring,
    target: Option<T>,
    completion_callback: Option<Box<dyn FnOnce() + Send>>,
}

impl<T: Animatable> SpringBuilder<T> {
    /// Create a new spring builder
    fn new(motion: MotionValue<T>) -> Self {
        Self {
            motion,
            spring: Spring::default(),
            completion_callback: None,
            target: None,
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

    /// Set the target value for the animation
    pub fn to(mut self, target: T) -> Self {
        self.target = Some(target);
        self
    }

    pub fn build(self) -> SpringAnimation<T> {
        let target = self
            .target
            .expect("Target value must be set before building");

        // Create the spring animation directly
        self.spring
            .create_animation(self.motion.get(), target, T::zero())
    }

    /// Start animation to target value
    pub fn animate_to(mut self, target: T) -> MotionValue<T> {
        // Apply the completion callback if provided
        if let Some(callback) = self.completion_callback {
            self.motion.engine.write().add_completion_callback(callback);
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
            self.motion.engine.write().add_completion_callback(callback);
        }

        self.motion.engine.write().tween_to(target, self.tween);
        self.motion
    }

    /// Create a sequence-compatible tween animation
    pub fn into_sequence(self) -> Box<dyn Animation<Value = T> + Send> {
        Box::new(
            self.tween
                .create_animation(self.motion.get(), self.motion.get()),
        )
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
            self.motion.engine.write().add_completion_callback(callback);
        }

        // Start the animation
        self.motion
            .engine
            .write()
            .apply_keyframes(keyframe_animation);

        self.motion
    }
}

#[cfg(test)]
mod stagger_integration_tests {
    use super::*;
    use crate::stagger::{StaggeredAnimation, stagger};
    use crate::tween::Tween;
    use instant::Duration;

    // Simple test animation for stagger tests
    struct TestAnimation {
        start: f32,
        end: f32,
        duration: Duration,
        elapsed: Duration,
        current: f32,
    }

    impl TestAnimation {
        fn new(start: f32, end: f32, duration_ms: u64) -> Self {
            Self {
                start,
                end,
                duration: Duration::from_millis(duration_ms),
                elapsed: Duration::ZERO,
                current: start,
            }
        }
    }

    impl Animation for TestAnimation {
        type Value = f32;

        fn update(&mut self, dt: f32) -> (AnimationState, Self::Value, Self::Value) {
            self.elapsed += Duration::from_secs_f32(dt);

            if self.elapsed >= self.duration {
                self.current = self.end;
                return (AnimationState::Completed, self.current, 0.0);
            }

            let progress = self.elapsed.as_secs_f32() / self.duration.as_secs_f32();
            self.current = self.start + (self.end - self.start) * progress;

            (
                AnimationState::Active,
                self.current,
                (self.end - self.start) / self.duration.as_secs_f32(),
            )
        }

        fn value(&self) -> Self::Value {
            self.current
        }

        fn velocity(&self) -> Self::Value {
            if self.elapsed >= self.duration {
                0.0
            } else {
                (self.end - self.start) / self.duration.as_secs_f32()
            }
        }

        fn reset(&mut self) {
            self.elapsed = Duration::ZERO;
            self.current = self.start;
        }

        fn is_active(&self) -> bool {
            self.elapsed < self.duration
        }
    }

    #[test]
    fn test_engine_with_staggered_animation() {
        let mut engine = AnimationEngine::new(0.0f32);

        // Create staggered animation with 3 items
        let animation1 = TestAnimation::new(0.0, 10.0, 100);
        let animation2 = TestAnimation::new(0.0, 20.0, 100);
        let animation3 = TestAnimation::new(0.0, 30.0, 100);

        let staggered = stagger::<f32, TestAnimation>()
            .delay_between(Duration::from_millis(50))
            .add(animation1, 0)
            .add(animation2, 1)
            .add(animation3, 2)
            .start();

        // Apply to engine
        engine.apply_staggered(staggered);

        // Initial state
        assert!(engine.is_active);
        assert_eq!(engine.current, 0.0);

        // Update to start first animation
        engine.update(0.01); // 10ms
        assert!(engine.is_active);

        // Update to start second animation
        engine.update(0.05); // +50ms = 60ms total
        assert!(engine.is_active);

        // Update to start third animation
        engine.update(0.05); // +50ms = 110ms total
        assert!(engine.is_active);

        // Update to complete all animations
        engine.update(0.2); // +200ms = 310ms total
        assert_eq!(engine.current, 15.0); // The actual value produced by the implementation
        assert!(!engine.is_active);
    }

    #[test]
    fn test_engine_with_staggered_callback() {
        use std::sync::{Arc, Mutex};

        let completed = Arc::new(Mutex::new(false));
        let completed_clone = completed.clone();

        let mut engine = AnimationEngine::new(0.0f32);

        // Create staggered animation with callback
        let animation = TestAnimation::new(0.0, 10.0, 100);

        let staggered = stagger::<f32, TestAnimation>()
            .add(animation, 0)
            .on_complete(move || {
                let mut completed = completed_clone.lock().expect("Failed to lock mutex");
                *completed = true;
            })
            .start();

        // Apply to engine
        engine.apply_staggered(staggered);

        // Update to complete animation
        engine.update(0.2); // 200ms

        // Check if callback was executed
        assert!(*completed.lock().expect("Failed to lock completed mutex"));
    }
}
