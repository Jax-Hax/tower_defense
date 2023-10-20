use bevy_ecs::entity::Entity;
use vertix::prelude::Instance;

use crate::balloon::BalloonType;
pub fn pool_object(pool_vec: Vec<PooledObject>) {
    let pooler = vec![];
    for pool_object in pool_vec {
        let instances = vec![(Instance { ..Default::default() },); pool_object.am_to_pool];
    }
    
    state
        .create_model_instances("cube.obj", instances.iter_mut().map(|(instance, )| instance).collect(), true)
        .await;
    state.world.spawn_batch(instances);
    state.schedule.add_systems(balloon_movement);
}
struct EntityPool {
    active: Vec<Entity>,
    inactive: Vec<Entity>
}
struct PooledObject{
    balloon_type: BalloonType,
    am_to_pool: usize,
}