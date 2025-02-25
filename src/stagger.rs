//! Staggered animations for sequential effects
//!
//! Provides support for running multiple similar animations with staggered starts,
//! creating a cascade or wave effect.

use instant::Duration;
use std::sync::{Arc, Mutex};

use crate::animatable::Animatable;
use crate::animation::{Animation, AnimationState};

/// Type alias for the FnMut callback
pub type MutCallback = Arc<Mutex<dyn FnMut() + Send>>;

/// A staggered animation item
pub struct StaggerItem<T: Animatable, A: Animation<Value = T>> {
    /// The animation
    animation: A,
    /// Delay before this animation starts
    delay: Duration,
    /// Elapsed delay time
    elapsed_delay: Duration,
    /// Whether this animation has started
    started: bool,
    /// Key for this animation
    key: usize,
}

/// A staggered set of animations that start at different times
pub struct StaggeredAnimation<T: Animatable, A: Animation<Value = T>> {
    /// Items to animate
    items: Vec<StaggerItem<T, A>>,
    /// Base delay between items
    base_delay: Duration,
    /// Whether all animations have completed
    all_completed: bool,
    /// Current aggregated value
    current: T,
    /// Completion callback
    pub on_complete: Option<MutCallback>,
    /// Whether the staggered animation is active
    is_active: bool,
}

impl<T: Animatable, A: Animation<Value = T>> Default for StaggeredAnimation<T, A> {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            base_delay: Duration::from_millis(50),
            all_completed: false,
            current: T::zero(),
            on_complete: None,
            is_active: false,
        }
    }
}

impl<T: Animatable, A: Animation<Value = T>> StaggeredAnimation<T, A> {
    /// Create a new staggered animation
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an animation with a custom delay
    pub fn add_with_delay(mut self, animation: A, delay: Duration, key: usize) -> Self {
        self.items.push(StaggerItem {
            animation,
            delay,
            elapsed_delay: Duration::ZERO,
            started: false,
            key,
        });

        self.is_active = true;
        self
    }

    /// Add an animation with a calculated delay
    pub fn add(self, animation: A, key: usize) -> Self {
        let delay = self.base_delay.mul_f32(key as f32);
        self.add_with_delay(animation, delay, key)
    }

    /// Set the base delay between animations
    pub fn delay_between(mut self, delay: Duration) -> Self {
        self.base_delay = delay;
        self
    }

    /// Set a completion callback
    pub fn with_on_complete<F>(mut self, f: F) -> Self
    where
        F: FnMut() + Send + 'static,
    {
        self.on_complete = Some(Arc::new(Mutex::new(f)));
        self
    }

    /// Set a completion callback (alternative method name for consistency)
    pub fn on_complete<F>(self, f: F) -> Self
    where
        F: FnMut() + Send + 'static,
    {
        self.with_on_complete(f)
    }

    /// Start all animations
    pub fn start(mut self) -> Self {
        self.is_active = !self.items.is_empty();
        self
    }
}

impl<T: Animatable, A: Animation<Value = T>> Animation for StaggeredAnimation<T, A> {
    type Value = T;

    fn update(&mut self, dt: f32) -> (AnimationState, Self::Value, Self::Value) {
        if !self.is_active {
            return (AnimationState::Completed, self.current, T::zero());
        }

        if self.all_completed {
            return (AnimationState::Completed, self.current, T::zero());
        }

        let dt_duration = Duration::from_secs_f32(dt);
        let mut all_completed = true;

        // Update and check each animation
        for item in &mut self.items {
            if !item.started {
                // Update delay time
                item.elapsed_delay += dt_duration;

                // Check if delay elapsed
                if item.elapsed_delay >= item.delay {
                    item.started = true;
                } else {
                    // Item still waiting to start
                    all_completed = false;
                    continue;
                }
            }

            // Update the animation
            let (state, _value, _) = item.animation.update(dt);

            if state == AnimationState::Active {
                all_completed = false;
            }
        }

        // If all animations completed
        if all_completed && !self.all_completed {
            self.all_completed = true;

            // Execute completion callback
            if let Some(on_complete) = &self.on_complete {
                if let Ok(mut callback) = on_complete.lock() {
                    callback();
                }
            }

            return (AnimationState::Completed, self.current, T::zero());
        }

        // Compute aggregated value (last active animation's value)
        for item in self.items.iter().rev() {
            if item.started {
                self.current = item.animation.value();
                break;
            }
        }

        (
            if all_completed {
                AnimationState::Completed
            } else {
                AnimationState::Active
            },
            self.current,
            T::zero(),
        )
    }

    fn value(&self) -> Self::Value {
        self.current
    }

    fn velocity(&self) -> Self::Value {
        T::zero() // Velocity is not well-defined for staggered animations
    }

    fn reset(&mut self) {
        // Reset all items
        for item in &mut self.items {
            item.animation.reset();
            item.elapsed_delay = Duration::ZERO;
            item.started = false;
        }

        self.all_completed = false;
        self.is_active = !self.items.is_empty();
    }

    fn is_active(&self) -> bool {
        self.is_active && !self.all_completed
    }
}

/// Helper function to create a new staggered animation
pub fn stagger<T: Animatable, A: Animation<Value = T>>() -> StaggeredAnimation<T, A> {
    StaggeredAnimation::new()
}

// Create a type-erased version of StaggeredAnimation that can be stored in
// a Box<dyn Animation> by the AnimationEngine
pub struct BoxedStaggeredAnimation<T: Animatable> {
    /// The inner value being animated
    current_value: T,
    /// Whether the animation is complete
    is_complete: bool,
    /// Completion callback from the original staggered animation
    on_complete: Option<MutCallback>,
}

impl<T: Animatable> Animation for BoxedStaggeredAnimation<T> {
    type Value = T;

    fn update(&mut self, _dt: f32) -> (AnimationState, Self::Value, Self::Value) {
        // This is a placeholder - the real implementation would delegate to the wrapped staggered animation
        if self.is_complete {
            (AnimationState::Completed, self.current_value, T::zero())
        } else {
            self.is_complete = true;

            // Execute completion callback
            if let Some(on_complete) = &self.on_complete {
                if let Ok(mut callback) = on_complete.lock() {
                    callback();
                }
            }

            (AnimationState::Active, self.current_value, T::zero())
        }
    }

    fn value(&self) -> Self::Value {
        self.current_value
    }

    fn velocity(&self) -> Self::Value {
        T::zero()
    }

    fn reset(&mut self) {
        self.is_complete = false;
    }

    fn is_active(&self) -> bool {
        !self.is_complete
    }
}
