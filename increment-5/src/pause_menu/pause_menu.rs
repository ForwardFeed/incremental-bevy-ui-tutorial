use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{actions::PauseMenuActions, state::PauseState};

use super::{rebind_ui::{spawn_pause_menu_keybinds, PauseMenuRebindsUIMarker, RebindPlugin}, root_ui::{spawn_pause_menu, PauseMenuUIMarker}, settings_ui::{spawn_pause_menu_settings, PauseMenuSettingsUIMarker}};


pub fn despawn<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn();
    }
}

fn controls(
    query_pause_actions: Query<&ActionState<PauseMenuActions>>,
    current_state: Res<State<PauseState>>,
    mut next_state: ResMut<NextState<PauseState>>
){
    for action in query_pause_actions{
        if action.just_pressed(&PauseMenuActions::Activate){
            match current_state.get() {
                PauseState::Game => next_state.set(PauseState::PauseMenu),
                PauseState::PauseMenu => next_state.set(PauseState::Game),
                PauseState::PauseMenuSettings => next_state.set(PauseState::Game),
                PauseState::PauseMenuRebinds => next_state.set(PauseState::Game),
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
            .add_systems(OnExit(PauseState::PauseMenu), despawn::<PauseMenuUIMarker>)

            .add_systems(OnEnter(PauseState::PauseMenuSettings), spawn_pause_menu_settings)
            .add_systems(OnExit(PauseState::PauseMenuSettings), despawn::<PauseMenuSettingsUIMarker>)

            // new systems
            .add_systems(OnEnter(PauseState::PauseMenuRebinds), spawn_pause_menu_keybinds)
            .add_systems(OnExit(PauseState::PauseMenuRebinds), despawn::<PauseMenuRebindsUIMarker>)
            // new plugin
            .add_plugins(RebindPlugin)
        ;
    }
}




