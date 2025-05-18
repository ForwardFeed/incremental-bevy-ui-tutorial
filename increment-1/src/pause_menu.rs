use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{actions::PauseMenuActions, state::PauseState};

#[derive(Component)]
pub struct PauseMenuUITag;

fn spawn_pause_menu(
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
                        
                        ..Default::default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.7, 0.3)),
                    Text("Hello World".into()),
                ), 
            ]
        ))
    ;
}

/// Utility function that will despawn any entity attached with given component
fn despawn<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    // Iterate and despawn all.
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
        ;
    }
}




