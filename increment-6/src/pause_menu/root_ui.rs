use bevy::{ecs::relationship::RelatedSpawner, prelude::*};

use crate::{directional::SpawnWithSouthEdges, fn_vertical_row_common_buttons, state::PauseState};


#[derive(Component)]
pub struct PauseMenuUITag;

pub fn spawn_pause_menu(
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
            PauseMenuUITag,
            children![ 
                (
                    Node {
                        width: Val::Percent(30.),
                        height: Val::Percent(30.),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::SpaceEvenly,
                        ..Default::default()
                    },
                    Children::spawn(SpawnWithSouthEdges(spawn_pause_menu_root_buttons))
                ),
                
            ]
        )
    );
}

fn_vertical_row_common_buttons!(spawn_pause_menu_root_buttons, [
    ("Resume", onclick_resume),
    ("Settings", onclick_settings),
    ("UI Exposition", onclick_exposition),
    ("Quit", onclick_quit)
]);

fn onclick_resume(_trigger: Trigger<Pointer<Released>>, mut next_state: ResMut<NextState<PauseState>>){
    next_state.set(PauseState::Game)
}

fn onclick_settings(_trigger: Trigger<Pointer<Released>>, mut next_state: ResMut<NextState<PauseState>>){
    next_state.set(PauseState::PauseMenuSettings)
}

fn onclick_exposition(_trigger: Trigger<Pointer<Released>>, mut next_state: ResMut<NextState<PauseState>>){
    next_state.set(PauseState::PauseMenuExposition)
}

fn onclick_quit(_trigger: Trigger<Pointer<Released>>, mut exit: EventWriter<AppExit>){
    exit.write(AppExit::Success);
}
