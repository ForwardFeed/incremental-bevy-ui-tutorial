use bevy::{ecs::{relationship::RelatedSpawner}, prelude::*};

use crate::{directional::SpawnWithSouthEdges, fn_vertical_row_common_buttons, state::PauseState};

#[derive(Component)]
pub struct PauseMenuSettingsUIMarker;

pub fn spawn_pause_menu_settings(
    mut commands: Commands,
){
    commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                ..Default::default()
            },
            PauseMenuSettingsUIMarker,
            children![ 
                (
                    Node {
                        width: Val::Percent(30.),
                        height: Val::Percent(30.),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::SpaceEvenly,
                        row_gap: Val::Percent(2.),
                        ..Default::default()
                    },
                    Children::spawn(SpawnWithSouthEdges(spawn_pause_menu_settings_buttons))
                )
            ]
        ))
    ;
}


fn_vertical_row_common_buttons!(spawn_pause_menu_settings_buttons, [
    ("Keybinds", onclick_keybinds),
    ("PlaceHolder", onclick_placeholder),
    ("Return", onclick_return)
]);

fn onclick_keybinds(_trigger: Trigger<Pointer<Released>>, mut next_state: ResMut<NextState<PauseState>>){
    next_state.set(PauseState::PauseMenuRebinds);
}

fn onclick_placeholder(_trigger: Trigger<Pointer<Released>>){
    info!("I haven't set anything about that yet, but who knows, I may need it.");
}

fn onclick_return(_trigger: Trigger<Pointer<Released>>, mut next_state: ResMut<NextState<PauseState>>){
    next_state.set(PauseState::PauseMenu);
}