use leafwing_input_manager::prelude::*;
use bevy::prelude::*;

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum PauseMenuActions {
    Activate,
}

// new! I also use that as a component for the rebind UI.
#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect, Component)]
pub enum GeneralActions {
    MoveUp, 
    MoveDown, 
    MoveLeft, 
    MoveRight, 
    Accept,
}