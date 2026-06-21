//! Systems used for exporting the user's planet

use bevy::prelude::*;
use rfd::FileDialog;


pub fn export_planet<'a>(
    mesh: Mesh
    /*
    meshes: &mut Res<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,

    planet: Mesh3d,
    planet_material: MeshMaterial3d,

    mut asset_server: ResMut<AssetServer>,
    */
) {
    export_by_file_dialog(mesh);
}

#[cfg(not(target_arch = "wasm32"))]
fn export_by_file_dialog(mesh: Mesh) {
    let path = FileDialog::new()
        .set_directory("~/")
        .set_title("Saving planet as .obj")
        .pick_folder();
    let Some(mut path) = path else { return; };
    path.push("planet.obj");
  
    let _ = std::fs::write(path.as_path(), mesh_to_obj(mesh).unwrap());
}


use bevy::mesh::{VertexAttributeValues, Indices}; 
fn mesh_to_obj(mesh: Mesh) -> Option<String> {
    let mut obj = String::new();

    let Some(VertexAttributeValues::Float32x3(poses)) =
        mesh.attribute(Mesh::ATTRIBUTE_POSITION) else { return None; };
 
    let Some(Indices::U32(indices)) =
        mesh.indices() else { return None; };

    let Some(VertexAttributeValues::Float32x3(norms)) =
        mesh.attribute(Mesh::ATTRIBUTE_NORMAL) else { return None; };

    for pos in poses {
        obj.push_str(&format!("v {} {} {}\n", pos[0], pos[1], pos[2]));
    }

    for norm in norms {
        obj.push_str(&format!("vn {} {} {}\n", norm[0], norm[1], norm[2]));
    }
    
    for chunk in indices.chunks(3) {
        let [i1, i2, i3] = chunk else { continue; };

        obj.push_str("f"); 
        obj.push_str(&format!(" {i}//{i}", i = i1 + 1 )); 
        obj.push_str(&format!(" {i}//{i}", i = i2 + 1)); 
        obj.push_str(&format!(" {i}//{i}", i = i3 + 1)); 
        obj.push_str("\n");
    }
    
    Some(obj)
}









