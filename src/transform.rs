//! Transform module for 2D transformations
//!
//! Provides a Transform type that can be animated, supporting:
//! - Translation (x, y)
//! - Scale (scaleX, scaleY)
//! - Rotation
//! - Skew (skewX, skewY)
//!
//! Uses radians for rotation and supports smooth interpolation.

use crate::animatable::Animatable;
use std::f32::consts::PI;

/// Transform animation for position, scale, rotation, and skew
///
/// # Example
/// ```
/// use dioxus_motion2::Transform;
/// use std::f32::consts::PI;
///
/// let transform = Transform::new(100.0, 0.0, 1.5, 1.5, PI/4.0, 0.0, 0.0);
/// ```
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Transform {
    /// X translation component (px)
    pub x: f32,
    /// Y translation component (px)
    pub y: f32,
    /// X scale factor
    pub scale_x: f32,
    /// Y scale factor
    pub scale_y: f32,
    /// Rotation in radians
    pub rotation: f32,
    /// X skew in radians
    pub skew_x: f32,
    /// Y skew in radians
    pub skew_y: f32,
}

impl Transform {
    /// Creates a new transform with specified parameters
    pub fn new(
        x: f32,
        y: f32,
        scale_x: f32,
        scale_y: f32,
        rotation: f32,
        skew_x: f32,
        skew_y: f32,
    ) -> Self {
        Self {
            x,
            y,
            scale_x,
            scale_y,
            rotation,
            skew_x,
            skew_y,
        }
    }

    /// Creates an identity transform (no transformation)
    pub fn identity() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            rotation: 0.0,
            skew_x: 0.0,
            skew_y: 0.0,
        }
    }

    /// Creates a translation transform
    pub fn translate(x: f32, y: f32) -> Self {
        let mut result = Self::identity();
        result.x = x;
        result.y = y;
        result
    }

    /// Creates a scaling transform
    pub fn scale(scale_x: f32, scale_y: f32) -> Self {
        let mut result = Self::identity();
        result.scale_x = scale_x;
        result.scale_y = scale_y;
        result
    }

    /// Creates a uniform scaling transform
    pub fn scale_uniform(scale: f32) -> Self {
        Self::scale(scale, scale)
    }

    /// Creates a rotation transform
    pub fn rotate(angle_radians: f32) -> Self {
        let mut result = Self::identity();
        result.rotation = angle_radians;
        result
    }

    /// Creates a rotation transform using degrees
    pub fn rotate_degrees(angle_degrees: f32) -> Self {
        Self::rotate(angle_degrees * PI / 180.0)
    }

    /// Creates a skew transform
    pub fn skew(skew_x: f32, skew_y: f32) -> Self {
        let mut result = Self::identity();
        result.skew_x = skew_x;
        result.skew_y = skew_y;
        result
    }

    /// Converts the transform to a CSS transform string
    pub fn to_css_string(&self) -> String {
        let mut transforms = Vec::new();

        // Only add non-identity transformations
        if self.x != 0.0 || self.y != 0.0 {
            transforms.push(format!("translate({}px, {}px)", self.x, self.y));
        }

        if self.rotation != 0.0 {
            transforms.push(format!("rotate({}rad)", self.rotation));
        }

        if self.scale_x != 1.0 || self.scale_y != 1.0 {
            if self.scale_x == self.scale_y {
                transforms.push(format!("scale({})", self.scale_x));
            } else {
                transforms.push(format!("scale({}, {})", self.scale_x, self.scale_y));
            }
        }

        if self.skew_x != 0.0 {
            transforms.push(format!("skewX({}rad)", self.skew_x));
        }

        if self.skew_y != 0.0 {
            transforms.push(format!("skewY({}rad)", self.skew_y));
        }

        if transforms.is_empty() {
            "none".to_string()
        } else {
            transforms.join(" ")
        }
    }

    /// Combines this transform with another one (this * other)
    pub fn combine(&self, other: &Self) -> Self {
        // This is a simplified combination that doesn't properly handle all transformations,
        // but it's sufficient for most animations
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            scale_x: self.scale_x * other.scale_x,
            scale_y: self.scale_y * other.scale_y,
            rotation: self.rotation + other.rotation,
            skew_x: self.skew_x + other.skew_x,
            skew_y: self.skew_y + other.skew_y,
        }
    }
}

/// Implementation of Animatable for Transform
impl Animatable for Transform {
    /// Creates a zero transform (all components 0)
    fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0)
    }

    /// Minimum meaningful difference between transforms
    fn epsilon() -> f32 {
        0.001
    }

    /// Calculates the magnitude of the transform
    fn magnitude(&self) -> f32 {
        (self.x * self.x
            + self.y * self.y
            + (self.scale_x - 1.0) * (self.scale_x - 1.0)
            + (self.scale_y - 1.0) * (self.scale_y - 1.0)
            + self.rotation * self.rotation
            + self.skew_x * self.skew_x
            + self.skew_y * self.skew_y)
            .sqrt()
    }

    /// Scales all transform components by a factor
    fn scale(&self, factor: f32) -> Self {
        // Special handling for scale - we scale the delta from identity, not the value itself
        let scale_x_delta = self.scale_x - 1.0;
        let scale_y_delta = self.scale_y - 1.0;

        Self {
            x: self.x * factor,
            y: self.y * factor,
            scale_x: 1.0 + scale_x_delta * factor,
            scale_y: 1.0 + scale_y_delta * factor,
            rotation: self.rotation * factor,
            skew_x: self.skew_x * factor,
            skew_y: self.skew_y * factor,
        }
    }

    /// Adds two transforms component-wise
    fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            scale_x: self.scale_x + other.scale_x - 1.0, // Adjust to prevent double-scaling
            scale_y: self.scale_y + other.scale_y - 1.0, // Adjust to prevent double-scaling
            rotation: self.rotation + other.rotation,
            skew_x: self.skew_x + other.skew_x,
            skew_y: self.skew_y + other.skew_y,
        }
    }

    /// Subtracts two transforms component-wise
    fn sub(&self, other: &Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            scale_x: self.scale_x - other.scale_x + 1.0, // Adjust to prevent double-scaling
            scale_y: self.scale_y - other.scale_y + 1.0, // Adjust to prevent double-scaling
            rotation: self.rotation - other.rotation,
            skew_x: self.skew_x - other.skew_x,
            skew_y: self.skew_y - other.skew_y,
        }
    }

    /// Interpolates between two transforms
    /// Handles rotation specially to ensure shortest path
    fn interpolate(&self, target: &Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);

        // Handle rotation specially to ensure shortest path
        let mut rotation_diff = target.rotation - self.rotation;

        // Ensure we go the shortest way around the circle
        if rotation_diff > PI {
            rotation_diff -= 2.0 * PI;
        } else if rotation_diff < -PI {
            rotation_diff += 2.0 * PI;
        }

        Self {
            x: self.x + (target.x - self.x) * t,
            y: self.y + (target.y - self.y) * t,
            scale_x: self.scale_x + (target.scale_x - self.scale_x) * t,
            scale_y: self.scale_y + (target.scale_y - self.scale_y) * t,
            rotation: self.rotation + rotation_diff * t,
            skew_x: self.skew_x + (target.skew_x - self.skew_x) * t,
            skew_y: self.skew_y + (target.skew_y - self.skew_y) * t,
        }
    }
}
