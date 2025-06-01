use bevy::{ecs::relationship::{RelatedSpawner}, prelude::*};

use super::root_ui::COLOR_BG_A;

#[derive(Component)]
pub struct ExpositionMainContentTag;

pub fn spawn_main_content_holder(parent: &mut RelatedSpawner<ChildOf>){
    parent.spawn(
        (
            Node{
                width: Val::Percent(80.),
                height: Val::Percent(100.),
                ..Default::default()
            },
            ExpositionMainContentTag,
            BackgroundColor(COLOR_BG_A),
        )
    );
}