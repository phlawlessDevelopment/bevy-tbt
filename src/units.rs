use bevy::{prelude::*, render::camera::RenderTarget};

use crate::{
    camera::MainCamera,
    grid::{GridPosition, Tile},
    states::TurnPhase,
};

pub struct UnitsPlugin;

#[derive(Debug, PartialEq)]
pub enum Team {
    PLAYER,
    AI,
}

#[derive(Component, Debug)]
pub struct Unit {
    pub has_acted: bool,
    pub team: Team,
}

#[derive(Component, Debug)]
pub struct Movement {
    pub distance: i32,
}

#[derive(Component)]
pub struct Health {
    pub max: i32,
    pub value: i32,
}

#[derive(Component)]
pub struct Attack {
    pub dmg: i32,
    pub range: i32,
}
#[derive(Default, Debug)]
pub struct ActiveUnit {
    pub value: u32,
}
#[derive(Default, Debug)]
pub struct SelectedUnit {
    pub value: u32,
    pub grid: (i32, i32),
}
#[derive(Default, Debug)]
pub struct Spawners {
    pub ai_locations: Vec<(f32, f32)>,
    pub player_locations: Vec<(f32, f32)>,
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

fn set_selected_unit(
    mut selected: ResMut<SelectedUnit>,
    mut active: ResMut<ActiveUnit>,
    mut mouse_input: ResMut<Input<MouseButton>>,
    windows: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    units: Query<(Entity, &Transform, &GridPosition, &Unit)>,
    mut phase: ResMut<State<TurnPhase>>,
) {
    if !(*phase.current() == TurnPhase::SelectMove || *phase.current() == TurnPhase::SelectTarget)
        && mouse_input.just_pressed(MouseButton::Left)
    {
        let mouse_pos = get_mouse_position(windows, q_camera);
        //get closest
        let min_dist = 32.0;
        // let mut selection: Option<&Label> = None;
        if let Some((entity, transform, grid, unit)) =
            units.into_iter().find(|(entity, transform, grid, unit)| {
                mouse_pos.distance(Vec2::new(transform.translation.x, transform.translation.y))
                    <= min_dist
            })
        {
            selected.value = entity.id();
            selected.grid = (grid.x, grid.y);
            let cur_phase = *phase.current();
            if !unit.has_acted && unit.team == Team::PLAYER {
                if cur_phase == TurnPhase::SelectUnit {
                    active.value = entity.id();
                    phase.set(TurnPhase::SelectMove).unwrap();
                } else if cur_phase == TurnPhase::SelectAttacker {
                    active.value = entity.id();
                    phase.set(TurnPhase::SelectTarget).unwrap();
                }
                mouse_input.reset(MouseButton::Left);
            }
        }
    }
}

impl Plugin for UnitsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedUnit>()
            .init_resource::<Spawners>()
            .add_system(set_selected_unit);
    }
}
