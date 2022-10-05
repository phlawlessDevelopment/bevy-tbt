use bevy::prelude::*;
use bevy::render::camera::RenderTarget;

use crate::camera::MainCamera;
use crate::common::{Label, Selectable};
use crate::grid::{
    self, calculate_manhattan_distance, GridPosition, SelectedPath, SelectedTile, Tile,
};
use crate::states::TurnPhase;
use crate::turns::ActiveUnit;
use crate::units::{Health, Movement, Unit};

pub struct AiUnitsPlugin;

#[derive(Component, Debug)]
pub struct Ai {
    pub has_acted: bool,
}

fn setup_active(mut commands: Commands) {
    commands.insert_resource(ActiveUnit { ..default() });
}

fn move_active_unit(
    time: Res<Time>,
    mut selected_path: ResMut<SelectedPath>,
    active: ResMut<ActiveUnit>,
    mut ai_units: Query<(Entity, &mut Transform, &mut GridPosition), With<Ai>>,
    mut phase: ResMut<State<TurnPhase>>,
) {
    let active = active.as_ref();
    if let Some((_e, mut transform, mut grid)) =
        ai_units.iter_mut().find(|(e, _t, _g)| e.id() == active.value)
    {
        let mut should_pop = false;
        if let Some(next_tile) = selected_path.tiles.last() {
            let direction = Vec3::new(
                next_tile.0 as f32 * 64.0 - (4.5 * 64.0),
                next_tile.1 as f32 * 64.0 - (4.5 * 64.0),
                0.0,
            ) - transform.translation;

            if direction.length() > 1.0 {
                transform.translation += direction.normalize() * time.delta_seconds() * 64.0;
            } else {
                transform.translation = Vec3::new(
                    next_tile.0 as f32 * 64.0 - (4.5 * 64.0),
                    next_tile.1 as f32 * 64.0 - (4.5 * 64.0),
                    0.0,
                );
                grid.x = next_tile.0;
                grid.y = next_tile.1;
                should_pop = true;
            }
        } else {
            phase.set(TurnPhase::SelectUnit).unwrap();
        }
        if should_pop {
            selected_path.tiles.pop();
        }
    }
}

fn make_units(mut commands: Commands, asset_server: Res<AssetServer>) {
    // for i in 0..16 {
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("sprites/chess_pawn.png"),
            transform: Transform::from_translation(Vec3::new(
                1 as f32 * 64.0 - (4.5 * 64.0),
                2 as f32 * 64.0 - (4.5 * 64.0),
                0.0,
            )),
            sprite: Sprite {
                color: Color::Rgba {
                    red: 1.0,
                    green: 0.0,
                    blue: 0.0,
                    alpha: 1.0,
                },
                ..default()
            },
            ..default()
        })
        .insert(Unit)
        .insert(Ai { has_acted: false })
        .insert(Selectable)
        .insert(Label {
            text: String::from("unit"),
        })
        .insert(Movement { distance: 4 })
        .insert(Health { max: 5, value: 5 })
        .insert(GridPosition { x: 1, y: 2 });
    // }
}

fn select_move(
    
) {
    //todo : make path to player, trimmed to move number of tiles allowed by Movement component, set state to AiDoMove
}

fn select_unit(
    entities: Query<(Entity, &Ai)>,
    mut active: ResMut<ActiveUnit>,
    mut phase: ResMut<State<TurnPhase>>,
) {
    for (entity, unit) in entities.into_iter() {
        if !unit.has_acted {
            active.value = entity.id();
            phase.set(TurnPhase::AISelectMove).unwrap();
        }
    }
}

fn check_enemy_has_acted(mut ai_units: Query<&mut Ai>, mut phase: ResMut<State<TurnPhase>>) {
    let mut still_to_act = false;
    for unit in ai_units.iter() {
        if !unit.has_acted {
            still_to_act = true;
        }
    }
    if !still_to_act {
        for mut unit in ai_units.iter_mut() {
            unit.has_acted = false;
        }
        phase.set(TurnPhase::SelectUnit).unwrap();
    }
}

fn clear_active_unit(mut active: ResMut<ActiveUnit>) {
    active.value = 0;
}

impl Plugin for AiUnitsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(make_units)
            .add_startup_system(setup_active)
            .add_system_set(SystemSet::on_update(TurnPhase::AIDoMove).with_system(move_active_unit))
            .add_system_set(
                SystemSet::on_update(TurnPhase::AISelectMove)
                    .with_system(select_move)
                    .with_system(check_enemy_has_acted),
            )
            .add_system_set(SystemSet::on_update(TurnPhase::AISelectUnit).with_system(select_unit))
            .add_system_set(
                SystemSet::on_enter(TurnPhase::SelectUnit).with_system(clear_active_unit),
            );
    }
}
