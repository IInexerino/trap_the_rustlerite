use std::path::PathBuf;

use anyhow::Context;
use bevy::{prelude::*};
use crate::{ game::{level_reset::reset_level, level_setup::{prepare_tile_traps, spawn_rustacean, tile_observer}, levels::{despawn_current_stats, goto_main_menu, level_text_update, reset_current_level_taps, run_levelend_timer, set_player_turn, spawn_current_stats_text, LevelState, TurnState}, pathfinding::pathfind_and_move}, utils::hexgrid_utils::{get_startup_hexgrid, GridSize, GridTilePos, HexGridOrientation, HextileF2FSize}};

const GRID_COLS: i32 = 7;
const GRID_ROWS: i32 = 12;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::app::App) {

        app.insert_state(AppState::MainMenu);

        // no logging developed yet
        app.insert_resource(
            TotalGameStats {
                record_level: 1,
                ..Default::default()
            }
        );

        app.add_systems(Startup, (spawn_camera, setup_total_game_stats));

        app.init_state::<LevelState>();
        app.add_systems(
            OnEnter(AppState::InGame), 
            (
                get_startup_hexgrid(
                    Vec3::new(0., 0., 0.),
                    GridSize::new(GRID_COLS, GRID_ROWS),
                    HexGridOrientation::Vertical,
                    HextileF2FSize(90.0),
                    Some("hex.png")
                ),
                add_clicking_observers_to_tiles,
                reset_current_level,
                spawn_current_stats_text,
                start_new_level,
            ).before(spawn_rustacean).before(prepare_tile_traps).chain()
        )
        .add_systems(Update, level_text_update.run_if(in_state(AppState::InGame)))
        .add_systems(
            OnExit(AppState::InGame),
            despawn_current_stats
        );

        app.init_state::<TurnState>();
        app.add_systems(
            OnEnter(LevelState::InLevel), 
            (
                spawn_rustacean,
                prepare_tile_traps,
                reset_current_level_taps,
                set_player_turn,
            ).chain()
        );
        app.add_systems(
            OnEnter(LevelState::LevelWin), 
                (run_levelend_timer, save_total_game_stats)
        )
        .add_systems(
            Update, 
            reset_level
                .before(spawn_rustacean)
                .before(prepare_tile_traps)
                .run_if(in_state(LevelState::LevelWin))
        )
        .add_systems(
            OnEnter(LevelState::LevelLose), 
                (run_levelend_timer, save_total_game_stats)
        )
        .add_systems(
            Update, 
            goto_main_menu
                .run_if(in_state(LevelState::LevelLose))
                .chain()
        );

        app.add_systems(
            OnEnter(TurnState::RustaceanTurn), 
                pathfind_and_move
        );
    }
}

#[derive(States, Clone, Debug, Hash, Eq, PartialEq)]
pub enum AppState {
    MainMenu,
    InGame,
}

#[derive(States, Default, Clone, Debug, Hash, Eq, PartialEq)]
pub enum MenuState{
    #[default]
    Disabled,
    Main,
    Stats,
    Quit
}

#[derive(Resource)]
pub struct CurrentLevel(pub u32);


#[derive(Resource, Default, serde::Serialize, serde::Deserialize)]
pub struct TotalGameStats {
    pub tiles_tapped: u64,
    pub tigers_trapped: u64,
    pub tigers_escaped: u64,
    pub games_played: u64,
    pub record_level: u64,
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn setup_total_game_stats(
    mut commands: Commands,
) {
    let game_stats = if !PathBuf::from(r"./configs").exists() {
        std::fs::create_dir("./configs")
            .expect("Error: User should have permission to the directory location\nPath should not yet exist");

        let tgame_stats = TotalGameStats {
            record_level: 1,
            ..Default::default()
        };

        let json = serde_json::to_string_pretty(&tgame_stats)
            .expect("Error: Implementation of Serialize must not decide to fail\nT should contain a map with string keys");

        std::fs::write(PathBuf::from(r"./configs/stats.json"), json)
            .expect("Error: Directory ./configs must exist");

        tgame_stats
    } else {
        let json = std::fs::read_to_string("./configs/stats.json")
            .expect("Error: Path must exist\nContents of the file must be valid utf8");

        let tgame_stats: TotalGameStats = serde_json::from_str(&json)
            .expect("Error: the data must be possible to be parsed into TotalGameStats");

        tgame_stats
    };

    commands.insert_resource(game_stats);
}

pub fn save_total_game_stats(
    game_stats: Res<TotalGameStats>
) {
    let json = serde_json::to_string_pretty(game_stats.into_inner())
        .expect("Error: Implementation of Serialize must not decide to fail\nT should contain a map with string keys");

    std::fs::write(PathBuf::from(r"./configs/stats.json"), json)
        .context("Writing game stats to json file at end of level")
        .expect("Error: Directory ./configs must exist");
}

fn reset_current_level(
    mut commands: Commands,
) {
    commands.insert_resource(CurrentLevel(1));
}

fn start_new_level(
    mut next_level_state: ResMut<NextState<LevelState>>,
    mut game_stats: ResMut<TotalGameStats>
) {
    next_level_state.set(LevelState::InLevel);
    game_stats.games_played += 1;
}

fn add_clicking_observers_to_tiles(
    tile_q: Query<Entity, With<GridTilePos>>,
    mut commands: Commands,
) {
    for entity in tile_q {
        debug!("preparing tiles: adding observers to tile");
        commands.entity(entity).observe(tile_observer());
    }
}

/* 
fn update_total_time(
    time: Res<Time>,
    mut game_stats: ResMut<TotalGameStats>,
) {
    let time_elapsed = time.elapsed_secs() as i32;

    let time_played = if time_elapsed <= 60 {
        String::from(
            format!("{time_elapsed} secs")
        )
    } else if time_elapsed <= 60*60 {
        String::from(
            format!("{time_elapsed} mins")
        )
    } else {
        String::from(
            format!("{time_elapsed} hours")
        )
    };

    game_stats.time_played = time_played;
}
*/