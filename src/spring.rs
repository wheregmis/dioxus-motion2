//! Spring-based animation physics
//!
//! Provides a physical spring model for smooth, natural-looking animations.
//! Based on Hooke's law with damping for realistic motion.

use crate::animatable::Animatable;
use crate::animation::{Animation, AnimationState, AnimationTiming};

/// Spring animation with configurable physics
///
/// # Example
/// ```
/// use dioxus_motion2::Spring;
///
/// let spring = Spring::new()
///     .stiffness(180.0)
///     .damping(20.0)
///     .mass(1.0);
/// ```
#[derive(Debug, Clone, Copy)]
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
}

impl Default for Spring {
    fn default() -> Self {
        Self {
            stiffness: 100.0,
            damping: 10.0,
            mass: 1.0,
            initial_velocity: None,
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

    /// Configure for a bouncy spring
    pub fn bouncy() -> Self {
        Self {
            stiffness: 120.0,
            damping: 8.0,
            mass: 1.0,
            initial_velocity: None,
        }
    }

    /// Configure for a smooth, non-bouncy spring
    pub fn smooth() -> Self {
        Self {
            stiffness: 80.0,
            damping: 20.0,
            mass: 1.0,
            initial_velocity: None,
        }
    }

    /// Configure for a stiff, quick spring
    pub fn stiff() -> Self {
        Self {
            stiffness: 210.0,
            damping: 20.0,
            mass: 1.0,
            initial_velocity: None,
        }
    }

    /// Configure for molasses-like, slow movement
    pub fn molasses() -> Self {
        Self {
            stiffness: 30.0,
            damping: 26.0,
            mass: 3.0,
            initial_velocity: None,
        }
    }

    /// Configure a critically damped spring that doesn't oscillate
    pub fn critically_damped() -> Self {
        Self {
            stiffness: 100.0,
            damping: 2.0_f32 * (100.0_f32 * 1.0_f32).sqrt(), // 2 * sqrt(k * m)
            mass: 1.0,
            initial_velocity: None,
        }
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
            spring: *self,
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

        // Check for completion
        if self.velocity.magnitude() < T::epsilon() && displacement.magnitude() < T::epsilon() {
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
                // Reset for next loop
                self.current = self.initial;
                self.velocity = T::zero();
                (AnimationState::Active, self.current, self.velocity)
            } else {
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
