use bevy::{input_focus::InputFocus, prelude::*};

use crate::fake_input::{send_fake_mouse_out, send_fake_mouse_over};

// InputFocus has the real current, But It won't tell you if it's a new one. 
// So you have to keep the information somewhere
#[derive(Debug, Resource, FromWorld)]
pub struct PreviousFocusEntity(Option<Entity>);

// Focus in will trigger a fake mouse over
// And focus out will trigger a fake mouse out
// I previously wrote a FocusIn And FocusOut here, but went back on that design
// converging things down to pointer events, just works great.
fn trigger_focus_events(
    input_focus: Res<InputFocus>,
    mut previous_focus: ResMut<PreviousFocusEntity>,
    mut commands: Commands,
) {
    if let Some(entity) = input_focus.0 {
        if let Some(previous_focus) = previous_focus.0{
            // if the entity is the same don't do anything
            if previous_focus == entity{
                return;
            }
            // trigger the old entity saying the focus has moved
            send_fake_mouse_out(previous_focus, &mut commands);
        }
        previous_focus.0 = Some(entity);
        // trigger the new entity saying that it has the focus
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