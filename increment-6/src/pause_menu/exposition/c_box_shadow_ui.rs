use bevy::ecs::relationship::RelatedSpawner;
use bevy::ecs::spawn::SpawnWith;
use bevy::prelude::*;
use bevy::color::palettes::css::RED;

use crate::common_button;

#[derive(Component)]
struct ShadownBoxMarker;
#[derive(Component)]
struct ShadownBoxMarkerText;

// S like shift and also it's the 1337 5P34K of 5
const S: f32 = 5.0;

pub fn box_shadow_ui() -> impl Bundle{
        (
            Node{
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                width: Val::Percent(100.),
                height: Val::Percent(100.),
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
            common_button!(parent, "+x 5px", observer_release_control_px);
            common_button!(parent, "-x 5px", observer_release_control_mx);
            common_button!(parent, "+y 5px", observer_release_control_py);
            common_button!(parent, "-y 5px", observer_release_control_my);
        }))
    )
}


fn observer_release_control_px(_trigger: Trigger<Pointer<Released>>, q_shadow_boxes: Query<&mut BoxShadow, With<ShadownBoxMarker>>){
    for mut shadow_box in q_shadow_boxes{
        for shadow_style in shadow_box.iter_mut(){
            shadow_style.x_offset = match shadow_style.x_offset{
                Val::Px(x) => {
                    Val::Px(x + S)
                },
                _ => {
                    // If someone changes that to anything but pixels it will simply make this system not doing anything.
                    return; 
                }
            }
        }
    }
}
fn observer_release_control_mx(_trigger: Trigger<Pointer<Released>>, q_shadow_boxes: Query<&mut BoxShadow, With<ShadownBoxMarker>>){
    for mut shadow_box in q_shadow_boxes{
        for shadow_style in shadow_box.iter_mut(){
            shadow_style.x_offset = match shadow_style.x_offset{
                Val::Px(x) => {
                    Val::Px(x - S)
                },
                _ => { return; }
            }
        }
    }
}
fn observer_release_control_py(_trigger: Trigger<Pointer<Released>>, q_shadow_boxes: Query<&mut BoxShadow, With<ShadownBoxMarker>>){
    for mut shadow_box in q_shadow_boxes{
        for shadow_style in shadow_box.iter_mut(){
            shadow_style.y_offset = match shadow_style.y_offset{
                Val::Px(y) => {
                    Val::Px(y + S)
                },
                _ => {
                    // If someone changes that to anything but pixels it will simply make this system not doing anything.
                    return; 
                }
            }
        }
    }
}
fn observer_release_control_my(_trigger: Trigger<Pointer<Released>>, q_shadow_boxes: Query<&mut BoxShadow, With<ShadownBoxMarker>>){
    for mut shadow_box in q_shadow_boxes{
        for shadow_style in shadow_box.iter_mut(){
            shadow_style.y_offset = match shadow_style.y_offset{
                Val::Px(y) => {
                    Val::Px(y - S)
                },
                _ => { return; }
            }
        }
    }
}


const BTN_ADD_TEXT: &str = "Click to add another shadow";
fn boxes_shadow() -> impl Bundle{
    (
        Node{
            width: Val::Percent(20.),
            height: Val::Percent(20.),
            margin: UiRect::AUTO,
            ..Default::default()
        },
        BoxShadow(vec![]),
        // If you remove the background color, then you will be surprised to see that the background 
        // will be the shadow (sort of)
        BackgroundColor(RED.into()),
        ShadownBoxMarker,
        Children::spawn(SpawnWith(|parent: &mut RelatedSpawner<ChildOf>|{
            parent.spawn(
                (
                    ShadownBoxMarkerText,
                    Text::new(BTN_ADD_TEXT)
                )
            ).observe(|_trigger: Trigger<Pointer<Pressed>>, mut text: Single<&mut Text, With<ShadownBoxMarkerText>>|{
                **text = "Will release another shadow".into();
            })
            .observe(|_trigger: Trigger<Pointer<Released>>, 
                mut shadow: Single<&mut BoxShadow, With<ShadownBoxMarker>>,
                mut text: Single<&mut Text, With<ShadownBoxMarkerText>>|{
                let len = shadow.iter().len();
                shadow.push(create_shadow(len as f32 + 1.0));
                **text = BTN_ADD_TEXT.into();
            });
        }))
    )
}

fn create_shadow(n: f32) -> ShadowStyle{
    let r = n * 0.2 % 1.0;
    let g = n * 0.4 % 1.0;
    let b = n * 0.6 % 1.0;
    let x = if n % 2.0 == 0.0{
        if n % 4.0 == 0.0 { -n } else { n }
    } else {
        if n % 3.0 == 0.0{ -n } else { n }
    } * S * 1.2;
    let y = if n % 3.0 == 0.0{
        if n % 5.0 == 0.0 { n } else { -n }
    } else {
        if n % 2.0 == 0.0{ n } else { -n }
    } * S * 1.2;

    ShadowStyle{
        color: Color::srgb(r, g, b),
        // Please if you change that from Px to anything else
        // Modify the systems units related too.
        x_offset: Val::Px(x),
        y_offset: Val::Px(y),
        spread_radius: Val::Percent(n * 3.0 % 20.0),
        blur_radius: Val::Px(n % 3.0),
    }
}