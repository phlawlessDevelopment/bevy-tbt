use bevy::prelude::*;

use crate::common::{Selectable, WorldPosition, Label};
use crate::grid::GridPosition;

pub struct UnitsPlugin;

#[derive(Component)]
struct Movement {
    pub distance: u8,
}
#[derive(Component)]
struct Health {
    max: u8,
    pub value: u8,
}

fn make_units(mut commands: Commands, asset_server: Res<AssetServer>) {
    // for i in 0..16 {
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("sprites/chess_pawn.png"),
            transform: Transform::from_translation(Vec3::new(
                0 as f32 * 64.0,
                0 as f32 * 64.0,
                0.0,
            )),
            sprite:Sprite{
                color:Color::Rgba { red: 0.0, green: 1.0, blue: 0.0, alpha: 1.0 },
                ..default()
            },
            ..default()
        })
        .insert(Selectable)
        .insert(Label{text:String::from("unit")})
        .insert(Movement { distance: 4 })
        .insert(Health { max: 5, value: 5 })
        .insert(GridPosition { x: 0, y: 0 })
        .insert(WorldPosition {
            x: 0 as f32 * 64.0,
            y: 0 as f32 * 64.0,
        });
    // }
}
impl Plugin for UnitsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(make_units);
    }
}
