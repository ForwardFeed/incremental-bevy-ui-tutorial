use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{actions::PauseMenuActions, directional::navigation, state::PauseState};

use super::{root_menu::{spawn_pause_menu, PauseMenuUITag, RootButtons}, settings_menu::{spawn_pause_menu_settings, PauseMenuSettingsUITag}, shared_widgets::pause_button_system};


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
            }
        }
    }
}

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin{
    fn build(&self, app: &mut App) {

        app
            .add_systems(Update, controls)
            // Handles the keyboard input actions
            .add_systems(Update, navigation::<RootButtons>
                .run_if(in_state(PauseState::PauseMenu)))

            .add_systems(Update, pause_button_system)

            .add_systems(OnEnter(PauseState::PauseMenu), spawn_pause_menu)
            .add_systems(OnExit(PauseState::PauseMenu), despawn::<PauseMenuUITag>)

            .add_systems(OnEnter(PauseState::PauseMenuSettings), spawn_pause_menu_settings)
            .add_systems(OnExit(PauseState::PauseMenuSettings), despawn::<PauseMenuSettingsUITag>)
        ;
    }
}




