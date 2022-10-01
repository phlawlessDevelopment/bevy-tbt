use crate::common::{Selectable, WorldPosition};
use bevy::prelude::*;

pub struct GridPlugin;

#[derive(Component, Debug)]
pub struct GridPosition {
    pub x: u8,
    pub y: u8,
}

#[derive(Component)]
struct Tile;

fn make_tiles(mut commands: Commands, asset_server: Res<AssetServer>) {
    for i in 0..16 {
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("sprites/dice_empty.png"),
                transform: Transform::from_translation(Vec3::new(
                    (i / 4) as f32 * 64.0,
                    (i % 4) as f32 * 64.0,
                    0.0,
                )),
                ..default()
            })
            .insert(Selectable)
            .insert(Tile)
            .insert(GridPosition { x: i / 4, y: i % 4 })
            .insert(WorldPosition {
                x: (i / 4) as f32 * 64.0,
                y: (i % 4) as f32 * 64.0,
            });
    }
}

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(make_tiles);
    }
}
