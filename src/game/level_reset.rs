use bevy::{color::Color, ecs::{entity::Entity, query::With, system::{Commands, Query, Res, ResMut, Single}}, log::debug, sprite::Sprite, state::state::NextState, time::Time};
use crate::{game::{game::{CurrentLevel, TotalGameStats}, level_setup::{RustaceanPos, TrapTile}, levels::{LevelEndTimer, LevelState}}, utils::hexgrid_utils::GridTilePos};

pub fn reset_level(
    tile_q: Query<(Entity, &mut Sprite), With<GridTilePos>>,
    rustacean_q: Single<Entity, With<RustaceanPos>>,
    mut commands: Commands,
    mut total_stats: ResMut<TotalGameStats>,
    mut current_level: ResMut<CurrentLevel>,
    mut level_end_timer: ResMut<LevelEndTimer>,
    mut level_st: ResMut<NextState<LevelState>>,
    time: Res<Time>,
) {
    level_end_timer.0.tick(time.delta());

    
    debug!("checking timer");
    if level_end_timer.0.elapsed_secs() >= 3.0 {
        commands.remove_resource::<LevelEndTimer>();
        for (entity, mut sprite) in tile_q {
            if sprite.color != Color::WHITE {
                sprite.color = Color::WHITE;
            }
                commands.entity(entity).remove::<TrapTile>();
        }

        let rustacean_entity = rustacean_q.into_inner();

        commands.entity(rustacean_entity).despawn(); 

        debug!("adding level");
        current_level.0 += 1;

        if current_level.0 > total_stats.record_level {
            total_stats.record_level = current_level.0
        }
        
        level_st.set(LevelState::InLevel);
    }
}