use std::time::Duration;

use bevy::{math::FloatOrd, picking::{backend::HitData, pointer::{Location, PointerId}}, prelude::*};
use bevy_render::camera::NormalizedRenderTarget;

#[allow(dead_code)]
pub fn send_fake_mouse_click(target: Entity, commands: &mut Commands){
    commands.trigger_targets(Pointer::<Click>{
        target: target,
        pointer_id: PointerId::Mouse,
        pointer_location: Location {
            target: NormalizedRenderTarget::Image(
                // This requires to add bevy render to the project
                bevy_render::camera::ImageRenderTarget {
                    handle: Handle::default(),
                    scale_factor: FloatOrd(1.0),
                },
            ),

            position: Vec2::ZERO,
        },
        event: Click {
            button: PointerButton::Primary,
            hit: HitData {
                camera: Entity::PLACEHOLDER,
                depth: 0.0,
                position: None,
                normal: None,
            },
            duration: Duration::from_secs_f32(0.1),
        },

    }, target);
}

pub fn send_fake_mouse_press(target: Entity, commands: &mut Commands){
    commands.trigger_targets(Pointer::<Pressed>{
        target: target,
        pointer_id: PointerId::Mouse,
        pointer_location: Location {
            target: NormalizedRenderTarget::Image(
                // This requires to add bevy render to the project
                bevy_render::camera::ImageRenderTarget {
                    handle: Handle::default(),
                    scale_factor: FloatOrd(1.0),
                },
            ),

            position: Vec2::ZERO,
        },
        event: Pressed {
            button: PointerButton::Primary,
            hit: HitData {
                camera: Entity::PLACEHOLDER,
                depth: 0.0,
                position: None,
                normal: None,
            },
        },
    }, target);
}

#[allow(dead_code)]
pub fn send_fake_mouse_release(target: Entity, commands: &mut Commands){
    commands.trigger_targets(Pointer::<Released>{
        target: target,
        pointer_id: PointerId::Mouse,
        pointer_location: Location {
            target: NormalizedRenderTarget::Image(
                // This requires to add bevy render to the project
                bevy_render::camera::ImageRenderTarget {
                    handle: Handle::default(),
                    scale_factor: FloatOrd(1.0),
                },
            ),

            position: Vec2::ZERO,
        },
        event: Released {
            button: PointerButton::Primary,
            hit: HitData {
                camera: Entity::PLACEHOLDER,
                depth: 0.0,
                position: None,
                normal: None,
            },
        },

    }, target);
}

#[allow(dead_code)]
pub fn send_fake_mouse_over(target: Entity, commands: &mut Commands){
    commands.trigger_targets(Pointer::<Over>{
        target: target,
        pointer_id: PointerId::Mouse,
        pointer_location: Location {
            target: NormalizedRenderTarget::Image(
                // This requires to add bevy render to the project
                bevy_render::camera::ImageRenderTarget {
                    handle: Handle::default(),
                    scale_factor: FloatOrd(1.0),
                },
            ),

            position: Vec2::ZERO,
        },
        event: Over {
            hit: HitData {
                camera: Entity::PLACEHOLDER,
                depth: 0.0,
                position: None,
                normal: None,
            },
        },

    }, target);
}

#[allow(dead_code)]
pub fn send_fake_mouse_out(target: Entity, commands: &mut Commands){
    commands.trigger_targets(Pointer::<Out>{
        target: target,
        pointer_id: PointerId::Mouse,
        pointer_location: Location {
            target: NormalizedRenderTarget::Image(
                // This requires to add bevy render to the project
                bevy_render::camera::ImageRenderTarget {
                    handle: Handle::default(),
                    scale_factor: FloatOrd(1.0),
                },
            ),

            position: Vec2::ZERO,
        },
        event: Out {
            hit: HitData {
                camera: Entity::PLACEHOLDER,
                depth: 0.0,
                position: None,
                normal: None,
            },
        },

    }, target);
}