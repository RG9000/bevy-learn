use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::sprite::collide_aabb;

const IMAGE_SIZE:Vec2 = Vec2::new(16.0,16.0);
const IMAGE_SIZE_F: f32 = 16.0;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_startup_system(spawn_camera)
    .add_startup_system(spawn_player)
    .add_startup_system(spawn_floor)
    .add_system(player_movement)
    .run();
}

#[derive(Component)]
pub struct Player {
    x_speed: f32,
    y_speed: f32
}

#[derive(Component)]
pub struct FloorTile {
}

pub fn spawn_floor (
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
) {
    let window = window_query.get_single().unwrap();
    commands.spawn(
        (
            SpriteBundle {
                transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0 - 48.0 , 0.0),
                texture: asset_server.load("sprites/floor.png"),
                ..default()
            },
            FloorTile {}
        )
    );

}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
) {
    let window = window_query.get_single().unwrap();
    commands.spawn(
        (
            SpriteBundle {
                transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
                texture: asset_server.load("sprites/player.png"),
                ..default()
            },
            Player {x_speed: 0.0, y_speed: 0.0}
        )
    );
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut Player), Without<FloorTile>>,
    floor_query: Query<&Transform, With<FloorTile>>,
    time: Res<Time>
) {
    if let Ok((mut transform, mut player)) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;
        let mut moving_x: bool = false;
        if keyboard_input.pressed(KeyCode::A) {
            moving_x = true;
            if player.x_speed > 0.0 {
                player.x_speed -= 20.0 * time.delta_seconds();
            }
            else if player.x_speed > -10.0
            {
                player.x_speed -= 5.0 * time.delta_seconds();
            }
        }
        if keyboard_input.pressed(KeyCode::D) {
            moving_x = true;
            if player.x_speed < 0.0 {
                player.x_speed += 20.0 * time.delta_seconds();
            }
            else if player.x_speed < 10.0
            {
                player.x_speed += 5.0 * time.delta_seconds();
            }
        }
        if moving_x == false {
            if player.x_speed > 0.0 {
                player.x_speed -= 20.0 * time.delta_seconds();
                if player.x_speed < 0.0 {
                    player.x_speed = 0.0;
                }
            }
            else if player.x_speed < 0.0 {
                player.x_speed += 20.0 * time.delta_seconds();
                if player.x_speed > 0.0 {
                    player.x_speed = 0.0;
                }
            }
        }
        let mut on_the_floor : bool = false;
        for tile_transform in floor_query.iter()
        {
            if collide_aabb::collide(tile_transform.translation, IMAGE_SIZE, transform.translation, IMAGE_SIZE).is_some() {
                on_the_floor = true;
                player.y_speed = 0.0;
                transform.translation.y = tile_transform.translation.y + IMAGE_SIZE_F;
                break;
            }
        }
        if !on_the_floor
        {
            player.y_speed -= 20.0 * time.delta_seconds();
        }
        else {
            if keyboard_input.pressed(KeyCode::Space)
            {
                player.y_speed += 10.0;
            }
        }
        if direction.length() > 0.0 {
            direction = direction.normalize();
        }
        direction += Vec3::new(player.x_speed, player.y_speed, 0.0);
        transform.translation += direction;
    }
}

pub fn spawn_camera(
     mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        }
    );
    
}

