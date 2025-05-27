use bevy::prelude::*;

use crate::actions::GeneralActions;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PauseState {
    #[default]
    Game,
    PauseMenu,
    PauseMenuSettings,
    // new state
    PauseMenuRebinds,
}

// new state that dictate when to listen to rebinding
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum RebindGeneralActionState {
    #[default]
    None,
    Rebinding(GeneralActions)
}

pub struct StatesPlugin;

impl Plugin for StatesPlugin{
    fn build(&self, app: &mut App) {
        app
            .init_state::<PauseState>()
            .init_state::<RebindGeneralActionState>()
        ;
    }
}