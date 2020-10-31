mod components;
mod systems;

use bevy::{
    asset::{AssetLoader, LoadContext},
    math::Vec3,
    prelude::*,
    type_registry::TypeUuid,
    utils::*,
};
use components::*;
use systems::*;

//use ron::de;
use serde::Deserialize;
//use anyhow::Error;

use std::{fs, time::Duration};

fn main() {
    App::build()
        .add_default_plugins()
        .init_resource::<ButtonMaterials>()
        .add_asset::<ParticleSystemSettings>()
        .init_asset_loader::<ParticleSystemSettingsAssetLoader>()
        .add_startup_system(setup.system())
        //.add_startup_system(setup_ui.system())
        .add_startup_system(add_particle_system.system())
        // Unsure if this staging is necessary but perhaps it's not bad to keep it tidy.
        .add_system_to_stage(stage::UPDATE, update_position.system())
        .add_system_to_stage(stage::UPDATE, update_velocity.system())
        .add_system_to_stage(stage::UPDATE, update_life.system())
        .add_system_to_stage(stage::POST_UPDATE, spawn_particles.system())
        .add_system_to_stage(stage::UPDATE, update_circular_motion.system())
        .run();
}

fn setup(mut commands: Commands, asset_server: ResMut<AssetServer>) {
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
        .spawn(UiCameraComponents::default());

    let _: Handle<ParticleSystemSettings> = asset_server.load("../assets/ps/ps1.json");
    //let _: Handle<ParticleSystemSettings> = asset_server.load("assets/fonts/FiraMono-Regular.ttf");
}

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
    _asset_server: Res<AssetServer>,
) {
    // This should really not be a thing for particle systems.
    let ps_mesh = Mesh::from(shape::Icosphere {
        subdivisions: 1,
        radius: 1e-9,
    });
    let ps = ParticleSystemSettings {
        position: Vec3::new(10.0, 0.0, 0.0),
        color: Color::rgba(1.0, 0.0, 0.0, 1.0),
        frequency_ms: 100,
        radius: 0.05,
        velocity: Vec3::new(0.0, 1.0, 0.0),
        angular_velocity: true
    };

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
        .with(CircularMotion{angular_velocity: Vec3::unit_x()});
}
