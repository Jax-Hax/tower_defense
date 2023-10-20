use bevy_ecs::system::{Query, ResMut};
use glam::{Quat, Vec3};
use vertix::{
    camera::{default_3d_cam, Camera},
    prelude::*,
};

use crate::balloon::balloon_movement;
mod balloon;
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
    const SPACE_BETWEEN: f32 = 3.0;
    const NUM_INSTANCES_PER_ROW: usize = 1000;
    let mut instances = vec![];
    for x in 0..NUM_INSTANCES_PER_ROW {
        for y in 0..NUM_INSTANCES_PER_ROW {
                let x = SPACE_BETWEEN * (x as f32 - NUM_INSTANCES_PER_ROW as f32 / 2.0);
                let y = SPACE_BETWEEN * (y as f32 - NUM_INSTANCES_PER_ROW as f32 / 2.0);

                let position = Vec3 { x, y, z: 0. };

                let rotation = Quat::from_axis_angle(position.normalize(), f32::to_radians(45.0));

                let instance = Instance { position, rotation, ..Default::default() };
                
                instances.push((instance,));
        }
    }
    state
        .create_model_instances("cube.obj", instances.iter_mut().map(|(instance, )| instance).collect(), true)
        .await;
    state.world.spawn_batch(instances);
    state.schedule.add_systems(balloon_movement);
    state.world.
    //render loop
    run_event_loop(
        state,
        event_loop,
        None,
    );
}