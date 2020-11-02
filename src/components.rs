use bevy::{
    asset::*, 
    math::Vec3, 
    prelude::*, 
    utils::*,
    type_registry::TypeUuid,
};

use rand::Rng;
use rand::distributions::{Distribution, Standard};
use serde::Deserialize;
use serde_json::Value;

#[derive(Default, Debug)]
pub struct Position(pub Vec3);

#[derive(Default, Debug)]
pub struct Velocity(pub Vec3);

impl Distribution<Velocity> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Velocity {
        let (rand_x, rand_y, rand_z): (f32, f32, f32) = rng.gen();
        let scaling_factor = 0.5;
        Velocity (
            Vec3::new(rand_x-0.5, rand_y-0.5, rand_z-0.5)*scaling_factor
        )
    }
}

#[derive(Default)]
pub struct Acceleration(pub Vec3);

pub struct Particle;

#[derive(Default, Debug)]
pub struct Lifetime(pub f32);

impl Distribution<Lifetime> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Lifetime {
        let life: f32 = rng.gen();
        let scaling_factor = 3.0;
        Lifetime(life*scaling_factor)
    }
}

pub struct SpawnFrequency(pub Timer);

pub struct ParticleSystem;

pub struct Radius(pub f32);

pub struct ButtonMaterials {
    pub normal: Handle<ColorMaterial>,
    pub hovered: Handle<ColorMaterial>,
    pub pressed: Handle<ColorMaterial>,
}

impl FromResources for ButtonMaterials {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        ButtonMaterials {
            normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
            pressed: materials.add(Color::rgb(0.35, 0.75, 0.35).into()),
        }
    }
}

pub struct CircularMotion {
    pub angular_velocity: Vec3
}



#[derive(Debug, Default, Deserialize, TypeUuid)]
#[uuid = "a48e9156-1b55-11eb-adc1-0242ac120002"]
pub struct ParticleSystemSettings {
    pub position: Vec3,
    pub color: Color,
    pub frequency_ms: u64,
    pub velocity: Vec3,
    pub radius: f32,
    pub angular_velocity: Vec3
}

#[derive(Default)]
pub struct ParticleSystemSettingsAssetLoader;

impl AssetLoader for ParticleSystemSettingsAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let path = load_context.path();
            println!("PS loaded: {}", path.display());
            let contents = std::str::from_utf8(bytes).unwrap();
            //let s = serde_json::from_str(contents);
            if let Ok(s) = serde_json::from_str::<ParticleSystemSettings>(contents) {
                println!("Settings: {:?}", &s);
                load_context.set_default_asset(LoadedAsset::new(s));
                println!("Set the load context successfully");
            }
            //let settings = serde_json::from_str(contents);
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["json"]
    }
}

#[derive(Debug, Default)]
pub struct SpawnCounter {
    pub spawned: i32,
    pub max_spawn: i32,
}

#[derive(Bundle, Default, Debug)]
pub struct ParticleSystemSpawner {
    // pub ps_handle: Handle<ParticleSystemSettings>,
    pub name: String,
}

pub struct ParticleSystemSettingsHandle {
    pub handle: Handle<ParticleSystemSettings>,
}