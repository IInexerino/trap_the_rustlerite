use bevy::{ ecs::{ component::Component, entity::Entity, query::With, resource::Resource, system::{Commands, Query, Res, ResMut, Single}, world::World }, state::state::{ NextState, States}, text::{TextFont, TextSpan}, time::{Stopwatch, Time}, ui::widget::Text, utils::default};
use crate::{game::{game::{AppState, CurrentLevel}, level_setup::RustaceanPos}, utils::hexgrid_utils::GridSize};

#[derive(Component)]
pub struct LevelText;

#[derive(Resource)]
pub struct LevelEndTimer(pub Stopwatch);

#[derive(Resource)]
pub struct CurrentLevelTaps(pub u32);

#[derive(States, Default, Clone, Debug, Hash, Eq, PartialEq)]
pub enum LevelState {
    #[default]
    OutOfLevel,
    InLevel,
    LevelWin,
    LevelLose,
}

#[derive(States, Default, Clone, Debug, Hash, Eq, PartialEq)]
pub enum TurnState {
    #[default]
    PlayerTurn,
    RustaceanTurn,
}

pub fn spawn_current_stats_text(
    world: &mut World,
) {
    world.spawn((
        Text::new("Level: "),        
        TextFont {
            font_size: 42.0,
            ..default()
        },
        LevelText,
    )).with_child((
        TextSpan::default(),
        TextFont {
            font_size: 33.0,
            ..default()
        },
        LevelText
    ));
}

pub fn despawn_current_stats(
    mut commands: Commands,
    text_q: Query<Entity, With<LevelText>>
) {
    for entity in text_q.iter() {
        commands.entity(entity).despawn();
    }

}

pub fn level_text_update(
    current_level: Res<CurrentLevel>,
    mut query: Query<&mut TextSpan, With<LevelText>>,
) {
    let level = current_level.0.to_string();
    for mut span in &mut query {
        if level != span.0 {
            **span = format!("{level}")
        }
    }

}

pub fn run_levelend_timer(
    mut commands: Commands
) {
    commands.insert_resource(LevelEndTimer(Stopwatch::new()));
}

pub fn set_player_turn(
    mut turn_st: ResMut<NextState<TurnState>>,
) {
    turn_st.set(TurnState::PlayerTurn);
}

pub(crate) fn reset_current_level_taps(
    mut commands: Commands,
) {
    commands.insert_resource(CurrentLevelTaps(0));
}

pub fn goto_main_menu(
    mut commands: Commands,
    grid_q: Single<Entity, With<GridSize>>,
    rustacean_q: Single<Entity, With<RustaceanPos>>,
    mut level_st: ResMut<NextState<LevelState>>,
    mut app_st: ResMut<NextState<AppState>>,
    mut level_end_timer: ResMut<LevelEndTimer>,
    time: Res<Time>,
) {
    level_end_timer.0.tick(time.delta());

    if level_end_timer.0.elapsed_secs() >= 2.0 {
        commands.remove_resource::<LevelEndTimer>();
        
        commands.entity(grid_q.into_inner()).despawn();
        commands.entity(rustacean_q.into_inner()).despawn();

        level_st.set(LevelState::OutOfLevel);
        app_st.set(AppState::MainMenu);
        
    }

}