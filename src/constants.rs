use bevy::prelude::*;

// Player constants
pub const PLAYER_MAX_SPEED: f32 = 200.;
pub const PLAYER_ACCELERATION: f32 = 50.;
pub const PLAYER_SPAWMN_POS: Vec2 = Vec2::new(-150.,0.);

pub const ASSET_SCALE: f32 = 20.0;

// Wall Constants
pub const WALL_THICKNESS: f32 = 1.5 * ASSET_SCALE;
// x coordinates
pub const LEFT_WALL: f32 = -500.0;
pub const RIGHT_WALL: f32 = 500.0;
// y coordinates
pub const TOP_WALL: f32 = 375.0;
pub const BOTTOM_WALL: f32 = -375.0;

pub const WALL_COLOR: Color = Color::MIDNIGHT_BLUE;

// Block Alert constant
// green_block coordinates
pub const X_GREEN_BLOCK: f32 = RIGHT_WALL - 100.0;
pub const Y_GREEN_BLOCK: f32 = BOTTOM_WALL + 200.0;
// yellow_block coordinates
pub const X_YELLOW_BLOCK: f32 = LEFT_WALL + 100.0;
pub const Y_YELLOW_BLOCK: f32 = TOP_WALL - 200.0;