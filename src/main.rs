mod components;
mod systems;

use components::*;
use systems::*;
use bevy::prelude::*;
use bevy::math::Vec3;
use rand::Rng;

fn spawn_some_particles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials : ResMut<Assets<StandardMaterial>>,
) {
    let mut rng = rand::thread_rng();
    for _ in 1..10 {
        let mut rand_vel: Velocity = rng.gen();
        rand_vel.0 /= 10.0;
        commands
        .spawn(PbrComponents{
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                subdivisions: 8,
                radius: 0.02,
            })),
            material: materials.add(Color::rgb(0.9, 0.9, 0.9).into()),
            ..Default::default()
        })
        .with(Particle)
        .with(rand_vel)
        .with(Transform::default())
        .with(Acceleration::default())
        ;
    }
}


fn main() {
    App::build()
    .add_default_plugins()
    .add_startup_system(setup.system())
    .add_startup_system(spawn_some_particles.system())
    .add_system(update_position.system())
    .add_system(update_velocity.system())
    .run();
}

fn setup(mut commands: Commands) {
    commands
        .spawn(Camera3dComponents {
            transform: Transform::new(Mat4::face_toward(
                Vec3::new(-3.0, 0.0, 0.0), 
                Vec3::new(500.0, 0.0, 0.0), 
                Vec3::unit_z(),
            )),
            ..Default::default()
        })
        .spawn(UiCameraComponents::default());
}
