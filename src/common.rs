use bevy::prelude::*;
#[derive(Component)]
pub struct Selectable;

#[derive(Component)]
pub struct Label{
    pub text: String,
}