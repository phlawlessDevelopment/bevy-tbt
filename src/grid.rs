use crate::common::{Label, Selectable, WorldPosition};
use bevy::prelude::*;

pub struct GridPlugin;

#[derive(Component, Debug)]
pub struct GridPosition {
    pub x:i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Tile;

fn make_tiles(mut commands: Commands, asset_server: Res<AssetServer>) {
    for i in 0..16 {
        let x = (i / 4) as f32 * 64.0;
        let y = (i % 4) as f32 * 64.0;
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("sprites/dice_empty.png"),
                transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
                ..default()
            })
            .insert(Selectable)
            .insert(Label {
                text: String::from(format!("tile {:?}", [i / 4, i % 4])),
            })
            .insert(Tile)
            .insert(GridPosition { x: i / 4, y: i % 4 })
            .insert(WorldPosition { x, y });
    }
}

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(make_tiles);
    }
}
