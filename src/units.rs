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

fn random_starting_unit(
    mut commands: Commands,
    units: Query<Entity, With<Movement>>,
) {
    // todo : replace with click to select
    println!("{:?}", units.is_empty());
    if let Some(unit) = units.into_iter().next() {
        println!("inner");
        commands.insert_resource(ActiveUnit { value: unit.id() });
    }
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
fn click_tile(
    mouse_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    entities: Query<(&GridPosition, &WorldPosition), With<Tile>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
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
            let ndc_to_world =
                camera_transform.compute_matrix() * camera.projection_matrix().inverse();
            // use it to convert ndc to world-space coordinates
            let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
            // reduce it to a 2D value
            let world_pos: Vec2 = world_pos.truncate();
            //get closest
            let min_dist = 32.0;
            // let mut selection: Option<&Label> = None;
            let selection = entities
                .into_iter()
                .find(|(grid, world)| world_pos.distance(Vec2::new(world.x, world.y)) <= min_dist);
            if let Some((grid, world)) = selection {
                println!("{:?} {:?}", world, grid);
            }
        }
    }
}
fn highlight_reachable_tiles(
    mut tiles: Query<(&GridPosition, &mut Sprite), With<Tile>>,
    unit_grids: Query<(Entity,&GridPosition), Without<Tile>>,
    movements: Query<(Entity, &Movement)>,
    active: Res<ActiveUnit>,
) {
    let active = active.as_ref();
    if let Some((_e,active_grid)) = unit_grids.into_iter().find(|(e,_g)| e.id() == active.value) {
        if let Some((_e,active_movement)) = movements.into_iter().find(|(e,_m)| e.id() == active.value) {
            if let Some((_grid, mut sprite)) = tiles
                .iter_mut()
                .find(|(grid, _s)| calculate_grid_distance(&active_grid, grid) <= active_movement.distance)
            {
                sprite.color.set_r(1.0);
                sprite.color.set_a(0.3);
            }
        }
    }
}
fn calculate_grid_distance(a: &GridPosition, b: &GridPosition) -> i32 {
    let dx = i32::abs(b.x as i32 - a.x as i32);
    let dy = i32::abs(b.y as i32 - a.y as i32);
    let min = std::cmp::min(dx, dy);
    let max = std::cmp::max(dx, dy);
    ((2.0 as f64).sqrt() * ((min + max - min) as f64)) as i32
}
impl Plugin for UnitsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(make_units);
        app.add_startup_system(random_starting_unit.after(make_units));
        app.add_system_set(
            SystemSet::on_update(TurnPhase::Move)
                .with_system(click_tile)
                .with_system(highlight_reachable_tiles),
        );
    }
}
