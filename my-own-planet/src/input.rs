//! Systems for user input

use crate::{get_surface_point, DisplaceEdit, FormMode, MeshEdits, RandomRes, redo_planet_mesh, Planet};

use bevy::prelude::*;


/// The plugin for all input related systems
pub fn input_plugin(app: &mut App) {
    app
        .insert_resource(OldMousePos::default())
        .add_systems(Update, (input_test, rotate_planet))
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
    
const PLANET_ROT_SPEED: f32 = 0.001;

use std::f32::consts::TAU; 
use bevy::window::PrimaryWindow;


/// Allows the user to rotate the planet useing the mouse
fn rotate_planet(
    mut planet: Single<&mut Transform, With<Planet>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    window: Single<&Window, With<PrimaryWindow>>,
    mut old_pos: ResMut<OldMousePos>,
) {
    let draging_buttons = [MouseButton::Left, MouseButton::Middle, MouseButton::Right];
    let ref mut transform = *planet;

    let current_pos = window.cursor_position();

    if mouse_buttons.any_pressed(draging_buttons) { 
        if let Some(some_pos) = current_pos{
            if let Some(some_old_pos) = old_pos.0 {
                let delta_pos = some_pos - some_old_pos;
     
                transform.rotate_y(delta_pos.x * TAU * PLANET_ROT_SPEED);
                transform.rotate_x(delta_pos.y * TAU * PLANET_ROT_SPEED);
            }
        }
    }

    old_pos.0 = current_pos;

}


#[derive(Resource, Default)]
struct OldMousePos(Option<Vec2>);
