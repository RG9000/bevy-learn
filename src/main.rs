use bevy::prelude::*;
use bevy::window::PrimaryWindow;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .run();
}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
) {
    let window = window_query.get_single().unwrap();
}

