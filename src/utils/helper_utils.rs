use bevy::{core_pipeline::core_2d::Camera2d, ecs::{ event::EventReader, query::With, system::{Res, Single}}, input::{keyboard::KeyCode, mouse::MouseWheel, ButtonInput}, log::error, render::camera::Projection, window::{MonitorSelection, Window, WindowMode}};

pub fn scroll_zoom_camera_system(
        mut evr_scroll: EventReader<MouseWheel>,
        mut query_camera: Single<&mut Projection, With<Camera2d>> 
) {{
        if let Some(mouse_wheel) = evr_scroll.read().next() {
            match query_camera.as_mut() {
                Projection::Orthographic(ortho) => {
                    // Alter the zoom
                    let new_ortho_scale = ortho.scale + -(mouse_wheel.y * 0.05);

                    let min = 0.01_f32;
                    let max = 5.0_f32;

                    if new_ortho_scale >= min && new_ortho_scale <= max {
                        ortho.scale = new_ortho_scale;
                    }
                }
                _ => {
                    eprintln!("Scrolling Error: Projection is not Orthograpic as should be by Default");
                }
            }
        }
    }
}

pub fn toggle_resolution(
    keys: Res<ButtonInput<KeyCode>>,
    mut window: Single<&mut Window>,
) {
    if keys.just_pressed(KeyCode::F11) {
        match window.mode {
            bevy::window::WindowMode::Windowed => {
                window.mode = WindowMode::BorderlessFullscreen(MonitorSelection::Primary);
            }
            bevy::window::WindowMode::BorderlessFullscreen(_) => window.mode = WindowMode::Windowed,
            _ => error!("Window is in invalid mode")
        }
    }
}