use bevy_ecs::{entity::Entity, system::Resource};
use vertix::{prelude::Instance, state::State, loader::load_string};
use serde::Deserialize;
use crate::enemy::EnemyType;
#[derive(Deserialize,Debug)]
struct EnemyJSON {
    name: String,
    starting_health: i32,
    speed: f64,
    damage_dealt: i32,
    image_file: String,
    children: Vec<i32>,  // Assuming children is an array of integers
    pool_am: i32
}

#[derive(Deserialize,Debug)]
struct EnemyJSONData {
    enemies: Vec<EnemyJSON>
}
//td specific
pub async fn pool_from_json(json_file: &str) {
    let json_string = load_string(json_file, env!("OUT_DIR")).await.unwrap();
    let enemy_data: EnemyJSONData =
        serde_json::from_str(&json_string).expect("JSON was not well-formatted");
    println!("{:#?}", enemy_data);
}
//pooler in general
pub async fn pool_object(pool_vec: Vec<PooledObject>, state: &mut State) {
    let mut pooler = ObjectPooler {pooled_objects: vec![]};
    for pool_object in pool_vec {
        let mut instances = vec![(Instance { ..Default::default() },); pool_object.am_to_pool];
        state
        .create_model_instances(&pool_object.balloon_type.image_file, instances.iter_mut().map(|(instance, )| instance).collect(), true)
        .await;
        let entities = state.world.spawn_batch(instances).collect::<Vec<Entity>>();
        pooler.pooled_objects.push(EntityPool {active: entities})
    }
    state.world.insert_resource(pooler);
}
pub struct EntityPool {
    pub active: Vec<Entity>,
}
pub struct PooledObject{
    balloon_type: EnemyType,
    am_to_pool: usize,
}
#[derive(Resource)]
pub struct ObjectPooler{
    pooled_objects: Vec<EntityPool>
}
impl ObjectPooler {
    pub fn set_inactive(&mut self, index: usize, entity: Entity) {
        self.pooled_objects[index].active.push(entity);
    }
    pub fn get_inactive(&mut self, index: usize) -> Entity{
        self.pooled_objects[index].active.pop().unwrap()
    }
}