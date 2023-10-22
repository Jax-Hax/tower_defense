use bevy_ecs::{system::{Resource, ResMut}, component::Component};
use serde::Deserialize;
use vertix::{loader::load_string, state::State, resources::Timer};

use crate::object_pooler::ObjectPooler;
#[derive(Deserialize)]
struct Wave {
    enemy_type: usize,
    time_btw_enemies: f32,
    is_camo: usize,
    enemy_am: usize,
}
#[derive(Deserialize)]
struct Round {
    time_btw_waves: f32,
    repeat_am: usize,
    waves: Vec<Wave>,
}

#[derive(Deserialize)]
struct Rounds {
    rounds: Vec<Round>
}
#[derive(Deserialize,Resource)]
pub struct RoundCounter {
    rounds: Rounds,
    round_num: usize,
    wave_num: usize,
}
#[derive(Component)]
struct FuseTime {
    /// track when the bomb should explode (non-repeating timer)
    timer: Timer,
}
//td specific
pub async fn load_rounds(json_file: &str, state: &mut State) {
    let json_string = load_string(json_file, env!("OUT_DIR")).await.unwrap();
    let rounds: Rounds =
        serde_json::from_str(&json_string).expect("Rounds JSON was not well-formatted");
    state.world.insert_resource(RoundCounter{rounds, round_num: 0, wave_num: 0})
}
pub fn spawn_rounds(mut pooler: ResMut<ObjectPooler>, mut round_counter: ResMut<RoundCounter>) {
    
}