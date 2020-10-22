use bevy::prelude::*;
use rand::Rng;
use crate::components::*;


pub fn update_velocity(time: Res<Time>, mut query: Query<(&mut Velocity, &Acceleration)>) {
    for (mut vel, acc) in &mut query.iter() {
        vel.0 += acc.0 * time.delta_seconds;
    }
}

pub fn update_position(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, vel) in &mut query.iter() {
        transform.translate(vel.0 * time.delta_seconds);
    }
}

pub fn update_life(mut commands: Commands, time: Res<Time>, mut query: Query<(&mut Lifetime, Entity)>) {
    for (mut lifetime, entity) in &mut query.iter() {
        (*lifetime).0 -= time.delta_seconds;
        if lifetime.0 < 0.0 {
            commands.despawn(entity);
        }
    }
}

pub fn update_circular_motion( time: Res<Time>, mut query: Query<(&mut Velocity, &CircularMotion)>) {
    for (mut vel, _) in &mut query.iter() {
        let mut ang_vel: Vec3 = Vec3::unit_x()*time.delta_seconds;
        if ang_vel.max_element() == 0.0 {
            ang_vel = Vec3::unit_x();
        }
        //println!("{:?}", vel);
        //println!("{:?}", ang_vel);
        let scaling_factor = 1.0;
        let dv: Vec3 = vel.0.cross(ang_vel) * scaling_factor;
        vel.0 = vel.0 + dv;
        vel.0 = vel.0.normalize();
    }
}

pub fn spawn_particles(
    mut commands: Commands, 
    time: Res<Time>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut query: Query<(&ParticleSystem, &mut SpawnFrequency, &Transform, &Radius)>, 
) {
    for (_ps, mut hz, transform, radius) in &mut query.iter() {
        (*hz).0.tick(time.delta_seconds);
        if hz.0.just_finished {
            let mut rng = rand::thread_rng();
            let rand_vel: Velocity = rng.gen();
            let rand_lifetime: Lifetime = rng.gen();

            commands
                .spawn(PbrComponents{
                    mesh: meshes.add(Mesh::from(shape::Icosphere {
                        subdivisions: 8,
                        radius: radius.0,
                        ..Default::default()
                    })),
                    material: materials.add(Color::rgb(0.9, 0.7, 0.7).into()),
                    transform: transform.clone(),
                    ..Default::default()
                })
                .with(Particle)
                .with(rand_vel)
                .with(Acceleration::default())
                .with(rand_lifetime)
                ;
                }
    }
}