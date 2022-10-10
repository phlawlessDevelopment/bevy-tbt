use bevy::{prelude::*, render::camera::RenderTarget};

use crate::camera::MainCamera;

pub struct UnitsPlugin;

#[derive(Component, Debug)]
pub struct Unit
{
    pub has_acted:bool,
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
    mut mouse_input: ResMut<Input<MouseButton>>,
    windows: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    units: Query<(Entity, &Transform), With<Unit>>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        let mouse_pos = get_mouse_position(windows, q_camera);
        //get closest
        let min_dist = 32.0;
        // let mut selection: Option<&Label> = None;
        if let Some((entity, transform)) = units.into_iter().find(|(entity, transform)| {
            mouse_pos.distance(Vec2::new(transform.translation.x, transform.translation.y))
                <= min_dist
        }) {
            selected.value = entity.id();
        }
    }
}

impl Plugin for UnitsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedUnit>()
            .add_system(set_selected_unit);
    }
}
