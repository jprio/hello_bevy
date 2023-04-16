use bevy::{ prelude::*, sprite::MaterialMesh2dBundle };

fn main() {
    App::new().add_plugins(DefaultPlugins).add_startup_system(setup).run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: asset_server.load("branding/icon.png"),
        ..default()
    });
}