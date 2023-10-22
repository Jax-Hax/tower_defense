use bevy_ecs::system::{Query, ResMut};
use glam::{Quat, Vec3};
use object_pooler::{pool_object, pool_from_json};
use vertix::{
    camera::{default_3d_cam, Camera},
    prelude::*,
};

use crate::enemy::enemy_movement;
mod enemy;
mod map;
mod object_pooler;
fn main() {
    pollster::block_on(run());
}
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {
    let camera = Camera::new(
        Vec3::new(0.0, 0.0,0.0),
        0.,
        0.,
    );
    // State::new uses async code, so we're going to wait for it to finish
    let (mut state, event_loop) = State::new(true, env!("OUT_DIR"), camera, 5.0, 2.0).await;
    //add models
    pool_from_json("enemy_types.json", &mut state).await;
    state.schedule.add_systems(enemy_movement);
    //render loop
    run_event_loop(
        state,
        event_loop,
        None,
    );
}