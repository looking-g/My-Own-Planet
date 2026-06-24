//! Systems used for exporting the user's planet

use bevy::prelude::*;

pub fn export_planet<'a>(
    mesh: Mesh,
) {
    let obj_file_data: String = mesh_to_obj(mesh)
        .expect("planet mesh should always have all data needed; this function only used for planet mesh");

    #[cfg(not(target_arch = "wasm32"))]
    non_wasm_export::export_by_file_dialog(obj_file_data);

    #[cfg(target_arch = "wasm32")]
    {
        match wasm_export::export_by_file_to_web(obj_file_data) {
            Ok(_) => (),
            Err(e) => wasm_export::alert(&format!("error: {:?}", e)), 
        }
    }
}


#[cfg(not(target_arch = "wasm32"))]
mod non_wasm_export {
    use rfd::FileDialog;

    pub fn export_by_file_dialog(obj_file: String) {
        let path = FileDialog::new()
            .set_directory("~/")
            .set_title("Saving planet as .obj")
            .pick_folder();
        let Some(mut path) = path else { return; };
        path.push("planet.obj");
      
        let _ = std::fs::write(path.as_path(), obj_file);
    }
}

#[cfg(target_arch = "wasm32")]
mod wasm_export { 
    use wasm_bindgen::{
        prelude::*,
        JsCast, Clamped,
    };
    use web_sys::{Element, Blob, Url, HtmlAnchorElement};

    #[wasm_bindgen]
    extern "C" {
        pub fn alert(s: &str);
    }

    #[wasm_bindgen]
    // the idia is that
    // obj -> blob -> url + doc -> downloded file on the computer
    pub fn export_by_file_to_web(obj_file: String) -> Result<(), JsValue> {

        // obj -> Blob(obj) -> URL(obj) (efectivaly)
        let value: Clamped<Vec<String>> = wasm_bindgen::Clamped(Vec::from([obj_file]));
        let blob_parts: JsValue = value.into();
        let blob = Blob::new_with_str_sequence(&blob_parts)?;
        let url: String = Url::create_object_url_with_blob(&blob)?;
     
        // Element -> HtmlAnchorElement(URL(obj)) (efectivaly)
        let window = web_sys::window().expect("window should exist");
        let document = window.document().expect("window should have a document");
        let element: Element = document.create_element("a")?; 
        let anchor: HtmlAnchorElement = element.dyn_into()?;
        anchor.set_href(&url);
        anchor.set_download("planet.obj");
        
        anchor.click();
        Url::revoke_object_url(&url)?;
        Ok(())
    }
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









