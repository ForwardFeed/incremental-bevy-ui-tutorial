use leafwing_input_manager::prelude::*;
use bevy::prelude::*;

/// This will be binded to some user inputs
/// So you don't have to use the keycode and use actions instead.
#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum PauseMenuActions {
    Activate,
}