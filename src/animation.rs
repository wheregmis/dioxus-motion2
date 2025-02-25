//! Core animation traits and types
//!
//! This module provides the fundamental abstraction for all animation types.

use crate::animatable::Animatable;
use std::sync::{Arc, Mutex};

/// State of an animation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationState {
    /// Animation is still running
    Active,
    /// Animation has completed
    Completed,
}

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

/// A callback that is executed when an animation completes
pub type CompletionCallback = Arc<Mutex<Option<Box<dyn FnOnce() + Send>>>>;

/// Animation loop mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoopMode {
    /// No looping - animation plays once
    None,
    /// Animation repeats indefinitely
    Infinite,
    /// Animation repeats a specific number of times
    Count(u32),
}

impl Default for LoopMode {
    fn default() -> Self {
        Self::None
    }
}

/// Animation playback direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaybackDirection {
    /// Forward playback (normal)
    Forward,
    /// Reverse playback
    Reverse,
    /// Alternate between forward and reverse
    Alternate,
    /// Alternate, starting with reverse
    AlternateReverse,
}

impl Default for PlaybackDirection {
    fn default() -> Self {
        Self::Forward
    }
}

/// Animation timing options
#[derive(Clone)]
pub struct AnimationTiming {
    /// Loop mode
    pub loop_mode: LoopMode,
    /// Playback direction
    pub direction: PlaybackDirection,
    /// Delay before starting
    pub delay: instant::Duration,
    /// Current loop count
    pub current_loop: u32,
    /// Whether delay has elapsed
    pub delay_elapsed: bool,
    /// Completion callback
    pub on_complete: Option<Arc<Mutex<dyn FnMut() + Send>>>,
}

impl Default for AnimationTiming {
    fn default() -> Self {
        Self {
            loop_mode: LoopMode::None,
            direction: PlaybackDirection::Forward,
            delay: instant::Duration::ZERO,
            current_loop: 0,
            delay_elapsed: false,
            on_complete: None,
        }
    }
}

impl AnimationTiming {
    /// Create a new animation timing configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the loop mode
    pub fn with_loop_mode(mut self, loop_mode: LoopMode) -> Self {
        self.loop_mode = loop_mode;
        self
    }

    /// Set the playback direction
    pub fn with_direction(mut self, direction: PlaybackDirection) -> Self {
        self.direction = direction;
        self
    }

    /// Set the delay before starting
    pub fn with_delay(mut self, delay: instant::Duration) -> Self {
        self.delay = delay;
        self
    }

    pub fn with_on_complete<F>(mut self, f: F) -> Self
    where
        F: FnMut() + Send + 'static,
    {
        self.on_complete = Some(Arc::new(Mutex::new(f)));
        self
    }
    /// Handle the delay
    pub fn handle_delay(&mut self, dt: f32) -> bool {
        if self.delay_elapsed {
            return true;
        }

        if self.delay.is_zero() {
            self.delay_elapsed = true;
            return true;
        }

        let dt_duration = instant::Duration::from_secs_f32(dt);
        if dt_duration >= self.delay {
            self.delay = instant::Duration::ZERO;
            self.delay_elapsed = true;
            return true;
        } else {
            self.delay -= dt_duration;
            return false;
        }
    }

    /// Handle loop completion
    pub fn handle_loop_completion(&mut self) -> bool {
        match self.loop_mode {
            LoopMode::None => {
                // Execute completion callback if provided
                if let Some(on_complete) = &self.on_complete {
                    if let Ok(mut callback) = on_complete.lock() {
                        callback();
                    }
                }
                false
            }
            LoopMode::Infinite => {
                self.current_loop += 1;
                // Toggle direction if alternating
                if self.direction == PlaybackDirection::Alternate
                    || self.direction == PlaybackDirection::AlternateReverse
                {
                    self.direction = match self.direction {
                        PlaybackDirection::Alternate => PlaybackDirection::AlternateReverse,
                        PlaybackDirection::AlternateReverse => PlaybackDirection::Alternate,
                        _ => self.direction,
                    };
                }
                true
            }
            LoopMode::Count(count) => {
                self.current_loop += 1;
                if self.current_loop >= count {
                    // Execute completion callback if provided
                    if let Some(on_complete) = &self.on_complete {
                        if let Ok(mut callback) = on_complete.lock() {
                            callback();
                        }
                    }
                    false
                } else {
                    // Toggle direction if alternating
                    if self.direction == PlaybackDirection::Alternate
                        || self.direction == PlaybackDirection::AlternateReverse
                    {
                        self.direction = match self.direction {
                            PlaybackDirection::Alternate => PlaybackDirection::AlternateReverse,
                            PlaybackDirection::AlternateReverse => PlaybackDirection::Alternate,
                            _ => self.direction,
                        };
                    }
                    true
                }
            }
        }
    }

    /// Get whether animation should play in reverse for current loop
    pub fn is_reverse(&self) -> bool {
        match self.direction {
            PlaybackDirection::Forward => false,
            PlaybackDirection::Reverse => true,
            PlaybackDirection::Alternate => self.current_loop % 2 == 1,
            PlaybackDirection::AlternateReverse => self.current_loop % 2 == 0,
        }
    }
}
