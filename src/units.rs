use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

#[derive(Component, Debug, Inspectable)]
pub struct Unit;

#[derive(Component, Debug)]
pub struct Movement {
    pub distance: i32,
}
#[derive(Component)]
pub struct Health {
    pub max: i32,
    pub value: i32,
}
