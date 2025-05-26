use bevy::{input_focus::InputFocus, prelude::*};

// InputFocus has the real current, But It won't tell you if it's a new one. 
// So you have to keep the information somewhere
#[derive(Debug, Resource, FromWorld)]
pub struct PreviousFocusEntity(Option<Entity>);

// Two Custom Events, I hope FocusIn and FocusOut to be in bevy in the future
#[derive(Copy, Clone, Event)]
pub struct FocusIn;
#[derive(Copy, Clone, Event)]
pub struct FocusOut;

// This will link the focus event
fn trigger_focus_events(
    input_focus: Res<InputFocus>,
    mut current_focus: ResMut<PreviousFocusEntity>,
    mut commands: Commands,
) {
    if let Some(entity) = input_focus.0 {
        if let Some(curr_entity) = current_focus.0{
            // if the entity is the same don't do anything
            if curr_entity == entity{
                return;
            }
            // trigger the old entity saying the focus has moved
            commands.trigger_targets(FocusOut, curr_entity);
        }
        current_focus.0 = Some(entity);
        // trigger the new entity saying that it has the focus
        commands.trigger_targets(FocusIn, entity);
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