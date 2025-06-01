use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{actions::PauseMenuActions, ecs::despawn, state::PauseState};

use super::{exposition::exposition::{ExpositionPlugin}, rebind_ui::{spawn_pause_menu_keybinds, PauseMenuRebindsUITag, RebindPlugin}, root_ui::{spawn_pause_menu, PauseMenuUITag}, settings_ui::{spawn_pause_menu_settings, PauseMenuSettingsUITag}};

fn controls(
    query_pause_actions: Query<&ActionState<PauseMenuActions>>,
    current_state: Res<State<PauseState>>,
    mut next_state: ResMut<NextState<PauseState>>
){
    for action in query_pause_actions{
        if action.just_pressed(&PauseMenuActions::Activate){
            match current_state.get() {
                PauseState::Game => next_state.set(PauseState::PauseMenu),
                _ => next_state.set(PauseState::Game),
            }
        }
    }
}

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin{
    fn build(&self, app: &mut App) {

        app
            .add_systems(Update, controls)

            .add_systems(OnEnter(PauseState::PauseMenu), spawn_pause_menu)
            .add_systems(OnExit(PauseState::PauseMenu), despawn::<PauseMenuUITag>)

            .add_systems(OnEnter(PauseState::PauseMenuSettings), spawn_pause_menu_settings)
            .add_systems(OnExit(PauseState::PauseMenuSettings), despawn::<PauseMenuSettingsUITag>)

            .add_systems(OnEnter(PauseState::PauseMenuRebinds), spawn_pause_menu_keybinds)
            .add_systems(OnExit(PauseState::PauseMenuRebinds), despawn::<PauseMenuRebindsUITag>)

            // new plugin
            .add_plugins((RebindPlugin, ExpositionPlugin))
        ;
    }
}




