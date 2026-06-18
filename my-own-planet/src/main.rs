use bevy::{
    prelude::*,
    camera::ClearColor,
};
use rand_chacha::{ChaCha8Rng, rand_core::SeedableRng};

mod object;
use object::{displace_mesh_verts, DisplaceEdit, FormMode, get_surface_point};

mod input;
use input::input_plugin;


fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .insert_resource(PlanetRes::default())
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

// holds info about the planet
#[derive(Resource)]
struct PlanetRes{
    pos: Vec3,
    size: f32,
    ico_divisions: u32,
    color: Color,
}

impl Default for PlanetRes {
    fn default() -> Self {
        Self{
            pos: Vec3::new(-2.0, 0.0, 0.0),
            size: 2.0,
            ico_divisions: 32,
            color: Color::hsla(0.0, 0.5, 0.5, 1.0),
        }
    }
}


fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    planet_info: Res<PlanetRes>,
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

    let planet = Sphere::new(planet_info.size).mesh().ico(planet_info.ico_divisions).unwrap();

    // The user's planet
    commands.spawn((
        Mesh3d(meshes.add(planet)),
        MeshMaterial3d(materials.add(planet_info.color)),
        Transform::from_translation(planet_info.pos),
        Planet, 
        MeshEdits(Vec::new()),
    ));

}


fn redo_planet_mesh(
    planet_mesh: &mut Mesh3d, 
    edits: &MeshEdits,
    meshes: &mut ResMut<Assets<Mesh>>,
    planet_info: Res<PlanetRes>,
) {

    let mut planet = Sphere::new(planet_info.size).mesh().ico(planet_info.ico_divisions).unwrap();

    displace_mesh_verts(&mut planet, &(edits.0), planet_info.size);

    planet_mesh.0 = (*meshes).add(planet);
}










