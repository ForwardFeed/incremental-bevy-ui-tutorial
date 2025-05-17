use bevy::prelude::*;

/// Spawns a camera because withtout a camera
/// There's nothing that can be visible, even an UI.
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
