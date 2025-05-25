use bevy::{input_focus::InputFocus, prelude::*};

#[derive(Debug, Resource, FromWorld)]
pub struct CurrentFocusEntity(Option<Entity>);



#[derive(Copy, Clone, Event)]
pub struct FocusIn;

#[derive(Copy, Clone, Event)]
pub struct FocusOut;


fn trigger_focus_events(
    input_focus: Res<InputFocus>,
    mut current_focus: ResMut<CurrentFocusEntity>,
    mut commands: Commands,
) {
    if let Some(entity) = input_focus.0 {
        if let Some(curr_entity) = current_focus.0{
            if curr_entity == entity{
                return;
            }
            commands.trigger_targets(FocusOut, curr_entity);
        }
        current_focus.0 = Some(entity);
        commands.trigger_targets(FocusIn, entity);
    }
}

pub struct FocusPlugin;

impl Plugin for FocusPlugin{
    fn build(&self, app: &mut App) {

        app
            .add_systems(Update, trigger_focus_events)
            
            .init_resource::<CurrentFocusEntity>()
        ;
    }
}