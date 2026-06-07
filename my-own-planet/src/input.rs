//! Systems for user input

use crate::{get_surface_point, DisplaceEdit, FormMode, MeshEdits, RandomRes, redo_planet_mesh, Planet};

use bevy::prelude::*;


/// The plugin for all input related systems
pub fn input_plugin(app: &mut App) {
    app
        .add_systems(Update, input_test)
    ;
}

/// Runs `add_crater` when C is pressed
fn input_test(
    mut planet: Single<(&mut MeshEdits, &mut Mesh3d), With<Planet>>,
    rand: ResMut<RandomRes>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    meshes: ResMut<Assets<Mesh>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyC) {
        let (ref mut edits, ref mut planet_mesh) = *planet;
        add_crater(
            edits,
            0.3,
            rand,
        );
        redo_planet_mesh(
            planet_mesh,
            edits,
            meshes,
        )
    }
}

/// Adds a new crator to a random spot
fn add_crater(
    edits: &mut MeshEdits,
    crater_radius: f32,
    rand: ResMut<RandomRes>,
) {
    edits.0.push(
        DisplaceEdit::Circle{
            pos: get_surface_point(1.0, Vec3::ZERO, rand),
            r: crater_radius,
            mode: FormMode::Sub,
        }
    );

}
    

