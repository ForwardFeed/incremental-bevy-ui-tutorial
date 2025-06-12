use bevy::{ecs::{relationship::RelatedSpawner, spawn::{SpawnIter, SpawnWith}}, prelude::*};

use crate::common_button;

#[derive(Component)]
struct GridMarkerData{
    // The reason I'm tracking the data here and not inside the 
    // grid_template_rows or grid_template_columns, is because as of now
    // the number of repetition or fraction isn't publically available. (it's in pub(crate))
    col_n: u16,
    row_n: u16,
}
impl Default for GridMarkerData{
    fn default() -> Self {
        GridMarkerData { col_n: 3, row_n: 3 }
    }
}
pub fn grid_ui() -> impl Bundle{
    (
        Node{
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            // Absolutely need to say that we want a grid
            display: Display::Grid,
            // repetition here is the number of boxes in a row or colum
            // fr => fraction => https://developer.mozilla.org/en-US/docs/Web/CSS/flex_value
            grid_template_rows: vec![
                RepeatedGridTrack::flex(1, 0.2), //0.2 means 20% of the space
                RepeatedGridTrack::flex(1, 1.0)
            ],
            ..Default::default()
        },
        children![
            grid_control(),
            grid_body(),
        ]
    )
}

fn grid_body() -> impl Bundle{
    let markerdata = GridMarkerData::default();
    (
        Node{
            // if you ever try to give height or width to children of a grid
            // it's going to mess with the parent grid display
            // It will not work like with flex.
            display: Display::Grid,
            grid_template_columns: RepeatedGridTrack::flex(markerdata.col_n, 1.0),
            grid_template_rows: RepeatedGridTrack::flex(markerdata.row_n, 1.0),
            row_gap: Val::Px(12.0),
            column_gap: Val::Px(12.0),
            ..Default::default()
        },
        markerdata,
        // SpawnIter is a bevy utils useful to spawn with an iterator
        Children::spawn(SpawnIter(
            (0..12).into_iter().map(|index|{
                grid_element_ui(index + 1)
            }),
        ))
    )
}

fn grid_element_ui(index: i32) -> impl Bundle{
    (
        Node{
            border: UiRect::all(Val::Px(2.0)),
            ..Default::default()
        },
        BorderColor(color_from_index(index as f32, true)),
        BackgroundColor(color_from_index(index as f32, false)),
        children![
            Text::new(index.to_string())
        ]
        
    )
}
// d for darker tone
fn color_from_index(n: f32, d: bool) -> Color{
    // darker tone as fload
    let df = if d {-0.2}else{0.0};
    let r = (n * 0.1 % 1.0) - df;
    let g = (n * 0.3 % 1.0) - df;
    let b = (n * 0.6 % 1.0) - df; 
    return Color::srgb(r, g, b)
}

fn grid_control() -> impl Bundle{
    (
        Node{
            ..Default::default()
        },
        Children::spawn(SpawnWith(|parent: &mut RelatedSpawner<ChildOf>|{
            common_button!(parent, "+1 col", observer_col_p);
            common_button!(parent, "-1 col", observer_col_m);
            common_button!(parent, "+1 row", observer_row_p);
            common_button!(parent, "-1 row", observer_row_m);
        }))
    )
}

fn observer_col_p(_trigger: Trigger<Pointer<Released>>, query: Single<(&mut Node, &mut GridMarkerData), With<GridMarkerData>>){
    let (mut node, mut grid_data) = query.into_inner();
    grid_data.col_n += 1;
    for col in &mut node.grid_template_columns{
        *col = RepeatedGridTrack::flex(grid_data.col_n, 1.0)
    }
}

fn observer_col_m(_trigger: Trigger<Pointer<Released>>, query: Single<(&mut Node, &mut GridMarkerData), With<GridMarkerData>>){
    let (mut node, mut grid_data) = query.into_inner();
    // pay attention not to go into the negative or it's going to crash. (idk about 0)
    if grid_data.col_n != 1{
        grid_data.col_n -= 1;
    }
    for col in &mut node.grid_template_columns{
        *col = RepeatedGridTrack::flex(grid_data.col_n, 1.0)
    }
}

fn observer_row_p(_trigger: Trigger<Pointer<Released>>, query: Single<(&mut Node, &mut GridMarkerData), With<GridMarkerData>>){
    let (mut node, mut grid_data) = query.into_inner();
    grid_data.row_n += 1;
    for row in &mut node.grid_template_rows{
        *row = RepeatedGridTrack::flex(grid_data.row_n, 1.0)
    }
}

fn observer_row_m(_trigger: Trigger<Pointer<Released>>, query: Single<(&mut Node, &mut GridMarkerData), With<GridMarkerData>>){
    let (mut node, mut grid_data) = query.into_inner();
    // pay attention not to go into the negative or it's going to crash. (idk about 0)
    if grid_data.row_n != 1{
        grid_data.row_n -= 1;
    }
    for row in &mut node.grid_template_rows{
        *row = RepeatedGridTrack::flex(grid_data.row_n, 1.0)
    }
}

