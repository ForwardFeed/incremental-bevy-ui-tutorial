use leafwing_input_manager::prelude::*;
use bevy::prelude::*;

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum PauseMenuActions {
    Activate,
}

// Creating a new set of actions
// I call it General Actions, but it is fair to call it UIActions too
#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum GeneralActions {
    MoveUp, // W
    MoveDown, // S
    MoveLeft, // A
    MoveRight, // D
    Accept, // Enter
}