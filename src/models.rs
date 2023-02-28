use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(SystemLabel)]
pub enum ModelsLabel {
    // system that loaded the models
    Loaded,
}

pub struct ModelsPlugin;

#[derive(Resource)]
pub struct ModelsAssets {
    pub default_player: Handle<Image>,
}

impl Plugin for ModelsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, setup_resources.label(ModelsLabel::Loaded));
    }
}

pub fn setup_resources(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let scene_assets = ModelsAssets {
        default_player: asset_server.load("robot/idle.png"),
    };
    commands.insert_resource(scene_assets);
}