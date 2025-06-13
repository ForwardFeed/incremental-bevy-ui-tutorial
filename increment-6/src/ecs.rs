use bevy::prelude::*;

pub fn despawn<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn();
    }
}

/// Utility that dispawn all children from the entity that was marked by a component
pub fn despawn_children<T: Component>(to_despawn_children: Query<Entity, With<T>>, mut commands: Commands){
    for entity in to_despawn_children{
        // this may get merged into despawn_children later, see https://github.com/bevyengine/bevy/pull/19283
        commands.entity(entity).despawn_related::<Children>();
    }
    
}