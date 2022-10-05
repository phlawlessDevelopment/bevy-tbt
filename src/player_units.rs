use bevy::prelude::*;
use bevy::render::camera::RenderTarget;

use crate::camera::MainCamera;
use crate::common::{Label, Selectable};
use crate::grid::{calculate_manhattan_distance, GridPosition, SelectedPath, SelectedTile, Tile};
use crate::states::TurnPhase;
use crate::turns::ActiveUnit;
use crate::units::{Unit, Movement,Health};

pub struct PlayerUnitsPlugin;


#[derive(Component, Debug)]
pub struct Player {
    pub has_acted: bool,
}

fn setup_active(mut commands: Commands) {
    commands.insert_resource(ActiveUnit { ..default() });
}

fn move_active_unit(
    time: Res<Time>,
    mut selected_path: ResMut<SelectedPath>,
    active: ResMut<ActiveUnit>,
    mut units: Query<(Entity, &mut Transform, &mut GridPosition), With<Unit>>,
    mut phase: ResMut<State<TurnPhase>>,
) {
    let active = active.as_ref();
    if let Some((_e, mut transform, mut grid)) =
        units.iter_mut().find(|(e, _t, _g)| e.id() == active.value)
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
                0 as f32 * 64.0 - (4.5 * 64.0),
                0 as f32 * 64.0 - (4.5 * 64.0),
                0.0,
            )),
            sprite: Sprite {
                color: Color::Rgba {
                    red: 0.0,
                    green: 1.0,
                    blue: 0.0,
                    alpha: 1.0,
                },
                ..default()
            },
            ..default()
        })
        .insert(Unit)
        .insert(Player { has_acted: false })
        .insert(Selectable)
        .insert(Label {
            text: String::from("unit"),
        })
        .insert(Movement { distance: 4 })
        .insert(Health { max: 5, value: 5 })
        .insert(GridPosition { x: 0, y: 0 });

    // }
}

fn get_mouse_position(
    windows: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) -> Vec2 {
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // get the window that the camera is displaying to (or the primary window)
    let wnd = if let RenderTarget::Window(id) = camera.target {
        windows.get(id).unwrap()
    } else {
        windows.get_primary().unwrap()
    };

    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = wnd.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);
        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();
        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
        // reduce it to a 2D value
        return world_pos.truncate();
    }
    return Vec2::ZERO;
}

fn select_move(
    mut mouse_input: ResMut<Input<MouseButton>>,
    windows: Res<Windows>,
    tiles: Query<(&GridPosition, &mut Transform), With<Tile>>,
    player_unit_grids: Query<(Entity, &GridPosition), With<Player>>,
    // ai_unit_grids: Query<(Entity, &GridPosition), With<Ai>>,
    movements: Query<(Entity, &Movement)>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    active: Res<ActiveUnit>,
    mut selected_tile: ResMut<SelectedTile>,
    mut phase: ResMut<State<TurnPhase>>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        let mouse_pos = get_mouse_position(windows, q_camera);
        //get closest
        let min_dist = 32.0;
        // let mut selection: Option<&Label> = None;
        let selection = tiles.into_iter().find(|(_grid, transform)| {
            mouse_pos.distance(Vec2::new(transform.translation.x, transform.translation.y))
                <= min_dist
        });
        if let Some((grid, _transform)) = selection {
            let active = active.as_ref();
            if let Some((_e, active_grid)) = player_unit_grids
                .into_iter()
                .find(|(e, _g)| e.id() == active.value)
            {
                if let Some((_e, active_movement)) =
                    movements.into_iter().find(|(e, _m)| e.id() == active.value)
                {
                    let dist = calculate_manhattan_distance(&active_grid, grid);
                    if dist >= 1 && dist <= active_movement.distance {
                        selected_tile.x = grid.x;
                        selected_tile.y = grid.y;
                        phase.set(TurnPhase::DoMove).unwrap();
                        mouse_input.reset(MouseButton::Left);
                    }
                }
            }
        }
    }
}

fn select_unit(
    mut mouse_input: ResMut<Input<MouseButton>>,
    windows: Res<Windows>,
    entities: Query<(Entity, &mut Transform), With<Player>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut active: ResMut<ActiveUnit>,
    mut phase: ResMut<State<TurnPhase>>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        let mouse_pos = get_mouse_position(windows, q_camera);
        //get closest
        let min_dist = 32.0;
        // let mut selection: Option<&Label> = None;
        let selection = entities.into_iter().find(|(_e, transform)| {
            mouse_pos.distance(Vec2::new(transform.translation.x, transform.translation.y))
                <= min_dist
        });
        if let Some((e, _transform)) = selection {
            active.value = e.id();
            phase.set(TurnPhase::SelectMove).unwrap();
            mouse_input.reset(MouseButton::Left);
        }
    }
}

fn check_player_has_acted(
    mut player_units: Query<&mut Player>,
    mut phase: ResMut<State<TurnPhase>>,
) {
    let mut still_to_act = false;
    for unit in player_units.iter() {
        if !unit.has_acted {
            still_to_act = true;
        }
    }
    if !still_to_act {
        for mut unit in player_units.iter_mut() {
            unit.has_acted = false;
        }
        phase.set(TurnPhase::AISelectUnit).unwrap();
    }
}

fn clear_active_unit(mut active: ResMut<ActiveUnit>) {
    active.value = 0;
}

impl Plugin for PlayerUnitsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(make_units)
            .add_startup_system(setup_active)
            .add_system_set(SystemSet::on_update(TurnPhase::DoMove).with_system(move_active_unit))
            .add_system_set(
                SystemSet::on_update(TurnPhase::SelectMove)
                    .with_system(select_move)
                    .with_system(check_player_has_acted),
            )
            .add_system_set(SystemSet::on_update(TurnPhase::SelectUnit).with_system(select_unit))
            .add_system_set(
                SystemSet::on_enter(TurnPhase::SelectUnit).with_system(clear_active_unit),
            );
    }
}