use leafwing_input_manager::prelude::*;
use bevy::prelude::*;

/// This is needed by leafwing_input
/// While this isn't essential, I find it great to keep track
/// of the actions from the user
#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum PauseMenuActions {
    Activate,
}