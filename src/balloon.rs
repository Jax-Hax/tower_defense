use bevy_ecs::{system::{Query, Res, ResMut}, component::Component, world::Mut};
use glam::Vec2;
use vertix::{structs::Instance, state::UpdateInstance};

use crate::map::Map;
#[derive(Component)]
pub struct Balloon {
    health: isize,
    bloon_type: usize,
    way_point_index: usize,
}
fn move_balloons(mut balloons: Query<(&mut Instance, &mut Balloon)>, map: Res<Map>, mut instance_update: ResMut<UpdateInstance>) {
    let mut instances = vec![];
    let mut temp_instance = Instance {..Default::default()};
    balloons
        .par_iter_mut()
        .for_each_mut(|(mut instance, mut balloon)| {
            balloon_movement(instance,balloon,map)
        });
    temp_instance.update(instances, &mut instance_update);
}
fn balloon_movement(instance: Mut<'_, Instance>, balloon: Mut<'_, Balloon>, map: Res<'_, Map>) {
    balloon.
}
pub struct BalloonType {
    max_health: usize,
    speed: f32,
    damage_dealt: isize, //allows for bloons that gain health?
    children: Vec<Box<BalloonType>>,
}
fn move_towards(p1: Vec2, p2: Vec2, smooth_val: f32) {
    let x = p1.x + p2.x * smooth_val;
    let y = p2.y + (p2.y - p1.y) * (x - p2.x)/(p2.x - p1.x);
    Vec2::new(x, y);
}