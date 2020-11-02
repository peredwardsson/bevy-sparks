mod components;
mod systems;

use bevy::{
    math::Vec3,
    prelude::*,
};
use components::*;
use systems::*;

use std::{time::Duration, path::PathBuf};
use glob::glob;

fn main() {
    App::build()
        .add_default_plugins()
        .init_resource::<ButtonMaterials>()
        .add_asset::<ParticleSystemSettings>()
        .init_asset_loader::<ParticleSystemSettingsAssetLoader>()
        .add_startup_system(setup.system())
        .add_startup_system(create_ps_from_files.system())
        //.add_startup_system(setup_ui.system())
        .add_system(add_particle_system.system())
        // Unsure if this staging is necessary but perhaps it's not bad to keep it tidy.
        .add_system_to_stage(stage::UPDATE, update_position.system())
        .add_system_to_stage(stage::UPDATE, update_velocity.system())
        .add_system_to_stage(stage::UPDATE, update_life.system())
        .add_system_to_stage(stage::UPDATE, update_circular_motion.system())
        .add_system_to_stage(stage::POST_UPDATE, spawn_particles.system())
        .run();
}

fn setup(mut commands: Commands) {
    
    commands
        .spawn(Camera3dComponents {
            transform: Transform::from_translation(Vec3::zero().into())
                .looking_at(Vec3::unit_x(), Vec3::unit_z()),
            ..Default::default()
        })
        .spawn(LightComponents {
            global_transform: GlobalTransform::from_translation(Vec3::new(0.0, 0.0, -100.0)),
            light: Light {
                fov: f32::to_radians(360.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .spawn(UiCameraComponents::default())
        ;
    }

#[allow(dead_code)]
fn setup_ui(
    mut commands: Commands,
    materials: ResMut<ButtonMaterials>,
    asset_server: Res<AssetServer>,
) {
    // Side note: rather than using a UI at this stage, maybe it is better to use a file to load parameters
    // and enable hot loading. Food for thought.
    commands
        .spawn(NodeComponents {
            // Background
            style: Style {
                size: Size::new(Val::Px(250.0), Val::Px(65.0)),
                // center button
                margin: Rect::all(Val::Px(5.0)),
                // horizontally center child text
                justify_content: JustifyContent::FlexStart,
                // vertically center child text
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: materials.normal.clone(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(TextComponents {
                text: Text {
                    value: "Status".to_string(),
                    font: asset_server.load("assets/fonts/FiraMono-Regular.ttf"),
                    style: TextStyle {
                        font_size: 20.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                },
                ..Default::default()
            });
            parent.spawn(TextComponents {
                text: Text {
                    value: "Boopus".to_string(),
                    font: asset_server.load("assets/fonts/FiraMono-Regular.ttf"),
                    style: TextStyle {
                        font_size: 20.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                },
                ..Default::default()
            });
        });
}

fn add_particle_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    ps_settings: Res<Assets<ParticleSystemSettings>>,
    mut ps_query: Query<(&ParticleSystemSettingsHandle, &mut SpawnCounter)>
) {
    
    for (handler, mut spawn_counter) in ps_query.iter_mut() {
        if let Some(ps) = ps_settings.get(&handler.handle){
            let ps_mesh = Mesh::from(shape::Icosphere {
                subdivisions: 1,
                radius: 1e-9,
            });
            if spawn_counter.spawned >= spawn_counter.max_spawn {
                break;
            }
            println!("Spawning new PS!");
            commands
                .spawn(PbrComponents {
                    mesh: meshes.add(ps_mesh),
                    material: materials.add(ps.color.into()),
                    transform: Transform::from_translation(ps.position),
                    ..Default::default()
                })
                .with(ParticleSystem)
                .with(SpawnFrequency(Timer::new(Duration::from_millis(ps.frequency_ms), true)))
                .with(Radius(ps.radius))
                .with(Velocity(ps.velocity))
                .with(CircularMotion{angular_velocity: ps.angular_velocity})
                .with(Color::from(ps.color))
                .with(SystemLifetime(3.1));
            (*spawn_counter).spawned += 1;
        }
    }    
}


pub fn create_ps_from_files(mut commands: Commands, asset_server: Res<AssetServer>) {

    for path in glob("assets/ps/*").expect("No particle systems found.") {
        if let Ok(p) = path {

            let mut pathmod = PathBuf::from("..\\");
            pathmod.push(p);

            let handle: Handle<ParticleSystemSettings> = asset_server.load(pathmod);
            asset_server.watch_for_changes().unwrap();

            commands
                .spawn(ParticleSystemSpawner {
                    name: "ps1".into(),
                })
                .with(ParticleSystemSettingsHandle{handle})
                .with(SpawnCounter {spawned: 0, max_spawn: 1});
        }
    }
}