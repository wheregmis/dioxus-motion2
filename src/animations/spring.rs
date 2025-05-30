//! Spring-based animation physics
//!
//! Provides a physical spring model for smooth, natural-looking animations.
//! Based on Hooke's law with damping for realistic motion.

use dioxus::signals::Writable;

use crate::animation::{Animation, AnimationState, AnimationTiming, LoopMode};
use crate::{Animatable, MotionValue};

/// Spring animation with configurable physics
///
/// # Example
#[derive(Debug, Clone)]
pub struct Spring {
    /// Spring stiffness coefficient (default: 100.0)
    /// Controls how quickly the spring moves toward the target
    pub stiffness: f32,

    /// Damping coefficient (default: 10.0)
    /// Controls how quickly oscillations settle down
    pub damping: f32,

    /// Mass of the object (default: 1.0)
    /// Controls the inertia of the object
    pub mass: f32,

    /// Initial velocity (optional)
    /// Can be used to give the animation an initial push
    pub initial_velocity: Option<f32>,

    /// Animation timing parameters
    pub timing: AnimationTiming,
}

impl Default for Spring {
    fn default() -> Self {
        Self {
            stiffness: 100.0,
            damping: 10.0,
            mass: 1.0,
            initial_velocity: None,
            timing: AnimationTiming::default(),
        }
    }
}

impl Spring {
    /// Create a new spring with default parameters
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the spring stiffness
    pub fn stiffness(mut self, stiffness: f32) -> Self {
        self.stiffness = stiffness.max(0.1);
        self
    }

    /// Set the damping coefficient
    pub fn damping(mut self, damping: f32) -> Self {
        self.damping = damping.max(0.0);
        self
    }

    /// Set the mass
    pub fn mass(mut self, mass: f32) -> Self {
        self.mass = mass.max(0.1);
        self
    }

    /// Set the initial velocity
    pub fn initial_velocity(mut self, velocity: f32) -> Self {
        self.initial_velocity = Some(velocity);
        self
    }

    /// Create a spring animation with the current configuration
    pub fn create_animation<T: Animatable>(
        &self,
        initial: T,
        target: T,
        initial_velocity: T,
    ) -> SpringAnimation<T> {
        SpringAnimation {
            initial,
            current: initial,
            target,
            velocity: initial_velocity,
            spring: self.clone(),
            timing: AnimationTiming::default(),
            is_active: true,
        }
    }
}

/// Spring-based animation implementation
pub struct SpringAnimation<T: Animatable> {
    /// Initial value
    initial: T,
    /// Current value
    current: T,
    /// Target value
    target: T,
    /// Current velocity
    velocity: T,
    /// Spring configuration
    spring: Spring,
    /// Animation timing parameters
    timing: AnimationTiming,
    /// Whether the animation is active
    is_active: bool,
}

impl<T: Animatable> SpringAnimation<T> {
    /// Create a new spring animation
    pub fn new(initial: T, target: T, spring: Spring, timing: AnimationTiming) -> Self {
        let velocity = spring.initial_velocity.map_or_else(T::zero, |v| {
            // Create initial velocity in the direction of the target
            let direction = target.sub(&initial);
            let magnitude = direction.magnitude();
            if magnitude > T::epsilon() {
                direction.scale(v / magnitude)
            } else {
                T::zero()
            }
        });

        Self {
            initial,
            current: initial,
            target,
            velocity,
            spring,
            timing,
            is_active: true,
        }
    }

    /// Update the spring physics
    fn update_physics(&mut self, dt: f32) -> bool {
        // Cap dt to avoid numerical instability
        let dt = dt.min(0.064);

        // Calculate spring force
        let displacement = self.target.sub(&self.current);
        let spring_force = displacement.scale(self.spring.stiffness);

        // Calculate damping force
        let damping_force = self.velocity.scale(self.spring.damping);

        // Calculate acceleration (F = ma)
        let acceleration = spring_force
            .sub(&damping_force)
            .scale(1.0 / self.spring.mass);

        // Update velocity
        self.velocity = self.velocity.add(&acceleration.scale(dt));

        // Update position
        self.current = self.current.add(&self.velocity.scale(dt));

        // Check for completion with more lenient thresholds
        let velocity_magnitude = self.velocity.magnitude();
        let displacement_magnitude = displacement.magnitude();

        // Use a much larger epsilon for completion check
        let completion_epsilon = T::epsilon() * 1000.0;

        println!(
            "Spring physics update - Velocity: {}, Displacement: {}, Epsilon: {}",
            velocity_magnitude, displacement_magnitude, completion_epsilon
        );

        if velocity_magnitude < completion_epsilon && displacement_magnitude < completion_epsilon {
            println!("Spring animation completed - velocity and displacement below threshold");
            // Snap to target for precision
            self.current = self.target;
            false // Animation completed
        } else {
            true // Animation still active
        }
    }
}

impl<T: Animatable> Animation for SpringAnimation<T> {
    type Value = T;

    fn update(&mut self, dt: f32) -> (AnimationState, Self::Value, Self::Value) {
        if !self.is_active {
            return (AnimationState::Completed, self.current, T::zero());
        }

        // Handle delay
        if !self.timing.handle_delay(dt) {
            return (AnimationState::Active, self.current, T::zero());
        }

        // Update spring physics
        let still_active = self.update_physics(dt);

        if still_active {
            (AnimationState::Active, self.current, self.velocity)
        } else {
            // Handle loop completion
            if self.timing.handle_loop_completion() {
                println!("Spring animation loop completed, resetting for next loop");
                // Reset for next loop but maintain target
                let target = self.target;
                self.current = self.initial;
                self.target = target;
                self.velocity = T::zero();
                self.is_active = true; // Keep animation active for next loop
                println!("Spring animation reset for next loop");
                (AnimationState::Active, self.current, self.velocity)
            } else {
                println!("Spring animation completed");
                self.is_active = false;
                (AnimationState::Completed, self.current, T::zero())
            }
        }
    }

    fn value(&self) -> Self::Value {
        self.current
    }

    fn velocity(&self) -> Self::Value {
        self.velocity
    }

    fn reset(&mut self) {
        self.current = self.initial;
        self.velocity = T::zero();
        self.timing.current_loop = 0;
        self.timing.delay_elapsed = false;
        self.is_active = true;
    }

    fn is_active(&self) -> bool {
        self.is_active
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
    pub(crate) fn new(motion: MotionValue<T>) -> Self {
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

    /// Set the loop mode for the animation
    pub fn loop_mode(mut self, mode: LoopMode) -> Self {
        let mut timing = AnimationTiming::default();
        timing.loop_mode = mode;
        self.spring.timing = timing;
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
