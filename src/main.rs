use bevy::{prelude::*, render::camera::RenderTarget};

#[derive(Component,Debug)]
struct GridPosition{x:u8,y:u8}

#[derive(Component,Debug,Clone, Copy)]
struct WorldPosition{x:f32,y:f32}

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct Selectable;

#[derive(Component)]
struct Tile;
#[derive(Component)]
struct Unit;


fn get_clicked_entity(
    mouse_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    entities:Query<(&WorldPosition, &GridPosition), With <Selectable>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>){
    if mouse_input.just_pressed(MouseButton::Left){
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
        let world_pos: Vec2 = world_pos.truncate();
        //get closest
        let mut min_dist = 32.0;
        let mut selection : Option<&GridPosition> = None;
        for (world,grid) in entities.into_iter(){
            let dist = world_pos.distance(Vec2::new(world.x,world.y));
            if dist < min_dist{
                min_dist = dist;
                selection = Some(grid);
            }
        }
        if let Some(result) = selection{
            println!("{:?}",result);
        }
    }
    }
    }
fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .
    .add_system(get_clicked_entity)
    .run();
}