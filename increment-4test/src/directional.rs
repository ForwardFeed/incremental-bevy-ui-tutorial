use std::fmt::Debug;

use bevy::{
    math::{
        CompassQuadrant,
    }, prelude::*, 
};
use leafwing_input_manager::prelude::ActionState;

use crate::{actions::GeneralActions, fake_input::{send_fake_mouse_out, send_fake_mouse_over, send_fake_mouse_press, send_fake_mouse_release}};

#[derive(Debug, Clone, Copy)]
pub struct Directions{
    north: bool,
    south: bool,
    east:  bool,
    west:  bool
}

// dirq for Direction Quadrant
#[macro_export]
macro_rules! dirq {
    () => {
        compile_error!("No input provided!");
    };
    ($( $x:ident ),*) => {
        $crate::directional::Directions::new(&[$(
            bevy::math::CompassQuadrant::$x,
        )*])
    };
}

impl Directions{
    /* pub fn all()-> Self{
        dirq!(South, North, West, East)
    }
    pub fn north_south()-> Self{
        dirq!(South, North)
    }
    pub fn east_west()-> Self{
        dirq!(West, East)
    } */
    pub fn new(x: &[CompassQuadrant]) -> Self{
        x.iter().fold(Directions{ north: false, south: false, east: false, west: false }, |mut acc, item|{
            match item {
                CompassQuadrant::North => acc.north = true,
                CompassQuadrant::South => acc.south = true,
                CompassQuadrant::East => acc.east = true,
                CompassQuadrant::West => acc.west = true,
            };
            acc
        })
        
    }
}

#[derive(Component)]
pub struct DirectionalNavigator<A: Component>{
    coords: Vec2,
    init: bool,
    rows: Vec<Vec<(A, Directions, Option<Entity>)>>,
}

impl<A: Component + Clone + PartialEq> DirectionalNavigator<A>{
    pub fn new<I: IntoIterator<Item = (A, Directions)>>(direction_navigator: impl IntoIterator<Item = I>) -> Self{
        let rows = direction_navigator.into_iter().fold(vec![], |mut acc, item|{
            let row = item.into_iter().fold(vec![], |mut acc, (comp, dir)|{
                acc.push((comp, dir, None));
                acc
            });
            acc.push(row);
            acc
        });
        DirectionalNavigator { rows, init: false, coords: Vec2::default()}
    }

    fn init_next_frame(&mut self) -> &mut Self{
        self.init = false;
        self
    }

    fn init(&mut self, q_targets: Query<(Entity, &A)>){
        self.coords = Vec2(0, 0);
        for (entity, comp) in q_targets{
            
        }
    }

    fn find(&self) -> std::result::Result<(A, Directions, usize, usize, usize, usize), ()> {
        let mut offset = self.target_id;
        let mut n_row = 0;
        let n_max_row = self.rows.len();
        for (_row_id, row) in self.rows.iter().enumerate(){
            // small optimisation
            if row.len() - 1 > offset{
                offset += row.len() - 1;
                continue;
            } 
            let mut n_col = 0;
            
            for (block_id, block) in row.iter().enumerate(){
                if block_id == offset{
                    let block_clone = (*block).clone();
                    let n_max_col = row.len();
                    info!("{n_col}, {offset}");
                    return Ok((block_clone.0, block_clone.1, n_row, n_max_row, n_col, n_max_col).clone());
                }
                offset -= 1;
                n_col += 1;
            }
            n_row += 1;
        }
        Err(())
    }
    fn find_id(&self, n_row: usize, n_col: usize,) -> usize{
        let mut id = 0;
        for (row_id, row_data) in self.rows.iter().enumerate(){
            if n_row != row_id{
                id += row_data.len();
                continue
            }  
            let n_max_col = row_data.len();
            if n_col >= n_max_col{
                id += n_max_col
            } else {
                id += n_col;
                break
            }

        }
        id
    }
    fn navigate(&mut self, direction: CompassQuadrant) -> Result<Result<(A, A), A>, ()> {
        let (comp, dir, n_row, n_max_row, n_col, n_max_col) = match self.find(){
            Ok(x) => {
                x
            },
            Err(_) => {
                return Err(());
            },
        };
        
        macro_rules! target {
            ($dir:ident, $row:expr, $col:expr) => {
                if dir.$dir{
                    self.target_id = self.find_id($row, $col);
                    for _i in $col..n_max_col{
                        match self.find(){
                            Ok(x)=>{
                                if (x.0 != comp){
                                    return Ok(Ok((x.0, comp)))
                                }
                            },
                            Err(_)=>{
                                return Err(())
                            }
                        }
                    }
                    return Err(())
                } else {
                    Ok(Err(comp))
                }
            };
        }

        match direction{
            CompassQuadrant::North => {
                target!(north, if n_row == 0 { n_max_row } else { n_row - 1},n_col)
            },
            CompassQuadrant::South => {
                target!(south, if n_row == n_max_row { 0 } else { n_row + 1},n_col)
            },
            CompassQuadrant::East => {
                target!(east, n_row , if n_col == n_max_col { 0 } else { n_col + 1})
            },
            CompassQuadrant::West => {
                target!(west, n_row , if n_col == 0 { n_max_col } else { n_col - 1})
            },
        }
    }
}



