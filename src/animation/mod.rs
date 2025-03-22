mod config;
mod state;
pub mod timing;
mod traits;

pub use config::{AnimationConfig, AnimationMode};
pub use state::AnimationState;
pub use timing::{AnimationTiming, LoopMode, PlaybackDirection};
pub use traits::Animation;
