use bevy::prelude::*;
pub struct CameraPlugin;

#[derive(Component)]
pub struct MainCamera;

fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(Camera2dBundle {
            transform: Transform::from_translation(Vec3 {
                x: -(9.0 * 64.0),
                y: -(9.0 * 64.0),
                z: 0.0,
            }),
            ..default()
        })
        .insert(MainCamera);
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, setup);
    }
}
