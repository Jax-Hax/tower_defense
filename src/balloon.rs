use bevy_ecs::{system::{Query, Res, ResMut}, component::Component};
use vertix::{structs::Instance, state::UpdateInstance};

use crate::map::Map;
#[derive(Component)]
pub struct Balloon {
    health: isize,
    bloon_type: usize,
    speed: f32,
    way_point_index: usize,
    damage_dealt: isize, //allows for bloons that gain health?
}
fn balloon_movement(mut balloons: Query<(&mut Instance, &mut Balloon)>, map: Res<Map>, mut instance_update: ResMut<UpdateInstance>) {
    let mut instances = vec![];
    let mut temp_instance = Instance {..Default::default()};
    balloons
        .par_iter_mut()
        .for_each_mut(|(mut instance, mut balloon)| {
            
        });
    temp_instance.update(instances, &mut instance_update);
}