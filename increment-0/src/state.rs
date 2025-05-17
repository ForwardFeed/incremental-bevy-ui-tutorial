use bevy::prelude::*;

/// A state to keep track to what to run, what not to run
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PauseState {
    #[default]
    Game,
    PauseMenu,
}

/// While this is not essential and could be done in the main.rs.
/// Separating your code into multiple plugins makes it more understandable
/// by keeping the logic into a single file.
pub struct StatesPlugin;

impl Plugin for StatesPlugin{
    fn build(&self, app: &mut App) {
        app
            .init_state::<PauseState>()
        ;
    }
}