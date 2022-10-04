use crate::{
    common::{Label, Selectable},
    states::TurnPhase,
    units::Unit,
};
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

#[derive(Default, Debug)]
pub struct SelectedTile {
    pub x: i32,
    pub y: i32,
}

fn make_tiles(mut commands: Commands, asset_server: Res<AssetServer>) {
    for i in 0..81 {
        let x = ((i / 9) as f32 * 64.0) - (4.5 * 64.0);
        let y = (i % 9) as f32 * 64.0 - (4.5 * 64.0);
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("sprites/dice_empty.png"),
                transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
                ..default()
            })
            .insert(Selectable)
            .insert(Label {
                text: String::from(format!("tile {:?}", [i / 9, i % 9])),
            })
            .insert(Tile { blocked: false })
            .insert(GridPosition { x: i / 9, y: i % 9 });
    }
}

fn set_blocked_tiles(
    units: Query<&GridPosition, With<Unit>>,
    mut tiles: Query<(&GridPosition, &mut Tile)>,
) {
    for (tile_pos, mut tile) in tiles.iter_mut() {
        if let Some(_unit_pos) = units
            .into_iter()
            .find(|u| u.x == tile_pos.x && u.y == tile_pos.y)
        {
            tile.blocked = true;
        } else {
            tile.blocked = false;
        }
    }
}

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedPath>()
            .init_resource::<SelectedTile>()
            .add_startup_system(make_tiles)
            .add_system_set(
                SystemSet::on_enter(TurnPhase::SelectMove)
                    .with_system(set_blocked_tiles),
            );
    }
}
