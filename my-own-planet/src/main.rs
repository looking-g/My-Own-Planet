use bevy::prelude::*;
use rand_chacha::{ChaCha8Rng, rand_core::SeedableRng};

mod object;
use object::{displace_mesh_verts, DisplaceEdit, FormMode, get_surface_point};

mod input;
use input::input_plugin;


fn main() {
    App::new()
        .add_plugins((DefaultPlugins, input_plugin))
        .add_systems(Startup, setup)
        .run();
}

#[derive(Resource)]
struct RandomRes(ChaCha8Rng);


#[derive(Component)]
struct MeshEdits(Vec<DisplaceEdit>);

#[derive(Component)]
struct Planet;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let seeded_rng = ChaCha8Rng::seed_from_u64(94757448641217);
    commands.insert_resource(RandomRes(seeded_rng));

    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    let planet = Sphere::new(1.0).mesh().ico(32).unwrap();

    // The user's planet
    commands.spawn((
        Mesh3d(meshes.add(planet)),
        MeshMaterial3d(materials.add(Color::srgb_u8(255, 255, 255))),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Planet, 
        MeshEdits(Vec::new()),
    ));

}


fn redo_planet_mesh(
    planet_mesh: &mut Mesh3d, 
    edits: &MeshEdits,
    mut meshes: ResMut<Assets<Mesh>>,
) {

    let mut planet = Sphere::new(1.0).mesh().ico(32).unwrap();

    displace_mesh_verts(&mut planet, &(edits.0));

    planet_mesh.0 = meshes.add(planet);
}










