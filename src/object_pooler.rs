use crate::enemy::{Enemy, EnemyType, EnemyTypes};
use bevy_ecs::{entity::Entity, system::Resource};
use serde::Deserialize;
use vertix::{loader::load_string, prelude::Instance, state::State};
#[derive(Deserialize, Debug)]
struct EnemyJSON {
    name: String,
    starting_health: usize,
    speed: f32,
    damage_dealt: isize,
    image_file: String,
    children: Vec<usize>, // Assuming children is an array of integers
    pool_am: usize,
}

#[derive(Deserialize, Debug)]
struct EnemyJSONData {
    enemies: Vec<EnemyJSON>,
}
//td specific
pub async fn pool_from_json(json_file: &str, state: &mut State) {
    let json_string = load_string(json_file, env!("OUT_DIR")).await.unwrap();
    let enemy_data: EnemyJSONData =
        serde_json::from_str(&json_string).expect("Enemy type JSON was not well-formatted");
    let mut enemy_types = vec![];
    let mut pooled_objects = vec![];
    for enemy in enemy_data.enemies {
        let enemy_type = EnemyType {
            starting_health: enemy.starting_health,
            speed: enemy.speed,
            damage_dealt: enemy.damage_dealt,
            children: enemy.children,
            image_file: enemy.image_file,
        };
        enemy_types.push(enemy_type.clone());
        pooled_objects.push(PooledObject {
            enemy_type,
            am_to_pool: enemy.pool_am,
        })
    }
    state
        .world
        .insert_resource(EnemyTypes { types: enemy_types });
    pool_object(pooled_objects, state).await;
}
//pooler in general
pub async fn pool_object(pool_vec: Vec<PooledObject>, state: &mut State) {
    let mut pooler = ObjectPooler {
        pooled_objects: vec![],
    };
    for pool_object in pool_vec {
        let mut instances = vec![
            (
                Instance {
                    ..Default::default()
                },
                Enemy {
                    health: 0,
                    bloon_type: 0,
                    way_point_index: 0
                }
            );
            pool_object.am_to_pool
        ];
        state.make_sprites(
            instances.iter_mut().map(|(instance, _)| instance).collect(),
            state
                .compile_material(&pool_object.enemy_type.image_file)
                .await, true
        );
        let entities = state.world.spawn_batch(instances).collect::<Vec<Entity>>();
        pooler.pooled_objects.push(EntityPool { active: entities })
    }
    state.world.insert_resource(pooler);
}
pub struct EntityPool {
    pub active: Vec<Entity>,
}
pub struct PooledObject {
    enemy_type: EnemyType,
    am_to_pool: usize,
}
#[derive(Resource)]
pub struct ObjectPooler {
    pooled_objects: Vec<EntityPool>,
}
impl ObjectPooler {
    pub fn set_inactive(&mut self, index: usize, entity: Entity) {
        self.pooled_objects[index].active.push(entity);
    }
    pub fn get_inactive(&mut self, index: usize) -> Entity {
        self.pooled_objects[index].active.pop().unwrap()
    }
}
