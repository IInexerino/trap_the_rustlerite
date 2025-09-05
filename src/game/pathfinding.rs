use std::{collections::{HashMap, HashSet, VecDeque}, fmt::Display};
use anyhow::Context;
use bevy::{color::{palettes::css::{LIGHT_GREEN, RED}, Color}, ecs::{query::{With, Without}, system::{Query, ResMut}}, math::Vec2, sprite::Sprite, state::state::NextState, transform::components::{GlobalTransform, Transform}};
use crate::{game::{game::TotalGameStats, level_setup::{RustaceanPos, TrapTile}, levels::{LevelState, TurnState}}, utils::hexgrid_utils::{get_hex_horizontal_neighbor_pos, GridSize, GridTilePos, HexGridOrientation, HextileF2FSize}};

pub fn coord_to_world(x: i32, y: i32, grid_size: &GridSize, hextile_f2f_size: &HextileF2FSize, orientation: &HexGridOrientation) -> Vec2 {
    match orientation {
        HexGridOrientation::Vertical => {
            let hextile_width = hextile_f2f_size.0;
            let hextile_height = hextile_f2f_size.to_height();

            let x = (
                (x as f32 * hextile_width * 0.75) 
                - 
                ((grid_size.cols / 2) as f32 * hextile_width * 0.75 - hextile_width * 0.375)
            ) - (
                if grid_size.cols % 2 != 0 {
                    hextile_width * 0.375
                } else { 
                    0.0 
                }
            );

            let y = (
                y as f32 * hextile_height - 
                (
                    if x as i32 % 2 != 0 { 
                        hextile_height / 2.0 
                    } else { 
                        0.0 
                    }
                )
            ) - (
                (grid_size.rows / 2) as f32 * hextile_height - (hextile_height / 4.0)
            ) - (
                if grid_size.rows % 2 != 0 {
                    hextile_height / 2.0
                } else { 
                    0.0 
                }
            );
            
            return Vec2::new(x,y);
        },
        HexGridOrientation::Horizontal => {
            println!("Error: not functional yet, fucking wait");
            panic!()
        }
    }
}

/// Get the coordiantes of all the out of bound pseudo-tiles which are adjascent to the grid border tiles 
pub fn escape_targets(grid_size: &GridSize) -> Vec<(i32, i32)> {
    let mut escapes = Vec::new();

    for x in 0..grid_size.cols {
        for y in 0..grid_size.rows {
            let pos = GridTilePos::new(x, y);
            // iterates over x y and creates a GridTilePos to then check if .is_border()
            if pos.is_border(grid_size) {
                // if it .is_border() -> we iterate over its neighbors
                for n in pos.get_neighbor_pos() {
                    if !in_bounds(n, grid_size) {
                        // if its neighbors are out of grid bounds we push them into the escapes
                        escapes.push(n);
                    }
                }
            }
        }
    }

    // remove duplicates just in case
    escapes.sort_unstable();
    escapes.dedup();
    escapes
}

pub fn in_bounds(pos: (i32, i32), grid_size: &GridSize) -> bool {
    pos.0 >= 0
        && pos.0 < grid_size.cols as i32
        && pos.1 >= 0
        && pos.1 < grid_size.rows as i32
}

pub enum Path {
    Escaped(i32,i32),
    Found(Vec<(i32,i32)>),
    NotFound,
}

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Escaped(x, y) => {
                write!(f, "Path::Escaped({}, {})", x, y)
            }
            Self::Found(vec) => {
                write!(f, "Path::Found(\n{:?}\n)", vec)
            }
            Self::NotFound => {
                write!(f, "Path::NotFound")
            }
        }
        
    }
}

