use bevy::prelude::*;
use rand::Rng;
use crate::components::*;


pub fn update_velocity(mut query: Query<(&mut Velocity, &Acceleration)>) {
    for (mut vel, acc) in &mut query.iter() {
        vel.0 += acc.0;
    }
}

pub fn update_position(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, vel) in &mut query.iter() {
        transform.translate(vel.0);
        //println!("Updating pos: {:?}", pos);
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

pub fn spawn_particles(
    mut commands: Commands, 
    time: Res<Time>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut query: Query<(&ParticleSystem, &mut SpawnFrequency, &Position, &Radius)>, 

) {
    for (_ps, mut hz, position, radius) in &mut query.iter() {
        (*hz).0.tick(time.delta_seconds);
        if hz.0.just_finished {
            let mut rng = rand::thread_rng();
            let mut rand_vel: Velocity = rng.gen();
                rand_vel.0 /= 100.0;
            let rand_lifetime: Lifetime = rng.gen();

            let pos = Transform::from_translation(position.0);

            commands
                .spawn(PbrComponents{
                    mesh: meshes.add(Mesh::from(shape::Icosphere {
                        subdivisions: 8,
                        radius: radius.0,
                        ..Default::default()
                    })),
                    material: materials.add(Color::rgb(0.9, 0.7, 0.7).into()),
                    ..Default::default()
                })
                .with(Particle)
                .with(rand_vel)
                .with(pos)
                .with(Acceleration::default())
                .with(rand_lifetime)
                ;
                }
    }
}
