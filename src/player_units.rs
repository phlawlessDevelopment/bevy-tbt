use crate::ai_units::Ai;
use crate::camera::MainCamera;
use crate::grid::{BlockedTiles, GridConfig, GridPosition, SelectedPath, SelectedTile, Tile};
use crate::pathfinding::calculate_a_star_path;
use crate::states::TurnPhase;
use crate::units::{ActiveUnit, Attack, Health, Movement, Unit};
use bevy::render::camera::RenderTarget;
use bevy::{prelude::*, transform};

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
    mut player_units: Query<(Entity, &mut Transform, &mut GridPosition, &mut Player)>,
    mut phase: ResMut<State<TurnPhase>>,
    grid_config: Res<GridConfig>,
) {
    let active = active.as_ref();
    if let Some((_e, mut transform, mut grid, mut player)) = player_units
        .iter_mut()
        .find(|(e, _t, _g, _p)| e.id() == active.value)
    {
        let mut should_pop = false;
        if let Some(next_tile) = selected_path.tiles.last() {
            let direction = Vec3::new(
                next_tile.0 as f32 * grid_config.tile_size - grid_config.offset(),
                next_tile.1 as f32 * grid_config.tile_size - grid_config.offset(),
                0.0,
            ) - transform.translation;

            if direction.length() > 1.0 {
                transform.translation += direction.normalize() * time.delta_seconds() * 100.0;
            } else {
                transform.translation = Vec3::new(
                    next_tile.0 as f32 * grid_config.tile_size - grid_config.offset(),
                    next_tile.1 as f32 * grid_config.tile_size - grid_config.offset(),
                    0.0,
                );
                grid.x = next_tile.0;
                grid.y = next_tile.1;
                should_pop = true;
            }
        } else {
            player.has_acted = true;
            player.set_changed();
            phase.set(TurnPhase::SelectUnit).unwrap();
        }
        if should_pop {
            selected_path.tiles.pop();
        }
    }
}

fn spawn_unit(
    x: f32,
    y: f32,
    i: i32,
    grid:(i32,i32),
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    grid_config: &Res<GridConfig>,
    sprite_path: &str,
    movement: i32,
    health: i32,
    dmg: i32,
    range: i32,
) -> Entity {
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load(sprite_path),
            transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
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
        .insert(Name::new(format!("Player Unit {}", i)))
        .insert(Movement { distance: movement })
        .insert(Health {
            max: health,
            value: health,
        })
        .insert(Attack {
            dmg: dmg,
            range: range,
        })
        .insert(GridPosition {
            x: grid.0,
            y: grid.1,
        })
        .id()
}

