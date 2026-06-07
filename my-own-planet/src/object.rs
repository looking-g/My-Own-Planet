//! Functions used when adding mechs to the planet

use bevy::math::Vec3;
use bevy::prelude::Mesh;
use bevy::mesh::{VertexAttributeValues, Indices};
use bevy::ecs::change_detection::ResMut;

use rand_chacha::{ChaCha8Rng, rand_core::SeedableRng}; 
use rand::TryRng;

use crate::RandomRes;

// random point

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

// high level mesh editing
//use 
use bevy::math::curve::EaseFunction;
use bevy::prelude::Curve;

#[derive(Clone, Copy)]
pub enum FormMode{
    Add,
    Sub,
}

/// used for storing the vertex displacement edit
pub enum DisplaceEdit{
    Circle{
        pos: Vec3,
        r: f32,
        mode: FormMode,
    },
    // use for mountins
    // Cone{ TODO }
}

impl DisplaceEdit{
    /// vertex's distence from the center should be `current dist + output` 
    pub fn get_displace(&self, point: Vec3) -> f32 {
        let mode: FormMode;
        let new_depth = match self {
            DisplaceEdit::Circle{pos, r, mode: m} => {
                let sq_dist = point.distance_squared(*pos);
                let max_dist = r * r;
                // 0 = point is out side the affected aria
                // 1 = point is at the center of the effected aria
                let norm_dist = (-sq_dist/max_dist) + 1.0;
                let norm_dist = norm_dist.clamp(0.0, 1.0);

                mode = *m;
                EaseFunction::CircularIn.sample(norm_dist).unwrap_or(0.0) * r
            },

        };

        match mode{
            FormMode::Add => new_depth,
            FormMode::Sub => -new_depth,
        }

    }

}

// low level mesh editing

pub fn displace_mesh_verts(
    mesh: &mut Mesh,
) {

    let Some(VertexAttributeValues::Float32x3(poses)) =
        mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION) else {
        return ();
    };

    // vertex displacement
    let edits = Vec::from([
        DisplaceEdit::Circle{
            pos: Vec3::Z,
            r: 0.5,
            mode: FormMode::Sub,
        }
    ]);


    for edit in edits.iter() {
        for pos in poses.iter_mut() {
            let vec3pos: Vec3 = Vec3::from_array(*pos);
            let pos_direction = vec3pos.normalize();
            let pos_dist = vec3pos.length();

            [pos[0], pos[1], pos[2]] = Vec3::to_array(&(
                pos_direction * (pos_dist + edit.get_displace(vec3pos))
            ));
        }
    }

    // recalculating vertex normals 
    
    vert_norm_update(mesh);
}

/// updates the vertex normals of a mesh
fn vert_norm_update(mesh: &mut Mesh) {

    // recalculating vertex normals 
    
    let Some(VertexAttributeValues::Float32x3(poses)) =
        mesh.attribute(Mesh::ATTRIBUTE_POSITION) else {
        return ();
    };

    let num_vertices = poses.len();

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








