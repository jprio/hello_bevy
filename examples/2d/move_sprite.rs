//! Renders a 2D scene containing a single, moving sprite.

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(sprite_movement)
        .run();
}

#[derive(Component)]
enum Direction {
    Up,
    Down,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("branding/icon.png"),
            transform: Transform::from_xyz(100f32, 0f32, 0f32),
            ..default()
        },
        Direction::Up,
    ));
}

/// The sprite is animated by changing its translation depending on the time that has passed since
/// the last frame.
fn sprite_movement(time: Res<Time>, mut sprite_position: Query<(&mut Direction, &mut Transform)>) {
    for (mut logo, mut transform) in &mut sprite_position {
        match *logo {
            Direction::Up => {
                transform.translation.y += 150f32 * time.delta_seconds();
            }
            Direction::Down => {
                transform.translation.y -= 150f32 * time.delta_seconds();
            }
        }

        if transform.translation.y > 200f32 {
            *logo = Direction::Down;
        } else if transform.translation.y < -200f32 {
            *logo = Direction::Up;
        }
    }
}