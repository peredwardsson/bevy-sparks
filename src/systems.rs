use bevy::prelude::*;
use rand::Rng;
use crate::components::*;


pub fn update_velocity(time: Res<Time>, mut query: Query<(&mut Velocity, &Acceleration)>) {
    for (mut vel, acc) in query.iter_mut() {
        vel.0 += acc.0 * time.delta_seconds;
    }
}

pub fn update_position(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, vel) in query.iter_mut() {
        transform.translation += vel.0 * time.delta_seconds;
    }
}

pub fn update_life(mut commands: Commands, time: Res<Time>, mut query: Query<(&mut Lifetime, Entity)>) {
    for (mut lifetime, entity) in query.iter_mut(){
        (*lifetime).0 -= time.delta_seconds;
        if lifetime.0 < 0.0 {
            commands.despawn(entity);
        }
    }
}

pub fn update_circular_motion( time: Res<Time>, mut query: Query<(&mut Velocity, &CircularMotion)>) {
    for (mut vel, ang_vel) in query.iter_mut() {
        let ang_vel_dt: Vec3 = ang_vel.angular_velocity*time.delta_seconds;
        let scaling_factor = 1.0;
        let dv: Vec3 = vel.0.cross(ang_vel_dt) * scaling_factor;
        vel.0 += dv;
        vel.0 = vel.0.normalize();
    }
}

pub fn spawn_particles(
    mut commands: Commands, 
    time: Res<Time>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut query: Query<(&ParticleSystem, &mut SpawnFrequency, &Transform, &Radius, &SystemLifetime, &Color)>, 
) {
    for (_ps, mut hz, transform, radius, _lifetime, color) in query.iter_mut(){
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
                    material: materials.add((*color).into()),
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