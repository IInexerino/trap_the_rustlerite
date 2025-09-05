use bevy::{asset::AssetServer, color::{palettes::css::DARK_GRAY, Color}, ecs::{component::Component, entity::Entity, observer::Trigger, query::Without, system::{Commands, Query, Res, ResMut, Single}}, math::Vec2, picking::events::{Click, Pointer}, sprite::Sprite, state::state::{NextState, State}, transform::components::GlobalTransform};

use crate::{game::{game::{CurrentLevel, TotalGameStats}, levels::{CurrentLevelTaps, TurnState}}, utils::hexgrid_utils::{get_hex_horizontal_neighbor_pos, GridSize, GridTilePos, HextileF2FSize}};


#[derive(Component)]
/// Position component for the rustacean entity
pub struct RustaceanPos {
    pub x: i32,
    pub y: i32
}

impl RustaceanPos {
    pub fn new(x: i32, y: i32) -> Self {
        RustaceanPos { x, y }
    }

    pub fn get_neighbor_pos(&self) -> [(i32, i32); 6] {
        get_hex_horizontal_neighbor_pos(self.x, self.y)
    }
}

#[derive(Component)]
pub struct TrapTile;

pub fn spawn_rustacean(
    mut commands : Commands,
    asset_server: Res<AssetServer>,
    grid_query: Single<(&HextileF2FSize, &GridSize)>,
    tile_query: Query<(&GridTilePos, &GlobalTransform)>
) {
    println!("spawning_rustacean: getting grid size");
    let (f2f_size, grid_size) = grid_query.into_inner();

    let texture = asset_server.load("rustacean.png");

    let size_mult = 0.75_f32;
    let size = Vec2::new(
        f2f_size.0 * size_mult,
        (f2f_size.0 * size_mult) * 0.667_f32
    );

    println!("spawning_rustacean: getting tile positions");
    for (tile_pos, global_transform) in tile_query.iter() {
        if tile_pos.x == grid_size.cols/2 && tile_pos.y == (grid_size.rows/2) {
            let mut transform = global_transform.compute_transform();
            transform.translation.z = 0.1;

            println!("spawning_rustacean: spawning rustacean");

            commands.spawn((
                RustaceanPos::new(tile_pos.x as i32, tile_pos.y as i32),
                transform,
                Sprite{
                    image: texture.clone(),
                    custom_size: Some(size),
                    ..Default::default()
                }
            ));
        }
    }
}

pub fn prepare_tile_traps(
    mut commands : Commands,
    current_level: Res<CurrentLevel>,
    rustacean_pos: Single<&RustaceanPos>,
    grid_size: Single<&GridSize>,
    mut tile_query: Query<(Entity, &GridTilePos, &mut Sprite)>
) {
    
    println!("preparing tiles: getting grid size");
    let total_tile_amount = grid_size.count();

    println!("preparing tiles: calculating trap number");
    let num_of_traps = ((total_tile_amount as f32 * 0.25974).round() as u32 - 
    (if current_level.0 < 21 { 
        current_level.0 - 1
    } 
    else { 
        20 
    })) as usize;




    println!("preparing tiles: calculating trap positions");
    let trap_positions = loop {

        let trap_positions = find_suitable_trap_positions(num_of_traps, &grid_size, &rustacean_pos);
        // check if the rustacean is blocked in 
        if rustacean_pos.get_neighbor_pos().iter().all(| (x, y) | trap_positions.contains(&(*x, *y)) ) {
            println!("Invalid!!!\nNeighbors: {:?}", rustacean_pos.get_neighbor_pos());
            println!("Traps: {:?}\n> continuing", trap_positions);
            continue
        } else {
            break trap_positions
        }
    };






    println!("preparing tiles: getting the tile entities");
    for (entity, pos, mut sprite) in tile_query.iter_mut() {

        println!("preparing tiles: adding traps to tile ({}, {})",pos.x, pos.y);
        // add traps to some
        if trap_positions.contains(&(pos.x, pos.y)) {
            sprite.color = Color::Srgba(DARK_GRAY);
            commands.entity(entity).insert(
                TrapTile
            );
        }

    }
}

pub fn tile_observer() -> impl FnMut(
    Trigger<Pointer<Click>>,
    Query<(&mut Sprite, &GridTilePos), Without<TrapTile>>,
    Single<&RustaceanPos>,
    ResMut<TotalGameStats>,
    ResMut<CurrentLevelTaps>,
    Res<State<TurnState>>,  
    ResMut<NextState<TurnState>>,
    Commands
) {
    move |  
        trigger: Trigger<Pointer<Click>>,
        mut tile_query: Query<(&mut Sprite, &GridTilePos), Without<TrapTile>>,
        rustacean_pos: Single<&RustaceanPos>,
        mut game_statistics: ResMut<TotalGameStats>,
        mut level_taps: ResMut<CurrentLevelTaps>,
        turn_state: Res<State<TurnState>>,  
        mut next_state: ResMut<NextState<TurnState>>,
        mut commands: Commands
    | {
        // check if its out turn
        if *turn_state != TurnState::PlayerTurn{
            return;
        }

        let tile_entity = trigger.target();
        let tile_query_result = tile_query.get_mut(tile_entity);

        match tile_query_result {
            Ok((mut spr, tile_pos)) => {

                // if the player doesnt tap the tile that the rustacean is on
                if !((tile_pos.x as i32, tile_pos.y as i32) == (rustacean_pos.x, rustacean_pos.y)) {
                    // insert traptile component and color gray
                    commands.entity(tile_entity).insert(TrapTile);
                        spr.color = Color::Srgba(DARK_GRAY);
                    // switch to RustaceanTurn turn state 
                    next_state.set(TurnState::RustaceanTurn);
                    // alter game stats
                    game_statistics.tiles_tapped += 1;
                    level_taps.0 += 1;
                }
            }
            Err(_) => {}
        } 
    }
}

pub fn find_suitable_trap_positions(num_of_traps: usize, grid_size: &GridSize, rustacean_pos: &RustaceanPos) -> Vec<(i32, i32)> {

    let mut trap_positions: Vec<(i32, i32)> = Vec::with_capacity(num_of_traps as usize);

    while trap_positions.len() < num_of_traps { 
        let x = rand::random_range(0..grid_size.cols);
        let y = rand::random_range(0..grid_size.rows);

        // check is selected position is the same as some previous position, 
        // or if its in the position of the rustacean
        if trap_positions.contains(&(x, y)) 
            || (x == rustacean_pos.x && y == rustacean_pos.y) {
            println!("Invalid!!!\nAttempted position: {x}, {y}\n> continuing");
            continue
        }
        trap_positions.push((x, y));
    }

    trap_positions
}
