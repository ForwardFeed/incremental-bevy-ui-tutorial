use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{actions::PauseMenuActions, state::PauseState};

/// A component tag, Its only purpose is to allow targetting the entity it's attached to by queries
#[derive(Component)]
pub struct PauseMenuUITag;

/**
 * There's a common question about if it's better to spawn or to hide.
 * Both works it's about if you want to conserve your UI state.
 * If you hide you can conserve the state the UI was, but at the same time
 * it's somewhat of an additionnal constraint
 */
/// Spawn the root of the menu
fn spawn_pause_menu(
    mut commands: Commands,
){
    commands
        .spawn((
            // The root node will be invisible and take all the screen
            // Its purpose is to center the menu that we will place as children of this node.
            // It is required if you want to place you buttons somewhere precisely on the screen
            // Although you can play with margins eventually? doesn't feel convenient to me.
            Node {
                // It looks like CSS but it isn't, just similar.
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                ..Default::default()
            },
            // Here's the component-tag from earlier
            PauseMenuUITag,
            // This new macro from 0.16 is handy to spawn children
            children![
                (
                    Node {
                        
                        ..Default::default()
                    },
                    // this gives a green color to our text node
                    BackgroundColor(Color::srgb(0.2, 0.7, 0.3)),
                    // Hello World will be printed into it
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

/// Will listen every frame.
fn controls(
    // query the actions
    query_pause_actions: Query<&ActionState<PauseMenuActions>>,
    // current pause state
    current_state: Res<State<PauseState>>,
    // the ability to modify the state
    mut next_state: ResMut<NextState<PauseState>>
){
    for action in query_pause_actions{
        // If escape has been pressed
        if action.just_pressed(&PauseMenuActions::Activate){
            // Toggle the state
            match current_state.get() {
                PauseState::Game => next_state.set(PauseState::PauseMenu),
                PauseState::PauseMenu => next_state.set(PauseState::Game),
            }
        }
    }
}

/// This Plugin will be the root of the logic of what will guide
/// The whole pause menu I intend to show in that series.
pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin{
    fn build(&self, app: &mut App) {

        app
            // The update schedule is each frame, which is important for controls.
            .add_systems(Update, controls)
            // If we enter the pause menu state, the pause menu will show
            .add_systems(OnEnter(PauseState::PauseMenu), spawn_pause_menu)
            // If we exit the pause menu state, the pause menu will be destroyed
            .add_systems(OnExit(PauseState::PauseMenu), despawn::<PauseMenuUITag>)
        ;
    }
}




