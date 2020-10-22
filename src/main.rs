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
    .init_resource::<ButtonMaterials>()
    .add_startup_system(setup.system())
    .add_startup_system(add_particle_system.system())
    // Unsure if this staging is necessary but perhaps it's not bad to keep it tidy.
    .add_system_to_stage(stage::UPDATE, update_position.system())
    .add_system_to_stage(stage::UPDATE, update_velocity.system())
    .add_system_to_stage(stage::UPDATE, update_life.system())
    .add_system_to_stage(stage::POST_UPDATE, spawn_particles.system())
    .run();
}

fn setup(mut commands: Commands, mut materials: Res<ButtonMaterials>) {
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
            light: Light{
                fov: f32::to_radians(360.0),
                ..Default::default()},
            ..Default::default()
        })
        .spawn(LightComponents{
            global_transform: GlobalTransform::from_translation(Vec3::new(0.0, 0.0,100.0)),
            ..Default::default()
        })
        .spawn(UiCameraComponents::default())
        .spawn(NodeComponents {
            style: Style {
                size: Size::new(Val::Px(250.0), Val::Px(65.0)),
                // center button
                margin: Rect::all(Val::Px(5.0)),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: materials.normal,
            ..Default::default()
        })
            .with_child(|parent| {
                parent.spawn(
                    TextComponents {
                        
                    }
                )
            })
        ;
}


fn add_particle_system(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    // This should really not be a thing for particle systems.
    let ps_mesh = Mesh::from(shape::Icosphere {
        subdivisions: 1,
        radius: 1e-9,
    });

    commands
        .spawn(PbrComponents{
            mesh: meshes.add(ps_mesh),
            material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..Default::default()
        })
        .with(ParticleSystem)
        .with(SpawnFrequency(Timer::new(Duration::from_millis(100), true)))
        .with(Radius(0.05))
        .with(Velocity(Vec3::new(0.0, 1.0, 0.0)));
}