use super::AnimationState;
use crate::Animatable;

/// Core trait for all animation types
///
/// This trait defines the interface that all animation implementations must provide.
pub trait Animation: Send + 'static {
    /// The type of value being animated
    type Value: Animatable;

    /// Update the animation with the given time delta in seconds
    ///
    /// Returns:
    /// - The current animation state (active or completed)
    /// - The current value
    /// - The current velocity
    fn update(&mut self, dt: f32) -> (AnimationState, Self::Value, Self::Value);

    /// Get the current value
    fn value(&self) -> Self::Value;

    /// Get the current velocity
    fn velocity(&self) -> Self::Value;

    /// Reset the animation to its initial state
    fn reset(&mut self);

    /// Is the animation in progress
    fn is_active(&self) -> bool;
}
