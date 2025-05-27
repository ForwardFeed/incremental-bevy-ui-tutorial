use bevy::{input_focus::InputFocus, prelude::*};

use crate::fake_input::{send_fake_mouse_out, send_fake_mouse_over};

#[derive(Debug, Resource, FromWorld)]
pub struct PreviousFocusEntity(Option<Entity>);

fn trigger_focus_events(
    input_focus: Res<InputFocus>,
    mut previous_focus: ResMut<PreviousFocusEntity>,
    mut commands: Commands,
) {
    if let Some(entity) = input_focus.0 {
        if let Some(previous_entity) = previous_focus.0{
            if previous_entity == entity{
                return;
            }
            send_fake_mouse_out(previous_entity, &mut commands);
        }
        previous_focus.0 = Some(entity);
        send_fake_mouse_over(entity, &mut commands);
    }
}

pub struct FocusPlugin;

impl Plugin for FocusPlugin{
    fn build(&self, app: &mut App) {

        app
            .add_systems(Update, trigger_focus_events)
            .init_resource::<PreviousFocusEntity>()
        ;
    }
}