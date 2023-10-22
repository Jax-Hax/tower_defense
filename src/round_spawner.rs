use serde::Deserialize;
use vertix::{loader::load_string, state::State};
#[derive(Deserialize,Debug)]
struct WaveJSON {
    enemy_type: usize,
    time_btw_enemies: f32,
    is_camo: usize,
    enemy_am: usize,
}
#[derive(Deserialize,Debug)]
struct RoundJSON {
    time_btw_waves: f32,
    repeat_am: usize,
    waves: Vec<WaveJSON>,
}

#[derive(Deserialize,Debug)]
struct RoundsJSONData {
    rounds: Vec<RoundJSON>
}
//td specific
pub async fn load_rounds(json_file: &str, state: &mut State) {
    let json_string = load_string(json_file, env!("OUT_DIR")).await.unwrap();
    let enemy_data: RoundsJSONData =
        serde_json::from_str(&json_string).expect("Rounds JSON was not well-formatted");
    println!("{:#?}", enemy_data);
    /*let mut enemy_types = vec![];
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
            am_to_pool: enemy.pool_am
        })
    }
    state.world.insert_resource(EnemyTypes {types: enemy_types});
    pool_object(pooled_objects, state).await;*/
}