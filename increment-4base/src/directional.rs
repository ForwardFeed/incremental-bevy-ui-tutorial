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
            Ok(_entity) => {}
            Err(_e) => {},
        }
    }
}

/// Look if the user pressed Enter
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
    // By splitting into pressed and release with different behaviors
    // It gives a better sense of interactibility
    // But it has some small side visual effects when you pressed and move afterwards.
    if actions.just_pressed(&GeneralActions::Accept){
        // Faking pressing the mouse primary down
        send_fake_mouse_press(target, &mut commands);
    }

    if actions.just_released(&GeneralActions::Accept){
        // Faking releasing the mouse primary up
        send_fake_mouse_release(target, &mut commands);
    }
}

// The reason why this is needed is because
// in order to use bevy directionnal, you need the entities.
// But you also need to spawn with the parent.
// If you don't want to use that, you will to not use any improvement from bevy 0.16
// If you want other directionnality, you need to make another implementation with another spawnwith.
pub struct SpawnWithSouthEdges<F>(pub F);

impl<R: Relationship, F: FnOnce(&mut RelatedSpawner<R>)->Vec<Entity> + Send + Sync + 'static> SpawnableList<R>
    for SpawnWithSouthEdges<F>
{
    fn spawn(self, world: &mut World, entity: Entity) {
        // Resource scope here allows to borrow mutably multiple things.
        world.resource_scope(|world, mut directional_nav_map: Mut<DirectionalNavigationMap>| {
            // And yes each resource you need requires you to add another level of identation
            world.resource_scope(|world, mut input_focus: Mut<InputFocus>|{
                // Allows to spawn something and giving it automatically a relation of parent-child
                world.entity_mut(entity).with_related_entities(|parent : &mut RelatedSpawner<R> |{
                    // This is the vector of our buttons entity
                    let entities = self.0(parent);
                    // If you want to loop, replace by add_looping_edges
                    directional_nav_map.add_edges(&entities, CompassOctant::South);
                    if let Some(first_entity) = (*entities).get(0){
                        // we inform the focus that this is now the new focus.
                        // Which means spawning forces a new focus since there can be ONLY 1 focus.
                        input_focus.set(*first_entity);
                    }
                    
                });
            })
        });
    }
    // I have no clue what that does
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