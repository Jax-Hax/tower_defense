use bevy_ecs::{system::{Query, Res, ResMut, Resource, Commands}, component::Component, world::World, bundle::Bundle};
use glam::{Vec2, Vec3};
use vertix::{prelude::{Instance, delta_time_to_seconds}, resources::{UpdateInstance, DeltaTime}};

use crate::{map::Map, object_pooler::ObjectPooler};
#[derive(Component, Clone, Copy)]
pub struct Enemy {
    pub health: isize,
    pub bloon_type: usize,
    pub way_point_index: usize,
}
pub fn enemy_movement(mut world: Commands, mut enemies: Query<(&mut Instance, &mut Enemy)>, map: Res<Map>, mut instance_update: ResMut<UpdateInstance>, enemy_types: Res<EnemyTypes>, mut pooler: ResMut<ObjectPooler>, delta_time: Res<DeltaTime>) {
    let mut instances = vec![];
    let mut temp_instance = Instance {..Default::default()};
    let mut is_more_than_one = false;
    for (mut instance,mut enemy,) in &mut enemies {
        if instance.enabled{
            instance.position = move_towards(instance.pos_2d(), map.map_waypoints[enemy.way_point_index], enemy_types.types[enemy.bloon_type].speed * delta_time_to_seconds(delta_time.dt));
            let instance_raw = instance.to_raw();
            if instance_raw.is_some() {
                instances.push(instance_raw.unwrap());
            }
            temp_instance = *instance;
            is_more_than_one = true;
        }
    }
    if is_more_than_one{temp_instance.update(instances, &mut instance_update);}
    
}
#[derive(Clone)]
pub struct EnemyType {
    pub starting_health: usize,
    pub speed: f32,
    pub damage_dealt: isize, //allows for bloons that gain health?
    pub children: Vec<usize>, //indexes into an array of enemyTypes
    pub image_file: String
}
#[derive(Resource)]
pub struct EnemyTypes{
    pub types: Vec<EnemyType>
}
fn move_towards(p1: Vec2, p2: Vec2, smooth_val: f32) -> Vec3 {
    let x = p1.x + p2.x * smooth_val;
    let y = p2.y + (p2.y - p1.y) * (x - p2.x)/(p2.x - p1.x);
    Vec3::new(x, y, 0.)
}