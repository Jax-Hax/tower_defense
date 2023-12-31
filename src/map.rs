use bevy_ecs::system::Resource;
use glam::Vec2;
#[derive(Resource)]
pub struct Map {
    pub spawn_location: Vec2,
    pub map_waypoints: Vec<Vec2>,
}