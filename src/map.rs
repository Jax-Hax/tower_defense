use bevy_ecs::system::Resource;
use glam::Vec2;
#[derive(Resource)]
pub struct Map {
    spawn_location: Vec2,
    map_waypoints: Vec<Vec2>,
}