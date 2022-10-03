use core::num;

use bevy::prelude::*;
use bevy::render::camera::RenderTarget;

use crate::camera::MainCamera;
use crate::common::{Label, Selectable, WorldPosition};
use crate::grid::{GridPosition, Tile};
use crate::states::TurnPhase;
use crate::turns::ActiveUnit;

pub struct UnitsPlugin;

#[derive(Component, Debug)]
pub struct Movement {
    pub distance: i32,
}
#[derive(Component)]
struct Health {
    max: i32,
    pub value: i32,
}

fn setup_active(mut commands: Commands) {
    commands.insert_resource(ActiveUnit { ..default() });
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
        .insert(Selectable)
        .insert(Label {
            text: String::from("unit"),
        })
        .insert(Movement { distance: 4 })
        .insert(Health { max: 5, value: 5 })
        .insert(GridPosition { x: 0, y: 0 })
        .insert(WorldPosition {
            x: 0 as f32 * 64.0,
            y: 0 as f32 * 64.0,
        });
    // }
}
fn get_mouse_position(
    mouse_input: Res<Input<MouseButton>>,
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
    mouse_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    tiles: Query<(&GridPosition, &WorldPosition), With<Tile>>,
    unit_grids: Query<(Entity, &GridPosition), Without<Tile>>,
    movements: Query<(Entity, &Movement)>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    active: Res<ActiveUnit>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        let mouse_pos = get_mouse_position(mouse_input, windows, q_camera);
        //get closest
        let min_dist = 32.0;
        // let mut selection: Option<&Label> = None;
        let selection = tiles
            .into_iter()
            .find(|(_grid, world)| mouse_pos.distance(Vec2::new(world.x, world.y)) <= min_dist);
        if let Some((grid, _world)) = selection {
            let active = active.as_ref();
            if let Some((_e, active_grid)) = unit_grids
                .into_iter()
                .find(|(e, _g)| e.id() == active.value)
            {
                if let Some((_e, active_movement)) =
                    movements.into_iter().find(|(e, _m)| e.id() == active.value)
                {
                    let dist = calculate_manhattan_distance(&active_grid, grid);
                    if dist >= 1 && dist <= active_movement.distance {
                        println!("Valid");
                    }
                }
            }
        }
    }
}

fn click_unit(
    mouse_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    entities: Query<(Entity, &WorldPosition), With<Movement>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut active: ResMut<ActiveUnit>,
    mut phase: ResMut<State<TurnPhase>>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        let mouse_pos = get_mouse_position(mouse_input, windows, q_camera);
        //get closest
        let min_dist = 32.0;
        // let mut selection: Option<&Label> = None;
        let selection = entities
            .into_iter()
            .find(|(_e, world)| mouse_pos.distance(Vec2::new(world.x, world.y)) <= min_dist);
        if let Some((e, _world)) = selection {
            active.value = e.id();
            phase.set(TurnPhase::Move).unwrap();
        }
    }
}

fn highlight_reachable_tiles(
    mut tiles: Query<(&GridPosition, &mut Sprite), With<Tile>>,
    unit_grids: Query<(Entity, &GridPosition), Without<Tile>>,
    movements: Query<(Entity, &Movement)>,
    active: Res<ActiveUnit>,
) {
    let active = active.as_ref();
    if let Some((_e, active_grid)) = unit_grids
        .into_iter()
        .find(|(e, _g)| e.id() == active.value)
    {
        if let Some((_e, active_movement)) =
            movements.into_iter().find(|(e, _m)| e.id() == active.value)
        {
            for (_grid, mut sprite) in tiles.iter_mut().filter(|(grid, _s)| {
                calculate_manhattan_distance(&active_grid, grid) <= active_movement.distance
            }) {
                sprite.color.set_r(1.0);
                sprite.color.set_a(0.3);
            }
        }
    }
}
fn clear_highlighted_tiles(mut tiles: Query<&mut Sprite, With<Tile>>) {
    for mut sprite in tiles.iter_mut() {
        sprite.color.set_r(1.0);
        sprite.color.set_g(1.0);
        sprite.color.set_b(1.0);
        sprite.color.set_a(1.0);
    }
}
fn calculate_manhattan_distance(a: &GridPosition, b: &GridPosition) -> i32 {
    i32::abs(b.x - a.x) + i32::abs(b.y - a.y)
}
impl Plugin for UnitsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(make_units);
        app.add_startup_system(setup_active);
        app.add_system_set(
            SystemSet::on_enter(TurnPhase::Move)
                .with_system(clear_highlighted_tiles)
                .with_system(highlight_reachable_tiles.after(clear_highlighted_tiles)),
        );
        app.add_system_set(SystemSet::on_update(TurnPhase::Move).with_system(select_move));
        app.add_system_set(SystemSet::on_update(TurnPhase::Select).with_system(click_unit));
    }
}
