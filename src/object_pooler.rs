use bevy_ecs::entity::Entity;
use vertix::{prelude::Instance, state::State};

use crate::balloon::BalloonType;
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
}
pub struct EntityPool {
    pub active: Vec<Entity>,
}
pub struct PooledObject{
    balloon_type: BalloonType,
    am_to_pool: usize,
}
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