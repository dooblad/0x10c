/// A state representing which game component is being inspected/debugged.
#[derive(PartialEq)]
pub enum DebugState {
    /// Nothing is being debugged.  Game behavior is normal in this state.
    None,
    PlayerViewRay,
    DepthBuffer,
}

impl DebugState {
    /// Defines a cycle through all states.  Given the current state, returns the next
    /// state in the cycle.
    pub fn next(&self) -> DebugState {
        use self::DebugState::*;
        match *self {
            None => PlayerViewRay,
            PlayerViewRay => DepthBuffer,
            DepthBuffer => None,
        }
    }
}