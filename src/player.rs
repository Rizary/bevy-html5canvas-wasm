use crate::user_input::PlayerInput;
use bevy::prelude::*;
use tracing::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;
use crate::constants::*;
use crate::models::{ModelsAssets, setup_resources};
use crate::logging::logwithdiv;


#[derive(SystemLabel)]
pub enum PlayerStages {
    Move,
}

#[derive(Component)]
pub struct DefaultPlayer;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_resources)
            .add_system(spawn_player)
            .add_system(move_player);
    }
}

fn spawn_player(mut commands: Commands, models: Res<ModelsAssets>) {
    commands.spawn((
       SpriteBundle {
        texture: models.default_player.clone(),
        transform: Transform { 
            translation: Vec3::new(0.0,0.0,1.0), 
            ..default()
        },
        ..default()
       },
       InputManagerBundle {
        input_map: PlayerInput::player_one(),
        ..default()
       },
       DefaultPlayer,
       RigidBody::KinematicPositionBased,
       Velocity::default(),
       Collider::cuboid(9., 15.95),
       LockedAxes::ROTATION_LOCKED_Z,
       Friction {
        coefficient: 5.,
        combine_rule: CoefficientCombineRule::Multiply,
       },
       Damping {
        linear_damping: 1.,
        angular_damping: 1.,
       },
       Name::new("Player")
    ));
}

fn move_player(
    mut player: Query<(
        &mut Velocity,
        &ActionState<PlayerInput>,
        &mut Transform,
    )>,
    rapier_context: Res<RapierContext>,
) {
    for (mut velocity, input, mut pos) in &mut player {
        let mut direction_x: f32 = 0.0;
        let mut direction_y = 0.0;

        if input.pressed(PlayerInput::Up) {
            error_span!("Up");
            direction_y += PLAYER_ACCELERATION;
            // velocity.linvel.y += PLAYER_ACCELERATION;
        } else if input.pressed(PlayerInput::Down) {
            direction_y -= PLAYER_ACCELERATION;
            // velocity.linvel.y -= PLAYER_ACCELERATION;
        } else if input.pressed(PlayerInput::Left) {
            // direction_x += PLAYER_ACCELERATION;
            velocity.linvel.x -= PLAYER_ACCELERATION;
        } else if input.pressed(PlayerInput::Right) {
            // direction_x -= PLAYER_ACCELERATION;
            velocity.linvel.x += PLAYER_ACCELERATION;
        };
        // velocity.linvel.x = velocity.linvel.x.clamp(-PLAYER_MAX_SPEED, PLAYER_MAX_SPEED);
        // velocity.linvel.y = velocity.linvel.y.clamp(-PLAYER_MAX_SPEED, PLAYER_MAX_SPEED);
        pos.translation.y = pos.translation.y + direction_y;
        pos.translation.x = pos.translation.x + direction_x;
    }
}
