use bevy::prelude::*;

use crate::camera::MainCamera;
use crate::common::{Selectable, WorldPosition};
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
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("sprites/chess_pawn.png"),
        transform: Transform::from_translation(Vec3::new(0 as f32 * 64.0, 0 as f32 * 64.0, 0.0)),
        ..default()
    })
    .insert(Selectable)
    .insert(Movement { distance: 4 })
    .insert(Health { max: 5, value: 5 })
    .insert(GridPosition { x: 0, y: 0 })
    .insert(WorldPosition {
        x: 0 as f32 * 64.0,
        y: 0 as f32 * 64.0,
    });
    // }
}
fn tint_units(mut query: Query<&mut Sprite>) {
    println!("OUTER");
    for mut sprite in query.iter_mut() {
        println!("INNER");
        sprite.color.set_g(1.0);
    }
}
impl Plugin for UnitsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(make_units)
            .add_startup_system(tint_units.after(make_units));
    }
}
