use bevy::prelude::*;

fn setup(mut commands: Commands){
    commands.spawn((
        Camera2d::default(),
        Transform::from_xyz(0., 0., 0.),
    ));
}



pub struct CameraPlugin;

impl Plugin for CameraPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup);
    }
}
