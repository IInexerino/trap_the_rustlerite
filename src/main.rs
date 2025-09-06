use bevy::{ prelude::*, window::{WindowMode, WindowResolution} };

use crate::{ game::game::GamePlugin, menu::menu::MenuPlugin, utils::helper_utils::toggle_resolution};

mod utils;
mod menu;
mod game;

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins
            .set(
                WindowPlugin{
                    primary_window: 
                        Some(Window{
                            title: "Trap the Tiger".into(),
                            position: WindowPosition::Centered(MonitorSelection::Primary),
                            resolution: WindowResolution::new(1920., 1080.),
                            mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                            ..Default::default()
                        }), 
                        ..Default::default()
                }
            )
            .set( ImagePlugin::default_nearest() ),
        MenuPlugin,
        GamePlugin
    ));

    app.add_systems(Update, toggle_resolution);

    app.run();
}