fn make_units(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    grid_config: Res<GridConfig>,
) {
    let mut units = Vec::new();
    let sprites = ["sprites/sword.png", "sprites/fire.png", "sprites/crown.png"];
    let movements = [4, 3, 1];
    let healths = [20, 15, 10];
    let dmgs = [3, 2, 5];
    let ranges = [1, 5, 2];
    let positions = [(0, 1), (1, 0), (0, 0)];

    for i in 0..sprites.len() as i32 {
        let x = (positions[i as usize].0 as f32 * grid_config.tile_size) - grid_config.offset();
        let y = (positions[i as usize].1 as f32 * grid_config.tile_size) - grid_config.offset();
        let unit = spawn_unit(
            x,
            y,
            i,
            positions[i as usize],
            &mut commands,
            &asset_server,
            &grid_config,
            sprites[i as usize],
            movements[i as usize],
            healths[i as usize],
            dmgs[i as usize],
            ranges[i as usize],
        );
        units.push(unit);
    }
    commands
        .spawn()
        .insert(Name::new("Player Units"))
        .insert_bundle(SpatialBundle::default())
        .push_children(&units);
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
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    tiles: Query<(&Tile, &GridPosition, &mut Transform)>,
    player_unit_grids: Query<(Entity, &GridPosition), With<Player>>,
    movements: Query<(Entity, &Movement)>,
    active: Res<ActiveUnit>,
    mut selected_tile: ResMut<SelectedTile>,
    mut phase: ResMut<State<TurnPhase>>,
    blocked: Res<BlockedTiles>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        let mouse_pos = get_mouse_position(windows, q_camera);
        //get closest
        let min_dist = 32.0;
        // let mut selection: Option<&Label> = None;
        let selection = tiles.into_iter().find(|(_tile, _grid, transform)| {
            mouse_pos.distance(Vec2::new(transform.translation.x, transform.translation.y))
                <= min_dist
        });
        if let Some((_tile, grid, _transform)) = selection {
            let active = active.as_ref();
            if let Some((_e, active_grid)) = player_unit_grids
                .into_iter()
                .find(|(e, _g)| e.id() == active.value)
            {
                if let Some((_e, active_movement)) =
                    movements.into_iter().find(|(e, _m)| e.id() == active.value)
                {
                    let dist = calculate_a_star_path(
                        (active_grid.x, active_grid.y),
                        (grid.x, grid.y),
                        &blocked,
                    )
                    .len() as i32;
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

fn select_attacker(
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
            phase.set(TurnPhase::SelectTarget).unwrap();
            mouse_input.reset(MouseButton::Left);
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
fn select_target(
    mut mouse_input: ResMut<Input<MouseButton>>,
    windows: Res<Windows>,
    mut ai_units: Query<(Entity, &GridPosition, &Transform, &mut Health), With<Ai>>,
    mut player_units: Query<(Entity, &mut Player, &GridPosition, &Attack), With<Player>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    active: ResMut<ActiveUnit>,
    mut phase: ResMut<State<TurnPhase>>,
    mut commands: Commands,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        let mouse_pos = get_mouse_position(windows, q_camera);
        //get closest
        let min_dist = 32.0;
        if let Some((active, mut active_player, active_grid, active_attack)) = player_units
            .iter_mut()
            .find(|(entity, unit, _grid, _attack)| entity.id() == active.value)
        {
            let selection = ai_units.iter_mut().find(|(e, grid, transform, health)| {
                let dist = std::cmp::max(
                    i32::abs(grid.x - active_grid.x),
                    i32::abs(grid.y - active_grid.y),
                );
                dist > 0
                    && dist <= active_attack.range
                    && mouse_pos
                        .distance(Vec2::new(transform.translation.x, transform.translation.y))
                        <= min_dist
            });
            if let Some((e, _g, _t, mut target_health)) = selection {
                target_health.value -= active_attack.dmg;
                if target_health.value <= 0 {
                    commands.entity(e).despawn_recursive();
                }
                phase.set(TurnPhase::SelectAttacker).unwrap();
                mouse_input.clear();
                active_player.has_acted = true;
            }
        }
    }
}
fn check_player_has_moved(
    mut player_units: Query<&mut Player>,
    mut phase: ResMut<State<TurnPhase>>,
) {
    let mut still_to_act = false;
    for unit in player_units.iter() {
        if unit.has_acted == false {
            still_to_act = true;
        }
    }
    if !still_to_act {
        for mut unit in player_units.iter_mut() {
            unit.has_acted = false;
        }
        phase.set(TurnPhase::SelectAttacker).unwrap();
    }
}

fn check_player_has_attacked(
    mut player_units: Query<&mut Player>,
    mut phase: ResMut<State<TurnPhase>>,
) {
    let mut still_to_act = false;
    for unit in player_units.iter() {
        if unit.has_acted == false {
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
            .add_system_set(SystemSet::on_update(TurnPhase::SelectMove).with_system(select_move))
            .add_system_set(
                SystemSet::on_update(TurnPhase::SelectUnit)
                    .with_system(check_player_has_moved)
                    .with_system(select_unit.after(check_player_has_moved)),
            )
            .add_system_set(
                SystemSet::on_update(TurnPhase::SelectAttacker)
                    .with_system(check_player_has_attacked)
                    .with_system(select_attacker),
            )
            .add_system_set(
                SystemSet::on_update(TurnPhase::SelectTarget).with_system(select_target),
            )
            .add_system_set(
                SystemSet::on_enter(TurnPhase::SelectUnit).with_system(clear_active_unit),
            );
    }
}
