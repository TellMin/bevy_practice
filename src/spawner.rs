use crate::{
    colider::Collider,
    collision_sound::CollisionSound,
    component::ball::Ball,
    component::brick::{Brick, STAGE_LAYOUT},
    constants::*,
    paddle::Paddle,
    velocity::Velocity,
    wall_bundle::WallBundle,
    wall_location::WallLocation,
};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

pub fn spawn_initial_entities(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Sound
    let ball_collision_sound = asset_server.load("sounds/breakout_collision.ogg");
    commands.insert_resource(CollisionSound::new(ball_collision_sound));

    // Paddle
    let paddle_y = BOTTOM_WALL + GAP_BETWEEN_PADDLE_AND_FLOOR;

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, paddle_y, 0.0),
                scale: PADDLE_SIZE,
                ..default()
            },
            sprite: Sprite {
                color: PADDLE_COLOR,
                ..default()
            },
            ..default()
        },
        Paddle,
        Collider,
    ));

    // Ball
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Circle::default()).into(),
            material: materials.add(ColorMaterial::from(BALL_COLOR)),
            transform: Transform::from_translation(BALL_STARTING_POSITION)
                .with_scale(Vec2::splat(BALL_DIAMETER).extend(1.)),
            ..default()
        },
        Ball,
        Velocity::new(INITIAL_BALL_DIRECTION.normalize() * BALL_SPEED),
    ));

    // Scoreboard
    commands.spawn(
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    font_size: SCOREBOARD_FONT_SIZE,
                    color: TEXT_COLOR,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: SCOREBOARD_FONT_SIZE,
                color: SCORE_COLOR,
                ..default()
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: SCOREBOARD_TEXT_PADDING,
            left: SCOREBOARD_TEXT_PADDING,
            ..default()
        }),
    );

    // Walls
    commands.spawn(WallBundle::new(WallLocation::Left));
    commands.spawn(WallBundle::new(WallLocation::Right));
    commands.spawn(WallBundle::new(WallLocation::Bottom));
    commands.spawn(WallBundle::new(WallLocation::Top));

    spawn_brick(&mut commands);
}

fn spawn_brick(commands: &mut Commands) {
    let brick_size_x = (RIGHT_WALL
        - LEFT_WALL
        - GAP_BETWEEN_BRICKS * (STAGE_LAYOUT[0].len() as f32 - 1.0)
        - GAP_BETWEEN_BRICKS_AND_SIDES * 2.0)
        / STAGE_LAYOUT[0].len() as f32;
    let brick_size_y = (TOP_WALL
        - BOTTOM_WALL
        - GAP_BETWEEN_BRICKS * (STAGE_LAYOUT.len() as f32 - 1.0)
        - GAP_BETWEEN_BRICKS_AND_CEILING
        - GAP_BETWEEN_PADDLE_AND_BRICKS)
        / STAGE_LAYOUT.len() as f32;

    let offset_x = LEFT_WALL + brick_size_x / 2. + GAP_BETWEEN_BRICKS_AND_CEILING;
    let offset_y = TOP_WALL - brick_size_y / 2. - GAP_BETWEEN_BRICKS_AND_SIDES;

    for (row_index, row) in STAGE_LAYOUT.iter().enumerate() {
        for (column_index, &cell) in row.iter().enumerate() {
            if cell == 1 {
                let brick_position = Vec2::new(
                    offset_x + column_index as f32 * (brick_size_x + GAP_BETWEEN_BRICKS),
                    offset_y - row_index as f32 * (brick_size_y + GAP_BETWEEN_BRICKS),
                );

                // brick
                commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: BRICK_COLOR,
                            ..default()
                        },
                        transform: Transform {
                            translation: brick_position.extend(0.0),
                            scale: Vec3::new(brick_size_x, brick_size_y, 1.0),
                            ..default()
                        },
                        ..default()
                    },
                    Brick,
                    Collider,
                ));
            }
        }
    }
}

pub fn respawn_bricks(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    brick_query: Query<Entity, With<Brick>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        for brick_entity in &brick_query {
            commands.entity(brick_entity).despawn();
        }

        spawn_brick(&mut commands);
    }
}
