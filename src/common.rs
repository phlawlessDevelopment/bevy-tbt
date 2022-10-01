use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy)]
pub struct WorldPosition {
    pub x: f32,
    pub y: f32,
}



#[derive(Component)]
pub struct Selectable;
