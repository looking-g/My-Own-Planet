//! Functions used when adding mechs to the planet

use bevy::math::Vec3;
use bevy::prelude::Mesh;
use bevy::mesh::VertexAttributeValues;
use bevy::ecs::change_detection::ResMut;

use rand_chacha::{ChaCha8Rng, rand_core::SeedableRng}; 
use rand::TryRng;

use crate::RandomRes;

/// gets a random surface point on a the input circle
pub fn get_surface_point(
    circle_r: f32,
    circle_pos: Vec3,
    mut rand: ResMut<RandomRes>,
) -> Vec3{
    // gets a random direction
    let mut rand_dir = [0.0_f32; 3];
    for i in 0..3{ // gets a random number for each direction
        rand_dir[i] =  u32_frac(
            rand.0
                .try_next_u32()
                .unwrap()
        )
    }
    let rand_dir = Vec3::from_array(rand_dir).normalize() * circle_r;

    rand_dir + circle_pos
}

/// converts a 0.0-MAX u32 value to a -1.0-1.0 f32 value
fn u32_frac(x: u32) -> f32{
    let fx = x as f32;
    let half_max = u32::MAX as f32 / 2.0;

    (fx - half_max) / half_max
}

pub fn displace_mesh_verts(
    mesh: &mut Mesh,
) {
    if let Some(VertexAttributeValues::Float32x3(poses)) =
        mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION)
    {
        for pos in poses.iter_mut() {
            if pos[1] > 0.0 {
                pos[1] = 0.0;
            }
        }
    }

}










