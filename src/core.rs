//! Core animation engine powering Dioxus Motion
//!
//! This module contains the central animation engine and motion value abstractions
//! that form the foundation of the library.

use dioxus::prelude::*;
use std::sync::{Arc, Mutex};

use crate::Animatable;
use crate::MotionTime;
use crate::animation::{Animation, AnimationState};
use crate::animations::keyframe::KeyframeAnimation;
use crate::animations::spring::Spring;
use crate::animations::spring::SpringBuilder;
use crate::animations::tween::Tween;
use crate::animations::tween::TweenBuilder;
use crate::platform::TimeProvider;
use crate::platform::request_animation_frame;
use crate::prelude::sequence::AnimationSequence;
use crate::prelude::sequence::SequenceBuilder;

use tokio_with_wasm::alias as tokio;

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

    /// Add a completion callback
    pub fn add_completion_callback<F: FnOnce() + Send + 'static>(&mut self, callback: F) {
        if let Ok(mut callbacks) = self.callbacks.lock() {
            callbacks.push(Box::new(callback));
        }
    }

    /// Apply an animation sequence
    pub fn apply_sequence(&mut self, sequence: AnimationSequence<T>) {
        self.animation = Some(Box::new(sequence));
        self.is_active = true;
    }

    pub async fn run_animation_loop(&mut self) {
        let mut last_frame = MotionTime::now();

        loop {
            request_animation_frame().await;

            let now = MotionTime::now();
            let dt = now.duration_since(last_frame).as_secs_f32();

            if dt > 0.032 {
                tokio::task::yield_now().await;
            }

            // Update animation state
            if let Some(animation) = &mut self.animation {
                let (state, value, velocity) = animation.update(dt);
                self.current = value;
                self.velocity = velocity;

                match state {
                    AnimationState::Active => {
                        // Continue animation
                    }
                    AnimationState::Completed => {
                        self.complete_animation();
                    }
                }
            }

            last_frame = now;
        }
    }

    fn complete_animation(&mut self) {
        self.is_active = false;
        self.animation = None;

        // Process callbacks
        if let Ok(mut callbacks) = self.callbacks.lock() {
            // Store callbacks before processing

            // Process callbacks immediately for web
            #[cfg(feature = "web")]
            {
                let callbacks_to_process = std::mem::take(&mut *callbacks);
                for callback in callbacks_to_process {
                    callback();
                }
            }

            #[cfg(not(feature = "web"))]
            {
                let mut callbacks_to_process = std::mem::take(&mut *callbacks);
                while !callbacks_to_process.is_empty() {
                    let chunk: Vec<_> = callbacks_to_process
                        .drain(..callbacks_to_process.len().min(5))
                        .collect();

                    tokio::spawn(async move {
                        for callback in chunk {
                            callback();
                            tokio::time::sleep(Duration::from_millis(1)).await;
                        }
                    });
                }
            }
        }
    }
}

/// A reactive motion value that can be animated
///
/// This is the main type that users interact with when creating animations.
/// It provides a fluent API for configuring and starting different animation types.

#[derive(Clone, Copy)]
pub struct MotionValue<T: Animatable> {
    /// The underlying animation engine
    pub(crate) engine: Signal<AnimationEngine<T>>,
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

    /// Start a keyframe animation
    pub fn animate_keyframes(&mut self, keyframes: KeyframeAnimation<T>) -> &Self {
        self.engine.write().apply_keyframes(keyframes);
        self
    }

    /// Sequence animation builder
    /// Create a sequence animation builder
    pub fn sequence(&self) -> SequenceBuilder<T> {
        SequenceBuilder::new(*self)
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
}
