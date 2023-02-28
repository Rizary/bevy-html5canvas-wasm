mod logging;
mod router;
mod player;
mod models;
mod walls;
mod constants;
mod user_input;
mod utils;

use std::sync::Arc;
use bevy_rapier2d::prelude::RapierPhysicsPlugin;
use bevy_rapier2d::prelude::NoUserData;
use bevy_rapier2d::render::RapierDebugRenderPlugin;
use constants::*;
use tracing::Level;
use user_input::PlayerInput;
use wasm_bindgen::{prelude::*, JsCast};
use leafwing_input_manager::prelude::*;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*, log::LogPlugin,
};
use dominator::append_dom;

use router::DominatorApp;
use logging::logwithdiv;
use crate::walls::WallPlugin;
use crate::models::ModelsPlugin;
use crate::player::PlayerPlugin;



#[wasm_bindgen(start)]
pub async fn main_js() {
    logwithdiv("load dominator app");
    let dominator_app = Arc::new(DominatorApp::new());
    dominator::append_dom(&dominator::body(), DominatorApp::render(DominatorApp::new()));

    logwithdiv("creating bevy app");
    let mut app = App::new();

    let window = WindowDescriptor {
        title: "Lobby Game".to_string(),
        resizable: false,
        canvas: Some(format!("#game")),
        height: 800.0,
        width: 800.0,
        decorations: false,
        fit_canvas_to_parent: true,
        mode: WindowMode::BorderlessFullscreen,
        ..default()
    };

    app.insert_resource(ClearColor(Color::WHITE))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window,
                    ..default()
                })
                .set(AssetPlugin {
                    asset_folder: "./static/assets".to_string(),
                    ..default()
                })
                .set(LogPlugin {
                    level: Level::DEBUG,
                    filter: "wgpu=error".to_string(),
                }),
        )
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(16.))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(InputManagerPlugin::<PlayerInput>::default())
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_map)
        .add_plugin(ModelsPlugin)
        .add_plugin(WallPlugin)
        .add_plugin(PlayerPlugin)
        .run();

    std::mem::forget(Box::new(dominator_app));

}

fn spawn_camera(mut commands: Commands) {
    logwithdiv("camera spawned");
    // camera
    commands.spawn(Camera2dBundle::default());
}

fn spawn_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    logwithdiv("map spawned");
    commands.spawn(SpriteBundle {
        texture: asset_server.load("background.png"),
        transform: Transform { 
            translation: Vec3::new(0.0,0.0,0.0001), 
            ..default()
        },
        ..default()
    });
    
    commands.spawn(SpriteBundle {
        texture: asset_server.load("block_green_large.png"),
        transform: Transform { 
            translation: Vec3::new(X_GREEN_BLOCK,Y_GREEN_BLOCK,0.01), 
            ..default()
        },
        ..default()
    });
    
    commands.spawn(SpriteBundle {
        texture: asset_server.load("block_yellow_large.png"),
        transform: Transform { 
            translation: Vec3::new(X_YELLOW_BLOCK,Y_YELLOW_BLOCK,0.01), 
            ..default()
        },
        ..default()
    });
}
