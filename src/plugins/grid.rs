pub struct GridPlugin;



fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default()).insert(MainCamera);
    
}
fn make_tiles(mut commands: Commands, asset_server: Res<AssetServer>){
    for i in 0..16{
        commands.spawn()
        .insert(Selectable)
        .insert(Tile)
        .insert(GridPosition{x:i/4,y:i%4})
        .insert(WorldPosition{x:(i/4) as f32 *64.0,y:(i%4) as f32*64.0});        
        commands.spawn_bundle(SpriteBundle {
            texture: asset_server.load("sprites/dice_empty.png"),
            transform: Transform::from_translation(
                Vec3::new((i/4) as f32 *64.0,(i%4) as f32*64.0, 0.0),
            ),
            ..default()
        });
    }
}

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
    app
    .add_startup_system_to_stage(StartupStage::PreStartup, setup)
    .add_startup_system(make_tiles);
    }
}