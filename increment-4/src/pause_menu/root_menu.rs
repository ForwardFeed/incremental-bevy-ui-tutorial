use bevy::{ecs::relationship::RelatedSpawner, prelude::*};

use crate::{directional::SpawnWithSouthEdges, pause_menu::shared_widgets::{focus_in_observer, focus_out_observer}, state::PauseState};

use super::shared_widgets::{hover_observer, out_observer, pause_menu_button_widget, pressed_observer};


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
                    // I have replaced by a self made SpawnWithSouthEdges that is very similar to SpawnWith
                    Children::spawn(SpawnWithSouthEdges(spawn_pause_menu_root_buttons))
                ),
                
            ]
        )
    );
}

// returns a Vector of entity
// needed by the directionnal navigator
fn spawn_pause_menu_root_buttons(parent: &mut RelatedSpawner<ChildOf>) -> Vec<Entity>{
    vec![
        parent.spawn(pause_menu_button_widget("Resume"))
            .observe(onclick_resume)
            .observe(hover_observer)  
            .observe(out_observer)
            .observe(pressed_observer)
            .observe(focus_in_observer)
            .observe(focus_out_observer)
            .id(), // don't forget to return the IDs
        parent.spawn(pause_menu_button_widget("Settings" ))
            .observe(onclick_settings)
            .observe(hover_observer)  
            .observe(out_observer)
            .observe(pressed_observer)
            .observe(focus_in_observer)
            .observe(focus_out_observer)
            .id(),
        parent.spawn(pause_menu_button_widget("Quit"))
            .observe(onclick_quit)
            .observe(hover_observer)  
            .observe(out_observer)
            .observe(pressed_observer)
            .observe(focus_in_observer)
            .observe(focus_out_observer)
            .id(),
    ]
}

fn onclick_resume(_trigger: Trigger<Pointer<Released>>, mut next_state: ResMut<NextState<PauseState>>){
    next_state.set(PauseState::Game)
}

fn onclick_settings(_trigger: Trigger<Pointer<Released>>, mut next_state: ResMut<NextState<PauseState>>){
    next_state.set(PauseState::PauseMenuSettings)
}

fn onclick_quit(_trigger: Trigger<Pointer<Released>>, mut exit: EventWriter<AppExit>){
    exit.write(AppExit::Success);
}