/// BFS that treats "escape positions" (which may be out-of-bounds) as valid goals.
/// When a neighbor is an escape, we immediately reconstruct a path that *ends* in that escape
/// coordinate and return it.
pub fn pathfind(
    rustacean_pos: &RustaceanPos,
    grid_size: &GridSize,
    tile_q: Query<&GridTilePos, Without<TrapTile>>,
) -> (Path, [(i32, i32); 6], Option<HashSet<(i32,i32)>>) {
    // starting position of rustacean
    let start = (rustacean_pos.x, rustacean_pos.y);
    // gets an array of coordinates of all adjascent hexagons (whether in bounds or not)
    let start_neighbors = rustacean_pos.get_neighbor_pos();
    // we take the out of bounds neighbor tiles, and put them in a hashset
    let escape_set: HashSet<_> = escape_targets(grid_size).into_iter().collect();

    // quick check: if any immediate neighbor of the rustacean at the start is an escape, return that (escaped)
    for &(nx, ny) in &start_neighbors {
        if escape_set.contains(&(nx, ny)) {
            return (Path::Escaped(nx, ny), start_neighbors, None);
        }
    }

    // collect open tiles
    let open_tiles: HashSet<(i32, i32)> =
        tile_q.iter().map(|p| (p.x, p.y) ).collect();

    // BFS init: mark start visited
    let mut queue: VecDeque<(i32, i32)> = VecDeque::new();
    // collection of all arrivals and Option<departures>
    let mut came_from: HashMap<(i32, i32), Option<(i32, i32)>> = HashMap::new();
    came_from.insert(start, None);

    // enqueue neighbors of start first (so we explore immediate moves first)
    // they all go into the came_from hashmap, but with different lineages
    for &(nx, ny) in &start_neighbors {
        // only enqueue if it's walkable (in open_tiles) and not visited
        if open_tiles.contains(&(nx, ny)) && !came_from.contains_key(&(nx, ny)) {
            came_from.insert((nx, ny), Some(start));
            queue.push_back((nx, ny));
        }
    }

    // BFS loop
    while let Some(current) = queue.pop_front() {
        // pop the queue and get the current coordinates
        for n in get_hex_horizontal_neighbor_pos(current.0, current.1).iter().copied() {
            // get neighbors of the current coordinate
            if escape_set.contains(&n) {
                // If a neighbor is in the escape coords list, end the operation and return the reconstructed path ending in that escape coord
                came_from.insert(n, Some(current));
                // set departure for the escape node in came_from so reconstruction works

                // reconstruct path from start -> ... -> current -> escape(n)
                let mut path = Vec::new();
                let mut cur = Some(n);
                while let Some(p) = cur {
                    path.push(p);
                    cur = if !(came_from[&p] == Some(start)) {
                        came_from[&p]
                    } else {
                        None
                    }
                }
                path.reverse();

                return (Path::Found(path), start_neighbors, None);
                // I am the best programmer
            }

            // otherwise, standard BFS into in-bounds open tiles
            if open_tiles.contains(&n) && !came_from.contains_key(&n) {
                came_from.insert(n, Some(current));
                queue.push_back(n);
            }
        }
    }

    return (Path::NotFound, start_neighbors, Some(open_tiles))
}

pub fn pathfind_and_move(
    mut rustacean_pos_q: Query<(&mut RustaceanPos, &mut Transform)>,
    grid_size_q: Query<(&GridSize, &HextileF2FSize, &HexGridOrientation)>,
    tile_q: Query<&GridTilePos, Without<TrapTile>>,
    tile_transform_q: Query<(&GridTilePos, &GlobalTransform)>,
    mut tile_sprite_q: Query<&mut Sprite, With<GridTilePos>>,
    mut next_turnstate: ResMut<NextState<TurnState>>,
    mut next_levelstate: ResMut<NextState<LevelState>>,
    mut game_stats: ResMut<TotalGameStats>
) {
    let (mut rustacean_pos, mut transform) = rustacean_pos_q.single_mut().context("Looking for a single RustaceanPos from query").unwrap();
    let (grid_size, tile_width, orientation) = grid_size_q.single().context("Looking for a single RustaceanPos from query").unwrap();

    let (path, current_neighbors, open_tiles) = pathfind(&rustacean_pos, grid_size, tile_q);

    match path {
        Path::Found(vec) => {
            for neighbor in current_neighbors {
                if vec.contains(&neighbor) {

                    let mut new_transform = Transform::default(); 

                    for (pos, transform) in tile_transform_q {
                        if pos.x as i32 == neighbor.0 && pos.y as i32 == neighbor.1 {

                            new_transform = transform.compute_transform();
                            new_transform.translation.z = 0.1;

                        }
                    };

                    transform.translation = new_transform.translation;
                    rustacean_pos.x = neighbor.0;
                    rustacean_pos.y = neighbor.1;
                    next_turnstate.set(TurnState::PlayerTurn)
                }
            }
        },
        Path::NotFound => {
            for neighbor in current_neighbors {
                if open_tiles.as_ref().unwrap().contains(&neighbor) {

                    let mut new_transform = Transform::default(); 

                    for (pos, transform) in tile_transform_q {
                        if pos.x as i32 == neighbor.0 && pos.y as i32 == neighbor.1 {

                            new_transform = transform.compute_transform();
                            new_transform.translation.z = 0.1;

                        }
                    };
                    transform.translation = new_transform.translation;

                    rustacean_pos.x = neighbor.0;
                    rustacean_pos.y = neighbor.1;

                    next_turnstate.set(TurnState::PlayerTurn);

                    return
                }
            }

            for mut sprite in tile_sprite_q.iter_mut() {
                sprite.color = Color::Srgba(LIGHT_GREEN);
            }

            next_levelstate.set(LevelState::LevelWin);
            game_stats.tigers_trapped += 1;

        },
        Path::Escaped(x, y) => {
            let mut new_translation = coord_to_world(x, y, grid_size, tile_width, orientation)
                .extend(0.1);
            new_translation.y += tile_width.to_height() / 2.;
            transform.translation = new_translation;

            rustacean_pos.x = x;
            rustacean_pos.y = y;

            game_stats.tigers_escaped += 1;

            for mut sprite in tile_sprite_q.iter_mut() {
                sprite.color = Color::Srgba(RED);
            }

            next_levelstate.set(LevelState::LevelLose);
        },
    }
}