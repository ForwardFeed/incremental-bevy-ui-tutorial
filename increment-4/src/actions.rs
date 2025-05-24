use leafwing_input_manager::prelude::*;
use bevy::prelude::*;

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum PauseMenuActions {
    Activate,
}