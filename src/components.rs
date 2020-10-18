use bevy::math::Vec3;
use bevy::prelude::Timer;

use rand::Rng;
use rand::distributions::{Distribution, Standard};

#[derive(Default, Debug)]
pub struct Position(pub Vec3);

#[derive(Default)]
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