use std::time::Duration;

use bevy::{math::FloatOrd, picking::{backend::HitData, pointer::{Location, PointerId}}, prelude::*};
use bevy_render::camera::NormalizedRenderTarget;

// I like to simulate a click rather than do some complicated things
// To activate a part of the UI using the keyboard I just simulate a click
// It works.

fn default_pointer_location() -> Location {
    Location {
        target: NormalizedRenderTarget::Image(
            // This requires to add bevy_render to the project
            bevy_render::camera::ImageRenderTarget {
                handle: Handle::default(),
                scale_factor: FloatOrd(1.0),
            },
        ),
        position: Vec2::ZERO,
    }
}

fn default_hit() -> HitData{
    HitData {
        camera: Entity::PLACEHOLDER,
        depth: 0.0,
        position: None,
        normal: None,
    }
}

#[allow(dead_code)]
pub fn send_fake_mouse_click(target: Entity, commands: &mut Commands){
    commands.trigger_targets(Pointer::<Click>{
        target: target,
        pointer_id: PointerId::Mouse,
        pointer_location: default_pointer_location(),
        event: Click {
            hit: default_hit(),
            button: PointerButton::Primary,
            duration: Duration::from_secs_f32(0.1),
        },

    }, target);
}

pub fn send_fake_mouse_press(target: Entity, commands: &mut Commands){
    commands.trigger_targets(Pointer::<Pressed>{
        target: target,
        pointer_id: PointerId::Mouse,
        pointer_location: default_pointer_location(),
        event: Pressed {
            button: PointerButton::Primary,
            hit: default_hit(),
        },
    }, target);
}

#[allow(dead_code)]
pub fn send_fake_mouse_release(target: Entity, commands: &mut Commands){
    commands.trigger_targets(Pointer::<Released>{
        target: target,
        pointer_id: PointerId::Mouse,
        pointer_location: default_pointer_location(),
        event: Released {
            button: PointerButton::Primary,
            hit: default_hit()
        }
    }, target);
}

#[allow(dead_code)]
pub fn send_fake_mouse_over(target: Entity, commands: &mut Commands){
    commands.trigger_targets(
        Pointer::<Over>{
            target: target,
            pointer_id: PointerId::Mouse,
            pointer_location: default_pointer_location(),
            event: Over {
                hit: default_hit(),
            },
        },target);
}

#[allow(dead_code)]
pub fn send_fake_mouse_out(target: Entity, commands: &mut Commands){
    commands.trigger_targets(Pointer::<Out>{
        target: target,
        pointer_id: PointerId::Mouse,
        pointer_location: default_pointer_location(),
        event: Out {
            hit: default_hit(),
        },

    }, target);
}