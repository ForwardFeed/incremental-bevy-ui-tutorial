use bevy::ecs::relationship::RelatedSpawner;
use bevy::ecs::spawn::SpawnWith;
use bevy::prelude::*;
use bevy::color::palettes::css::{BLUE, RED};

#[derive(Component)]
struct ShadownBoxMarker;

pub fn box_shadow_ui() -> impl Bundle{
        (
            Node{
                flex_direction: FlexDirection::Row,
                width: Val::Percent(100.),
                ..Default::default()
            },
            
            children![
                boxes_control(),
                boxes_shadow() 
            ]    
        )
}

fn boxes_control() -> impl Bundle{
    (
        Node{
            width: Val::Percent(100.),
            ..Default::default()
        },
        Children::spawn(SpawnWith(|parent: &mut RelatedSpawner<ChildOf>|{
            parent.spawn(
                control_text("-x"),
            ).observe(|_trigger: Trigger<Pointer<Released>>, q_shadow_boxes: Query<&mut BoxShadow, With<ShadownBoxMarker>>|{
                for mut shadow_box in q_shadow_boxes{
                    for shadow_style in shadow_box.iter_mut(){
                        shadow_style.x_offset = shadow_style.x_offset * 0.99
                    }
                }
            });
        }))
    )
}

fn control_text<T: Into<String>>(text: T) -> impl Bundle{
    (
        Node{
            ..Default::default()
        },
        Text::new(text)
    )
}

fn boxes_shadow() -> impl Bundle{
    (
        Node{
            ..Default::default()
        },
        BoxShadow::new(
            BLUE.with_alpha(0.7).into(),
            Val::Px(1.), 
            Val::Px(5.),
            Val::Px(5.), 
            Val::Px(1.) 
        ),
        BackgroundColor(RED.into()),
        ShadownBoxMarker,
        children![
            Text::new("azeazeaze")
        ],
    )
}