use crate::animation::timing::LoopMode;
use crate::animations::{spring::Spring, tween::Tween};
use instant::Duration;

/// Configuration for animations
pub struct AnimationConfig {
    /// The animation mode (spring or tween)
    pub mode: AnimationMode,
    /// Loop configuration
    pub loop_mode: Option<LoopMode>,
    /// Delay before animation starts
    pub delay: Option<Duration>,
    /// Callback to run on completion
    pub on_complete: Option<Box<dyn FnOnce() + Send>>,
}

impl Clone for AnimationConfig {
    fn clone(&self) -> Self {
        Self {
            mode: self.mode,
            loop_mode: self.loop_mode,
            delay: self.delay,
            on_complete: None,
        }
    }
}

impl Default for AnimationConfig {
    fn default() -> Self {
        Self {
            mode: AnimationMode::Spring(Spring::default()),
            loop_mode: None,
            delay: None,
            on_complete: None,
        }
    }
}

impl AnimationConfig {
    /// Create a new animation configuration
    pub fn new(mode: AnimationMode) -> Self {
        Self {
            mode,
            loop_mode: None,
            delay: None,
            on_complete: None,
        }
    }

    /// Set the loop mode
    pub fn with_loop(mut self, mode: LoopMode) -> Self {
        self.loop_mode = Some(mode);
        self
    }

    /// Set the delay before animation starts
    pub fn with_delay(mut self, delay: Duration) -> Self {
        self.delay = Some(delay);
        self
    }

    /// Set the completion callback
    pub fn with_on_complete<F: FnOnce() + Send + 'static>(mut self, callback: F) -> Self {
        self.on_complete = Some(Box::new(callback));
        self
    }
}

/// Mode of animation (spring or tween)
#[derive(Debug, Clone, Copy)]
pub enum AnimationMode {
    /// Spring-based physics animation
    Spring(Spring),
    /// Time-based tween animation
    Tween(Tween),
}
