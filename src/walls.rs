use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

use crate::constants::*;

pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_walls);
    }
}

fn setup_walls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(WallBundle::new(
        WallLocation::Left,
        &mut meshes,
        &mut materials,
    ));
    commands.spawn(WallBundle::new(
        WallLocation::Right,
        &mut meshes,
        &mut materials,
    ));
    commands.spawn(WallBundle::new(
        WallLocation::Bottom,
        &mut meshes,
        &mut materials,
    ));
    commands.spawn(WallBundle::new(
        WallLocation::Top,
        &mut meshes,
        &mut materials,
    ));
}

#[derive(Component)]
pub struct Wall;

#[derive(Bundle)]
struct WallBundle<ColorMaterial: 'static + bevy::sprite::Material2d> {
    #[bundle]
    material_bundle: MaterialMesh2dBundle<ColorMaterial>,
    // wall: Wall,
}

impl WallBundle<ColorMaterial> {
    fn new(
        location: WallLocation,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) -> WallBundle<ColorMaterial> {
        let mut mesh = Mesh::from(shape::Quad::default());

    let vertex_colors: Vec<[f32; 4]> = vec![
        Color::RED.as_rgba_f32(),
        Color::GREEN.as_rgba_f32(),
        Color::BLUE.as_rgba_f32(),
        Color::WHITE.as_rgba_f32(),
    ];
    
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, vertex_colors);

    let mesh_handle: Mesh2dHandle = meshes.add(mesh).into();
    
        WallBundle {
            material_bundle: MaterialMesh2dBundle {
                mesh: mesh_handle.clone(),
                transform: Transform {
                    translation: location.position().extend(0.01),
                    scale: location.size().extend(1.0),
                    ..default()
                },
                material: materials.add(ColorMaterial::from(WALL_COLOR)),
                // sprite: Sprite {
                //     color: WALL_COLOR,
                //     ..default()
                // },
                ..default()
            },
            // wall: Wall,
        }
    }
}

enum WallLocation {
    Left,
    Right,
    Bottom,
    Top
}

impl WallLocation {
    fn position(&self) -> Vec2 {
        match self {
            WallLocation::Left => Vec2::new(LEFT_WALL, 0.),
            WallLocation::Right => Vec2::new(RIGHT_WALL, 0.),
            WallLocation::Bottom => Vec2::new(0., BOTTOM_WALL),
            WallLocation::Top => Vec2::new(0., TOP_WALL),
        }
    }

    fn size(&self) -> Vec2 {
        let arena_height = TOP_WALL - BOTTOM_WALL;
        let arena_width = RIGHT_WALL - LEFT_WALL;
        // Make sure we haven't messed up our constants
        assert!(arena_height > 0.0);
        assert!(arena_width > 0.0);

        match self {
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS)
            }
            WallLocation::Bottom | WallLocation::Top => {
                Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS)
            }
        }
    }
}
