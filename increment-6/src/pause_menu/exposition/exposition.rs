use bevy::prelude::*;

use crate::{ecs::{despawn, despawn_children}, pause_menu::exposition::{c_align_items_ui::spawn_align_items, c_justify_text_ui::spawn_justify_text, main_content_ui::ExpositionMainContentTag}, state::{ExpositionState, PauseState}};

use super::root_ui::{spawn_pause_menu_exposition, PauseMenuExpositionUiTag};



pub struct ExpositionPlugin;

impl Plugin for ExpositionPlugin{
    fn build(&self, app: &mut App) {
        // I rather use a macro because it gets verbose really quickly
        macro_rules! add_content_side_selectible {
            ($func:ident, $state: ident) => {
                app.add_systems(OnEnter(ExpositionState::$state),
                    |parent: Single<Entity, With<ExpositionMainContentTag>>, mut commands: Commands|{
                        // I give the commands, so I will be able to spawn observers down the line
                        let child = $func(&mut commands);
                        //  And then I can add child to the main content parent.
                        commands.entity(*parent).add_child(child);
                    })
                .add_systems(OnExit(ExpositionState::$state), despawn_children::<ExpositionMainContentTag>)
            };
        }
        app
            .add_systems(OnEnter(PauseState::PauseMenuExposition), spawn_pause_menu_exposition)
            .add_systems(OnExit(PauseState::PauseMenuExposition), despawn::<PauseMenuExpositionUiTag>);
        add_content_side_selectible!(spawn_justify_text, JustifyText);
        add_content_side_selectible!(spawn_align_items, AlignItems);
        
    }
}