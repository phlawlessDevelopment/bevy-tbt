use crate::common::{Label, Selectable};
use bevy::prelude::*;

pub struct GridPlugin;

#[derive(Component, Debug)]
pub struct GridPosition {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Tile {
    pub blocked: bool,
}


#[derive(Default)]
pub struct SelectedPath {
    pub tiles: Vec<(i32, i32)>,
}

#[derive(Default,Debug)]
pub struct SelectedTile {
    pub x: i32,
    pub y: i32,
}

fn make_tiles(mut commands: Commands, asset_server: Res<AssetServer>) {
    for i in 0..81 {
        let x = (i / 9) as f32 * 64.0;
        let y = (i % 9) as f32 * 64.0;
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
            .insert(Tile { blocked: false })
            .insert(GridPosition { x: i / 4, y: i % 4 });
    }
}

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<SelectedPath>()
        .init_resource::<SelectedTile>()
        .add_startup_system(make_tiles);

    }
}
