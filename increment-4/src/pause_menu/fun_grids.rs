use bevy::{ecs::{relationship::RelatedSpawner, spawn::SpawnWith}, prelude::*};

use crate::{directional::DirectionalNavigator, dirq};
use super::shared_widgets::{hover_observer, out_observer, pause_menu_button_widget, pressed_observer};

#[derive(Component)]
pub struct FunGridTag;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub enum FunGridTags{
    TopA,
    TopB,
    TopC,

    MidA,
    MidB,

    BotA,
    BotB,
    BotC,
}

pub fn spawn_fun_grid(
    mut commands: Commands,
){
    commands.spawn((
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceEvenly,
            ..Default::default()
        },
        DirectionalNavigator::new(
            [
                [
                    (FunGridTags::TopA, dirq!(South, East)),
                    (FunGridTags::TopB, dirq!(South, East, West)),
                    (FunGridTags::TopC, dirq!(South, West))
                ],
                [
                    (FunGridTags::MidA, dirq!(South, North, East)),
                    (FunGridTags::MidB, dirq!(South, North, West)),
                    (FunGridTags::MidB, dirq!(South, North, West))
                ],
                [
                    (FunGridTags::BotA, dirq!(North, East)),
                    (FunGridTags::BotB, dirq!(North, East, West)),
                    (FunGridTags::BotC, dirq!(North, West))
                ]
            ]
        ),
        FunGridTag,
        children![ 
            (
                Node {
                    width: Val::Percent(80.),
                    height: Val::Percent(80.),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::SpaceEvenly,
                    ..Default::default()
                },
                children![
                    row_widget(top_row),
                    row_widget(mid_row),
                    row_widget(bot_row),
                ]
            )
        ]
    ));
}


fn row_widget<F: FnOnce(&mut RelatedSpawner<'_, bevy::prelude::ChildOf>) + Send + Sync + 'static>(childrens: F) -> impl Bundle{
    (
        Node{
            width: Val::Percent(100.),
            ..Default::default()
        },
        Children::spawn(SpawnWith(childrens))
    )
}
macro_rules! new_btn {
    ($parent:expr, $text:tt, $comp:ident) => {
        $parent.spawn(pause_menu_button_widget($text, FunGridTags::$comp))
            .observe(|_trigger: Trigger<Pointer<Released>>|{
                info!("clicked!");
            })
            .observe(hover_observer)  
            .observe(out_observer)
            .observe(pressed_observer);
    }
}


fn top_row(parent: &mut RelatedSpawner<ChildOf>){
    new_btn!(parent, "TopA", TopA);
    new_btn!(parent, "TopB", TopB);
    new_btn!(parent, "TopC", TopC);
}

fn mid_row(parent: &mut RelatedSpawner<ChildOf>){

    new_btn!(parent, "MidA", MidA);
    new_btn!(parent, "MidB", MidB);
}

fn bot_row(parent: &mut RelatedSpawner<ChildOf>){
    new_btn!(parent, "BotA", BotA);
    new_btn!(parent, "BotB", BotB);
    new_btn!(parent, "BotC", BotC);
}