use bevy_ecs::{system::{Query, Res, ResMut, Resource}, component::Component};
use glam::{Vec2, Vec3};
use vertix::{prelude::{Instance, delta_time_to_seconds}, resources::{UpdateInstance, DeltaTime}};

use crate::map::Map;
#[derive(Component)]
pub struct Balloon {
    health: isize,
    bloon_type: usize,
    way_point_index: usize,
}
fn balloon_movement(mut balloons: Query<(&mut Instance, &mut Balloon)>, map: Res<Map>, mut instance_update: ResMut<UpdateInstance>, balloon_types: Res<BalloonTypes>, delta_time: Res<DeltaTime>) {
    let mut instances = vec![];
    let mut temp_instance = Instance {..Default::default()};
    for (mut instance,mut balloon) in &mut balloons {
        instance.position = move_towards(instance.pos_2d(), map.map_waypoints[balloon.way_point_index], balloon_types.types[balloon.bloon_type].speed * delta_time_to_seconds(delta_time.dt))
    }
    temp_instance.update(instances, &mut instance_update);
}
pub struct BalloonType {
    max_health: usize,
    speed: f32,
    damage_dealt: isize, //allows for bloons that gain health?
    children: Vec<Box<BalloonType>>,
}
#[derive(Resource)]
pub struct BalloonTypes{
    types: Vec<BalloonType>
}
fn move_towards(p1: Vec2, p2: Vec2, smooth_val: f32) -> Vec3 {
    let x = p1.x + p2.x * smooth_val;
    let y = p2.y + (p2.y - p1.y) * (x - p2.x)/(p2.x - p1.x);
    Vec3::new(x, y, 0.)
}