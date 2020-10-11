use bevy::prelude::{Query, Transform};
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