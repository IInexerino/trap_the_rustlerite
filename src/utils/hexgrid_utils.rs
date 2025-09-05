use bevy::{asset::{AssetServer, Handle}, ecs::{component::Component, hierarchy::ChildOf, system::{Commands, Res}}, image::Image, math::{Vec2, Vec3}, picking::Pickable, render::view::Visibility, sprite::Sprite, transform::components::Transform};

use crate::game::level_setup::tile_observer;

// GRIDS
/// Size of the actual grid in game in columns and rows
#[derive(Component, Default, Clone, Copy, Debug)]
pub struct GridSize{
    pub cols: i32,
    pub rows: i32
}

impl GridSize {
    pub fn new(cols: i32, rows: i32) -> Self {
        GridSize {
            cols,
            rows
        }
    }
    pub fn count(&self) -> usize {
        (self.cols * self.rows) as usize
    }
}

#[derive(Component, Copy, Clone)]
pub struct HextileF2FSize(pub f32);

impl HextileF2FSize {
    pub fn to_height(&self) -> f32 {
        self.0 * 0.866
    }
}


/// Defines the orientation of the HexCells (pointy to the side, or upwards).
#[derive(Component, Clone, Copy)]
pub enum HexGridOrientation {
    /// Straight columns offset by 0.75.
    /// 
    /// Horizontal part of the hexagon is upwards and downwards.
    Vertical,

    /// Straight rows offset by 0.75.
    /// 
    /// Horizontal part of the hexagon is to the left and right.
    Horizontal
}

// GRIDTILES
#[derive(Component, Default, Clone, Copy, Debug)]
pub struct GridTilePos {
    pub x: i32,
    pub y: i32
}

impl GridTilePos {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    /// Converts a tile position (2D) into an index in a flattened vector (1D), assuming the
    /// tile position lies in a tilemap of the specified size.
    pub fn to_index(&self, grid_size: &GridSize) -> usize {
        ((self.y * grid_size.cols) + self.x) as usize
    }

    pub fn is_border(&self, grid_size: &GridSize) -> bool {
        if self.x == 0 
        || self.y == 0
        || self.x == (grid_size.cols - 1)
        || self.y == (grid_size.rows - 1) {
            true
        } else {
            false
        }
    }
    
    pub fn get_neighbor_pos(&self) -> [(i32, i32); 6] {
        let (x, y) = (self.x, self.y);

        if x % 2 == 0 {
            [
                (x, y + 1),
                (x, y - 1),
                (x-1, y),
                (x-1, y + 1),
                (x+1, y),
                (x+1, y + 1),
            ]
        } else {
            [
                (x, y + 1),
                (x, y - 1),
                (x-1, y),
                (x-1, y - 1),
                (x+1, y),
                (x+1, y - 1),
            ]
        }
    }

    pub fn coord_to_world(&self, grid_size: &GridSize, hextile_f2f_size: &HextileF2FSize, orientation: &HexGridOrientation) -> Vec2 {
        match orientation {
            HexGridOrientation::Vertical => {
                let hextile_width = hextile_f2f_size.0;
                let hextile_height = hextile_f2f_size.to_height();

                let x = (
                    (self.x as f32 * hextile_width * 0.75) 
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
                    self.y as f32 * hextile_height - 
                    (
                        if self.x % 2 != 0 { 
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
}


//HELPER TEMPLATES

pub fn startup_hexgrid(mut commands: Commands, asset_server: Res<AssetServer>) {
    
// user cofigurations
    let grid_position = Transform::from_translation(Vec3::new(0., 0., 0.));
    let grid_size = GridSize::new(11, 12);
    let hextile_f2f_size = HextileF2FSize(60.0);
    let orientation = HexGridOrientation::Vertical;
    let default_texture: Handle<Image> = asset_server.load("hex.png");

// template
    let grid_entity = commands.spawn_empty().id();

    for x in 0..grid_size.cols {
        for y in 0..grid_size.rows {
            let tile_pos = GridTilePos::new(x, y);
            let relative_transform = tile_pos
                .coord_to_world(&grid_size, &hextile_f2f_size, &orientation)
                .extend(0.);

            commands.spawn( (
                tile_pos,
                ChildOf(grid_entity),
                Transform::from_translation(relative_transform),
                Sprite{
                    custom_size: Some(Vec2::new(
                        hextile_f2f_size.0,
                        hextile_f2f_size.to_height()
                    )),
                    image: default_texture.clone(),
                    ..Default::default()
                }
            ));
            
        }
    }

    commands.entity(grid_entity).insert((
        grid_size,
        hextile_f2f_size,
        orientation,
        grid_position
    ));
}

pub fn get_startup_hexgrid(
    position: Vec3,
    size: GridSize,
    orientation: HexGridOrientation,
    hextile_f2f_size: HextileF2FSize,
    default_texture: Option<&str>,
) -> impl FnMut(Commands, Res<AssetServer>) {
    move | 
        mut commands: Commands,
        asset_server: Res<AssetServer>
    | {
        let position = Transform::from_translation(position);

        let grid_entity = commands.spawn((
            position,
            Visibility::default(),
            size,
            hextile_f2f_size,
            orientation,
        ))
        .id();


        let mut texture = Handle::default();

        if let Some(default_texture) = default_texture {
            texture = asset_server.load(default_texture);
        }

        for x in 0..size.cols {
            for y in 0..size.rows {
                let tile_pos = GridTilePos::new(x, y);
                let relative_transform = tile_pos
                    .coord_to_world(&size, &hextile_f2f_size, &orientation)
                    .extend(0.);

                let mut tile_entity_commands = commands.spawn( (
                        tile_pos,
                        ChildOf(grid_entity),
                        Transform::from_translation(relative_transform),
                        Pickable::default()
                    ));

                if let Some(_) = default_texture {
                    tile_entity_commands.insert(
                        Sprite{
                            custom_size: Some(Vec2::new(
                                hextile_f2f_size.0,
                                hextile_f2f_size.to_height()
                            )),
                            image: texture.clone(),
                            ..Default::default()
                        }
                    );
                };
            }
        }
    }
}

/// Returns neighboring coordinates of this tile on a horizontal hexgrid
pub fn get_hex_horizontal_neighbor_pos(x: i32, y: i32) -> [(i32, i32); 6] {
    if x % 2 == 0 {
        [
            (x, y + 1),
            (x, y - 1),
            (x-1, y),
            (x-1, y + 1),
            (x+1, y),
            (x+1, y + 1),
        ]
    } else {
        [
            (x, y + 1),
            (x, y - 1),
            (x-1, y),
            (x-1, y - 1),
            (x+1, y),
            (x+1, y - 1),
        ]
    }
}