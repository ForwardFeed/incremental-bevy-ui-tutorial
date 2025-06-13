use bevy::prelude::*;

use crate::{ecs::{despawn, despawn_children}, pause_menu::exposition::{c_align_items_ui::align_items_ui, c_box_shadow_ui::box_shadow_ui, c_grid_ui::grid_ui, c_justify_text_ui::justify_text_ui, main_content_ui::ExpositionMainContentMarker}, state::{ExpositionState, PauseState}};

use super::root_ui::{spawn_pause_menu_exposition, PauseMenuExpositionUiMarker};



pub struct ExpositionPlugin;

impl Plugin for ExpositionPlugin{
    fn build(&self, app: &mut App) {
        // I rather use a macro because it gets verbose really quickly
        macro_rules! add_content_side_selectible {
            ($func:ident, $state: ident) => {
                app.add_systems(OnEnter(ExpositionState::$state),
                    |parent: Single<Entity, With<ExpositionMainContentMarker>>, mut commands: Commands|{
                        //spawning the children element
                        let child = commands.spawn($func()).id();
                        // Adding the bundle function to the parent
                        commands.entity(*parent).add_child(child);
                    })
                .add_systems(OnExit(ExpositionState::$state), despawn_children::<ExpositionMainContentMarker>)
            };
        }
        app
            .add_systems(OnEnter(PauseState::PauseMenuExposition), spawn_pause_menu_exposition)
            .add_systems(OnExit(PauseState::PauseMenuExposition), despawn::<PauseMenuExpositionUiMarker>);
        add_content_side_selectible!(justify_text_ui, JustifyText);
        add_content_side_selectible!(align_items_ui, AlignItems);
        add_content_side_selectible!(box_shadow_ui, BoxShadow);
        add_content_side_selectible!(grid_ui, Grid);
    }
}