use bevy::{ecs::relationship::{RelatedSpawner}, prelude::*};

use crate::state::PauseState;

use super::root_ui::{COLOR_BG_A, COLOR_RETURN, COLOR_RETURN_OVER};

#[derive(Component)]
pub struct ExpositionMainContentTag;

#[derive(Component)]
enum MainContentButtonsTag{
    Return
}

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
    parent.spawn(
        (
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(30.),
                height: Val::Percent(10.),
                top: Val::Percent(-10.),
                right: Val::Percent(0.),
                align_items: AlignItems::Center,
                ..Default::default()
            },
            MainContentButtonsTag::Return,
            BackgroundColor(COLOR_RETURN),
            children![
                (
                    Node{
                        flex_grow: f32::MAX,
                        ..Default::default()
                    },
                    Pickable::IGNORE,
                    TextLayout::new_with_justify(JustifyText::Center),
                    Text::new("Return")
                )
            ]
            
        )
    ).observe(|_trigger: Trigger<Pointer<Released>>, mut next_state: ResMut<NextState<PauseState>>|{
        next_state.set(PauseState::PauseMenu);
    })
    .observe(|trigger: Trigger<Pointer<Over>>, q_buttons: Query<(Entity, &mut BackgroundColor), With<MainContentButtonsTag>>|{
        for (entity, mut color) in q_buttons{
            if entity == trigger.target{
                *color = COLOR_RETURN_OVER.into();
            }
        }
    })
    .observe(|trigger: Trigger<Pointer<Out>>, q_buttons: Query<(Entity, &mut BackgroundColor), With<MainContentButtonsTag>>|{
        for (entity, mut color) in q_buttons{
            if entity == trigger.target{
                *color = COLOR_RETURN.into();
            }
        }
    })
    ;
}