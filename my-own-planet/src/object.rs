//! Functions used when adding mechs to the planet

use bevy::math::Vec3;
use bevy::prelude::Mesh;
use bevy::mesh::{VertexAttributeValues, Indices};
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
    let num_vertices;

    // vertex displacement
    if let Some(VertexAttributeValues::Float32x3(poses)) =
        mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION)
    {
        num_vertices = poses.len();
        for pos in poses.iter_mut() {
            if pos[1] > 0.0 {
                pos[1] = 0.0;
            }
        }
    } else {
        return ();
    }

    // recalculating vertex normals 
    
    let Some(VertexAttributeValues::Float32x3(poses)) =
        mesh.attribute(Mesh::ATTRIBUTE_POSITION) else {
        return ();
    };

    let Some(Indices::U32(indices)) =
        mesh.indices() else {
        return ();
    };


    // creates a vec with the pos data for every triangle in the mesh
    let num_triangles = indices.len()/3;
    // Vec<triangle((pos, where in list) × 3)> 
    let mut triangles: Vec<[(Vec3, u32); 3]> = Vec::with_capacity(num_triangles);
    for i in 0..num_triangles{ 
        triangles.push([
            ( poses[ indices[i * 3]     as usize ].into(), indices[i * 3]     ),
            ( poses[ indices[i * 3 + 1] as usize ].into(), indices[i * 3 + 1] ),
            ( poses[ indices[i * 3 + 2] as usize ].into(), indices[i * 3 + 2] ),
        ]);
    }

    // replaces triangle positions with triangle normals
    let tri_norms: Vec<[(Vec3, u32); 3]> = triangles.into_iter()
        .map(|[mut p1, mut p2, mut p3]| {
            let norm = ((p2.0 - p1.0).cross(p3.0 - p1.0)).normalize();
            [p1.0, p2.0, p3.0] = [norm; 3];
            [p1, p2, p3]
        })
        .collect();
    
    // transitioning the data from being stored in groupes of triangles to a normal flat vec
    let mut normals: Vec<AverageVec3> = vec![AverageVec3::new(); num_vertices];

    // AverageVec3 is used so that each vertex is the average of all faces that ues it
    for triangle in tri_norms{
        for (norm, index) in Vec::from(triangle){
            normals[index as usize].add(norm);
        }
    }

    // convert Vec<AverageVec3> to Vec<[f32; 3]>
    let normals: Vec<[f32; 3]> = normals.into_iter()
        .enumerate()
        // just unwraps the value or gets the direction of the point from (0, 0, 0)
        .map(|(i, avg)| Vec3::to_array( &avg.solve().unwrap_or( Vec3::from_array(poses[i]).normalize() ) ))
        .collect();


    // sets the meshes current normal vec to the new one 
    if let Some(VertexAttributeValues::Float32x3(mesh_norms)) =
        mesh.attribute_mut(Mesh::ATTRIBUTE_NORMAL)
    {
        *mesh_norms = normals;
    }


}

/// keeps track of vales that are to be averaged
#[derive(Clone)]
struct AverageVec3{
    sum: Vec3,
    count: u32,
}

impl AverageVec3{
    /// makes a new counter
    fn new() -> Self{
        Self{
            sum: Vec3::ZERO,
            count: 0,
        }
    }

    /// adds a value to the count
    fn add(&mut self, val: Vec3){
        self.sum += val;
        self.count += 1_u32;
    }

    /// calculates the average
    fn solve(&self) -> Option<Vec3>{
        if self.count == 0{
            None
        } else {
            Some(self.sum / self.count as f32)
        }
    }

}








