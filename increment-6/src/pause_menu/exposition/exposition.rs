use bevy::prelude::*;

use crate::{pause_menu::pause_menu::despawn, state::PauseState};

use super::root_ui::{spawn_pause_menu_exposition, PauseMenuExpositionUiTag};



pub struct ExpositionPlugin;

impl Plugin for ExpositionPlugin{
    fn build(&self, app: &mut App) {

        app
            .add_systems(OnEnter(PauseState::PauseMenuExposition), spawn_pause_menu_exposition)
            .add_systems(OnExit(PauseState::PauseMenuExposition), despawn::<PauseMenuExpositionUiTag>)
        ;
        
    }
}