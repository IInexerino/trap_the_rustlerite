use bevy::{core_pipeline::core_2d::Camera2d, ecs::{event::EventReader, query::With, system::Single}, input::mouse::MouseWheel, render::camera::Projection};

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