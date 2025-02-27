//! Platform abstraction for time-related functionality
//!
//! Provides cross-platform timing operations for animations.
//! Supports both web (WASM) and native platforms.

use instant::{Duration, Instant};
use std::future::Future;
use tokio_with_wasm::alias as tokio;

#[cfg(feature = "web")]
use wasm_bindgen::closure::Closure;

/// Provides platform-agnostic timing operations
///
/// Abstracts timing functionality across different platforms,
/// ensuring consistent animation behavior in both web and native environments.
pub trait TimeProvider {
    /// Returns the current instant
    fn now() -> Instant;

    /// Creates a future that completes after the specified duration
    fn delay(duration: Duration) -> impl Future<Output = ()>;
}

/// Default time provider implementation for motion animations
///
/// Implements platform-specific timing operations:
/// - For web: Uses requestAnimationFrame or setTimeout
/// - For native: Uses tokio's sleep
pub struct MotionTime;

impl TimeProvider for MotionTime {
    fn now() -> Instant {
        Instant::now()
    }

    fn delay(duration: Duration) -> impl Future<Output = ()> {
        tokio::time::sleep(duration)
    }
}

// Add a new helper for frame timing
pub async fn request_animation_frame() {
    #[cfg(feature = "web")]
    {
        use wasm_bindgen::JsCast;
        let (tx, rx) = tokio::sync::oneshot::channel();
        let cb = Closure::once_into_js(move || {
            let _ = tx.send(());
        });

        web_sys::window()
            .expect("global window does not exist")
            .request_animation_frame(cb.as_ref().unchecked_ref())
            .expect("should register `requestAnimationFrame` OK");

        rx.await.expect("channel should not be closed");
    }

    #[cfg(feature = "desktop")]
    {
        tokio::time::sleep(Duration::from_micros(16667)).await;
    }
}
