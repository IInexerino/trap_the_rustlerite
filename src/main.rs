use bevy::{ prelude::*, window::{WindowResolution} };

use crate::{ game::game::GamePlugin, menu::menu::MenuPlugin, utils::helper_utils::{scroll_zoom_camera_system, toggle_resolution}};

mod utils;
mod menu;
mod game;

fn main() {
    let mut app = App::new();

    let window_resolution = WindowResolution::new(960., 540. );

    app.add_plugins((
        DefaultPlugins
            .set(
                WindowPlugin{
                    primary_window: 
                        Some(Window{
                            title: "Trap the Tiger".into(),
                            resolution: window_resolution,
                            ..Default::default()
                        }), 
                        ..Default::default()
                }
            )
            .set( ImagePlugin::default_nearest() ),
        MenuPlugin,
        GamePlugin
    ));

    app.add_systems(Update, (toggle_resolution, scroll_zoom_camera_system));

    app.run();
}

