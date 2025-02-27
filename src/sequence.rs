//! Animation sequences for chained animations
//!
//! Provides support for running a series of animations in sequence,
//! where each animation starts when the previous one completes.

use std::sync::{Arc, Mutex};

use tracing::{debug, warn};

use crate::animatable::Animatable;
use crate::animation::{Animation, AnimationState};

/// A step in an animation sequence
pub struct AnimationStep<T: Animatable> {
    /// The animation for this step
    animation: Box<dyn Animation<Value = T>>,
    /// Whether this step has started
    started: bool,
    /// Whether this step has completed
    completed: bool,
}

/// A sequence of animations that run one after another
pub struct AnimationSequence<T: Animatable> {
    /// Steps in the sequence
    steps: Vec<AnimationStep<T>>,
    /// Current step index
    current_step: usize,
    /// Current value
    current: T,
    /// Current velocity
    velocity: T,
    /// Whether the sequence is active
    is_active: bool,
    /// Completion callback
    pub on_complete: Option<Arc<Mutex<dyn FnMut() + Send>>>,
}

impl<T: Animatable> Default for AnimationSequence<T> {
    fn default() -> Self {
        Self {
            steps: Vec::new(),
            current_step: 0,
            current: T::zero(),
            velocity: T::zero(),
            is_active: false,
            on_complete: None,
        }
    }
}

impl<T: Animatable> AnimationSequence<T> {
    /// Create a new empty animation sequence
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an animation to the sequence
    pub fn then<A: Animation<Value = T> + Send + 'static>(mut self, animation: A) -> Self {
        self.steps.push(AnimationStep {
            animation: Box::new(animation),
            started: false,
            completed: false,
        });
        self
    }

    /// Set a completion callback
    pub fn on_complete<F: FnMut() + Send + 'static>(mut self, callback: F) -> Self {
        self.on_complete = Some(Arc::new(Mutex::new(callback)));
        self
    }

    /// Start the animation sequence
    pub fn start(mut self) -> Self {
        if !self.steps.is_empty() {
            self.is_active = true;
            self.current_step = 0;
            // Mark all steps as not completed
            for step in &mut self.steps {
                step.completed = false;
            }
        } else {
            warn!("Attempting to start empty animation sequence");
        }
        self
    }

    /// Build an animation for use with a MotionValue
    pub fn build(self) -> Box<dyn Animation<Value = T> + Send + 'static> {
        Box::new(self)
    }
}

impl<T: Animatable> Animation for AnimationSequence<T> {
    type Value = T;

    fn update(&mut self, dt: f32) -> (AnimationState, Self::Value, Self::Value) {
        if !self.is_active {
            debug!("Sequence not active");
            return (AnimationState::Completed, self.current, T::zero());
        }

        // Check if we have any steps
        if self.steps.is_empty() {
            debug!("Sequence has no steps");
            self.is_active = false;
            return (AnimationState::Completed, self.current, T::zero());
        }

        // Update current step
        let current_step = &mut self.steps[self.current_step];

        if !current_step.started {
            debug!("Starting step {}", self.current_step);
            current_step.started = true;
        }

        debug!("Updating step {} with dt: {}", self.current_step, dt);

        let (state, value, velocity) = current_step.animation.update(dt);

        self.current = value;
        self.velocity = velocity;

        // Check if current step completed
        if state == AnimationState::Completed {
            debug!("Step {} completed", self.current_step);
            current_step.completed = true;

            // Move to next step if available
            if self.current_step < self.steps.len() - 1 {
                debug!(
                    "Moving to next step: {} -> {}",
                    self.current_step,
                    self.current_step + 1
                );
                self.current_step += 1;
                self.steps[self.current_step].started = true;
                return (AnimationState::Active, self.current, self.velocity);
            } else {
                debug!("All steps completed");
                self.is_active = false;

                // Execute completion callback
                if let Some(on_complete) = &self.on_complete {
                    debug!("Executing completion callback");
                    if let Ok(mut callback) = on_complete.lock() {
                        callback();
                    }
                }
                return (AnimationState::Completed, self.current, T::zero());
            }
        }

        debug!("Step {} ", self.current_step);

        (AnimationState::Active, self.current, self.velocity)
    }

    fn value(&self) -> Self::Value {
        self.current
    }

    fn velocity(&self) -> Self::Value {
        self.velocity
    }

    fn reset(&mut self) {
        // Reset all steps
        for step in &mut self.steps {
            step.animation.reset();
            step.started = false;
            step.completed = false;
        }

        self.current_step = 0;

        // Start the first step if there is one
        if !self.steps.is_empty() {
            self.steps[0].started = true;
        }

        self.is_active = !self.steps.is_empty();
    }

    fn is_active(&self) -> bool {
        self.is_active
    }
}

/// Helper function to create a new animation sequence
pub fn sequence<T: Animatable>() -> AnimationSequence<T> {
    AnimationSequence::new()
}
