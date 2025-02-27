//! Defines the Animatable trait for types that can be animated
//!
//! The Animatable trait provides the core operations needed for interpolation
//! and physics-based animations.

/// A trait for types that can be animated
///
/// Types implementing this trait can be used with both tween and spring animations.
/// The trait provides basic mathematical operations needed for interpolation and
/// physics calculations.
pub trait Animatable: Copy + Send + Sync + 'static {
    /// Creates a zero value for the type
    fn zero() -> Self;

    /// Returns the smallest meaningful difference between values
    /// Used for determining when an animation has completed
    fn epsilon() -> f32;

    /// Calculates the magnitude/length of the value
    /// Used for spring physics and completion detection
    fn magnitude(&self) -> f32;

    /// Scales the value by a factor
    /// Used for velocity and acceleration calculations
    fn scale(&self, factor: f32) -> Self;

    /// Adds another value
    /// Used for position updates in physics simulations
    fn add(&self, other: &Self) -> Self;

    /// Subtracts another value
    /// Used for calculating deltas between positions
    fn sub(&self, other: &Self) -> Self;

    /// Interpolates between self and target using t (0.0 to 1.0)
    /// Used for tween animations and keyframe interpolation
    fn interpolate(&self, target: &Self, t: f32) -> Self;

    /// Creates a value from a normalized parameter (0.0 to 1.0)
    /// Used for advanced interpolation and parameterization
    fn from_parameter(parameter: f32) -> Self {
        Self::zero().scale(parameter)
    }

    /// Equality check with epsilon tolerance
    fn approx_eq(&self, other: &Self) -> bool {
        self.sub(other).magnitude() < Self::epsilon()
    }
}

/// Implementation of Animatable for primitive f32
impl Animatable for f32 {
    fn zero() -> Self {
        0.0
    }

    fn epsilon() -> f32 {
        0.001
    }

    fn magnitude(&self) -> f32 {
        self.abs()
    }

    fn scale(&self, factor: f32) -> Self {
        self * factor
    }

    fn add(&self, other: &Self) -> Self {
        self + other
    }

    fn sub(&self, other: &Self) -> Self {
        self - other
    }

    fn interpolate(&self, target: &Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        self * (1.0 - t) + target * t
    }

    fn from_parameter(parameter: f32) -> Self {
        parameter
    }
}

/// Implementation of Animatable for primitive f64
impl Animatable for f64 {
    fn zero() -> Self {
        0.0
    }

    fn epsilon() -> f32 {
        0.001
    }

    fn magnitude(&self) -> f32 {
        self.abs() as f32
    }

    fn scale(&self, factor: f32) -> Self {
        self * factor as f64
    }

    fn add(&self, other: &Self) -> Self {
        self + other
    }

    fn sub(&self, other: &Self) -> Self {
        self - other
    }

    fn interpolate(&self, target: &Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0) as f64;
        self * (1.0 - t) + target * t
    }

    fn from_parameter(parameter: f32) -> Self {
        parameter as f64
    }
}

/// Implementation of Animatable for primitive i32
impl Animatable for i32 {
    fn zero() -> Self {
        0
    }

    fn epsilon() -> f32 {
        0.5
    }

    fn magnitude(&self) -> f32 {
        self.abs() as f32
    }

    fn scale(&self, factor: f32) -> Self {
        (*self as f32 * factor) as i32
    }

    fn add(&self, other: &Self) -> Self {
        self + other
    }

    fn sub(&self, other: &Self) -> Self {
        self - other
    }

    fn interpolate(&self, target: &Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        (*self as f32 * (1.0 - t) + *target as f32 * t) as i32
    }
}
