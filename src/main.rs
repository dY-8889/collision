use bevy::{prelude::*, sprite::collide_aabb::collide};

const PLAYER_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
const PLAYER_SIZE: Vec3 = Vec3::new(70., 70., 0.);

const BLOCK_COLOR: Color = Color::GRAY;
const BLOCK_SIZE: Vec3 = Vec3::new(90., 90., 0.);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Update, (move_player, check_collisions))
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Block;

#[derive(Component)]
struct Collider;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0., 100., 1.),
                scale: PLAYER_SIZE,
                ..default()
            },
            sprite: Sprite {
                color: PLAYER_COLOR,
                ..default()
            },
            ..default()
        },
        Player,
    ));

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0., 0., 0.),
                scale: BLOCK_SIZE,
                ..default()
            },
            sprite: Sprite {
                color: BLOCK_COLOR,
                ..default()
            },
            ..default()
        },
        Block,
        Collider,
    ));
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let mut player_transform = query.single_mut();

    if keyboard_input.pressed(KeyCode::Left) {
        player_transform.translation.x -= 5.
    }
    if keyboard_input.pressed(KeyCode::Right) {
        player_transform.translation.x += 5.
    }
    if keyboard_input.pressed(KeyCode::Down) {
        player_transform.translation.y -= 5.
    }
    if keyboard_input.pressed(KeyCode::Up) {
        player_transform.translation.y += 5.
    }
}

fn check_collisions(
    player_query: Query<&Transform, With<Player>>,
    collider_query: Query<&Transform, With<Collider>>,
) {
    let player_transform = player_query.single();
    let collider_transform = collider_query.single();

    let collision = collide(
        player_transform.translation,
        player_transform.scale.truncate(),
        collider_transform.translation,
        collider_transform.scale.truncate(),
    );
    if let Some(collision) = collision {
        println!("衝突しました: {:#?}", collision);
    }
}
