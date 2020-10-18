mod components;
mod systems;

use components::*;
use systems::*;
use bevy::prelude::*;
use bevy::math::Vec3;

use std::time::Duration;

fn main() {
    App::build()
    .add_default_plugins()
    .add_startup_system(setup.system())
    //.add_plugin(ScheduleRunnerPlugin::run_loop(Duration::from_secs(1)))
    //.add_system(spawn_some_particles.system())
    .add_system_to_stage(stage::UPDATE, update_position.system())
    .add_system_to_stage(stage::UPDATE, update_velocity.system())
    .add_system_to_stage(stage::UPDATE, update_life.system())
    .add_system_to_stage(stage::POST_UPDATE, spawn_particles.system())
    .run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    commands
        .spawn(Camera3dComponents {
            transform: Transform::new(Mat4::face_toward(
                // TODO: get a grasp on where the heck 
                // i) my particles are spawning
                // ii) what these parameters mean
                Vec3::new(-10.0, 0.0, 0.0), 
                Vec3::new(500.0, 0.0, 0.0), 
                Vec3::unit_z(),
            )),
            ..Default::default()
        })
        .spawn(UiCameraComponents::default())
        .spawn(LightComponents{
            global_transform: GlobalTransform::from_translation(Vec3::new(0.0,0.0,-100.0)),
            ..Default::default()
        })
        .spawn(LightComponents{
            global_transform: GlobalTransform::from_translation(Vec3::new(0.0, 0.0,100.0)),
            ..Default::default()
        })
        .spawn(PbrComponents{
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                subdivisions: 1,
                radius: 0.00001,
            })),
            material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
            ..Default::default()
        })
        .with(ParticleSystem)
        .with(SpawnFrequency(Timer::new(Duration::from_millis(200), true)))
        .with(Radius(0.05))
        .with(Position(Vec3::new(0.0, 0.0, 0.0)))
        ;
}
