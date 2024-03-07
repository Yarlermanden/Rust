use bevy::prelude::*;

#[derive(Component)]
pub struct Voxel {
    //Size,
    //position,
    //color,
    //health, //for destruction
    //texture,
    position: Vec3,
}

#[derive(Component)]
pub struct GrowingVoxel {
    pub timer: Timer,
}

impl Default for GrowingVoxel {
    fn default() -> Self { GrowingVoxel { timer: Timer::from_seconds(10.0, TimerMode::Repeating) }}
}