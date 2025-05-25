use bevy::{
    ecs::{
        relationship::{
            RelatedSpawner,
            Relationship
        }, spawn::SpawnableList
    }, input_focus::{
        directional_navigation::{
            DirectionalNavigation,
            DirectionalNavigationMap,
            DirectionalNavigationPlugin
        },
        InputDispatchPlugin,
        InputFocus
    },
    math::CompassOctant,
    prelude::*
};
use leafwing_input_manager::prelude::*;

use crate::{actions::GeneralActions, fake_input::{send_fake_mouse_press, send_fake_mouse_release}};

fn controls_directions(
    actions: Single<&ActionState<GeneralActions>>,
    mut directional_navigation: DirectionalNavigation,
){
    // If the user is pressing both left and right, or up and down,
    // it should move in neither direction.
    let net_east_west = actions
        .just_pressed(&GeneralActions::MoveRight) as i8
        - actions
        .just_pressed(&GeneralActions::MoveLeft) as i8;

    let net_north_south = actions
        .just_pressed(&GeneralActions::MoveUp) as i8
        - actions
        .just_pressed(&GeneralActions::MoveDown) as i8;
    

    // Compute the direction that the user is trying to navigate in
    let maybe_direction = match (net_east_west, net_north_south) {
        (0, 0) => None,
        (0, 1) => Some(CompassOctant::North),
        (1, 1) => Some(CompassOctant::NorthEast),
        (1, 0) => Some(CompassOctant::East),
        (1, -1) => Some(CompassOctant::SouthEast),
        (0, -1) => Some(CompassOctant::South),
        (-1, -1) => Some(CompassOctant::SouthWest),
        (-1, 0) => Some(CompassOctant::West),
        (-1, 1) => Some(CompassOctant::NorthWest),
        _ => None,
    };
    if let Some(direction) = maybe_direction {
        match directional_navigation.navigate(direction) {
            // In a real game, you would likely want to play a sound or show a visual effect
            // on both successful and unsuccessful navigation attempts
            Ok(_entity) => {
            }
            Err(_e) => {},
        }
    }
}

/// Look if the user pressed enter
fn controls_accept(
    actions: Single<&ActionState<GeneralActions>>,
    directional_navigation: DirectionalNavigation,
    mut commands: Commands,
){
    let target = match directional_navigation.focus.0 {
        Some(x) => x,
        None => {
            return;
        },
    };
    if actions.just_pressed(&GeneralActions::Accept){
        // Faking pressing the mouse primary down
        send_fake_mouse_press(target, &mut commands);
    }

    if actions.just_released(&GeneralActions::Accept){
        // Faking releasing the mouse primary up
        send_fake_mouse_release(target, &mut commands);
    }
}

pub struct SpawnWithSouthEdges<F>(pub F);

impl<R: Relationship, F: FnOnce(&mut RelatedSpawner<R>)->Vec<Entity> + Send + Sync + 'static> SpawnableList<R>
    for SpawnWithSouthEdges<F>
{
    fn spawn(self, world: &mut World, entity: Entity) {
        world.resource_scope(|world, mut directional_nav_map: Mut<DirectionalNavigationMap>| {
            world.resource_scope(|world, mut input_focus: Mut<InputFocus>|{
                world.entity_mut(entity).with_related_entities(|parent : &mut RelatedSpawner<R> |{
                    let entities = self.0(parent);
                    directional_nav_map.add_edges(&entities, CompassOctant::South);
                    let top_left_entity = *entities.get(0).unwrap();
                    input_focus.set(top_left_entity);
                });
            })
        });
    }
    fn size_hint(&self) -> usize {
        1
    }
}



pub struct DirectionalPlugin;

impl Plugin for DirectionalPlugin{
    fn build(&self, app: &mut App) {

        app
            .add_plugins((
                // We need both of these ones.
                InputDispatchPlugin,
                DirectionalNavigationPlugin
            ))
            // the chain is to be sure than it's always run the direction first and the accept after
            // I do believe it's a small detail
            .add_systems(Update, (controls_directions, controls_accept).chain())
        ;
    }
}