pub fn navigation<T: Component + Debug + Clone + PartialEq>(
    pause_actions: Single<&ActionState<GeneralActions>>,
    mut navigator: Single<&mut DirectionalNavigator<T>>,
    query_targets: Query<(Entity, &T)>,
    mut commands: Commands,
){
    macro_rules! find_entity_and_do {
        ($target:expr, $func:expr) => {
            for (entity, target_comp) in query_targets{
                if *target_comp == $target{
                    $func(entity, &mut commands);
                    break
                }
            }
        };
    }
    if !navigator.init{
        match navigator.find() {
            Ok(x) => {
                find_entity_and_do!(x.0, send_fake_mouse_over)
            },
            Err(_) => {
                panic!("The navigator failed to initialize");
            },
        }
        navigator.init = true
    }
    // If the user is pressing both left and right, or up and down,
    // it should not move in either direction.
    let net_east_west = pause_actions
        .just_pressed(&GeneralActions::MoveRight) as i8
        - pause_actions
        .just_pressed(&GeneralActions::MoveLeft) as i8;

    let net_north_south = pause_actions
        .just_pressed(&GeneralActions::MoveUp) as i8
        - pause_actions
        .just_pressed(&GeneralActions::MoveDown) as i8;
    
    // Compute the direction that the user is trying to navigate in
    // I'll use a CompassOctan later.
    let maybe_direction = match (net_east_west, net_north_south) {
        (0, 0) => None,
        (0, 1) => Some(CompassQuadrant::North),
        (1, 1) => Some(CompassQuadrant::North), // would be north east
        (1, 0) => Some(CompassQuadrant::East),
        (1, -1) => Some(CompassQuadrant::East), // would be south east
        (0, -1) => Some(CompassQuadrant::South),
        (-1, -1) => Some(CompassQuadrant::South), // would be south west
        (-1, 0) => Some(CompassQuadrant::West),
        (-1, 1) => Some(CompassQuadrant::West), // would be nord west
        _ => None,
    };
    let (current, old) = match maybe_direction {
        Some(direction) =>{ 
            match navigator.navigate(direction){
                Ok(opt_navigation) => {
                    match opt_navigation {
                        Ok((current, old)) => {
                            (current, Some(old))
                        },
                        Err(current) => {
                            (current, None)
                        },
                    }
                },
                Err(_) => {
                    error!("The navigation went out of bounds, please fix it, reseting to 0");
                    navigator.target_id = 0;
                    return;
                },
            }
        },
        None => {
            match navigator.find() {
                Ok(x) => {
                    (x.0, None)
                },
                Err(_) => {
                    error!("Error in navigation, cannot find current target");
                    navigator.target_id = 0;
                    return;
                },
            }
        },
    };

    if let Some(old) = old{
        find_entity_and_do!(old, send_fake_mouse_out);
        find_entity_and_do!(current, send_fake_mouse_over);
    }
    
    if pause_actions.just_pressed(&GeneralActions::Accept){
        find_entity_and_do!(current, send_fake_mouse_press);
    }
    
    if pause_actions.just_released(&GeneralActions::Accept){
        find_entity_and_do!(current, send_fake_mouse_release);
    }
    
}