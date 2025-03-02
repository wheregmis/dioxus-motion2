/// State of an animation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationState {
    /// Animation is still running
    Active,
    /// Animation has completed
    Completed,
}
