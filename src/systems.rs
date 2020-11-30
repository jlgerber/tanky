use bevy::{prelude::*, input::mouse::{MouseMotion, MouseWheel}};

use crate::components::*;
use crate::state::*;
use crate::constants::{TO_RAD, VELOCITY_SCALAR};


pub fn tank_movement_system(
    time: Res<Time>, 
    mut tank_query: Query<(&mut Tank, &mut Transform, &Velocity, &mut Orientation, &Rotation)>
) {
    let delta_seconds =  time.delta_seconds;
    for (_tank, mut transform, velocity, mut orientation, rotation) in tank_query.iter_mut() {
        if velocity.0 > 0.0 {
            orientation.0 += rotation.0;

            let  matrx = Mat4::from_quat(
                Quat::from_rotation_z( orientation.0 * TO_RAD ));

            let mattx = Mat4::from_translation( Vec3::unit_y() * velocity.0 * delta_seconds * VELOCITY_SCALAR);
            let mat = matrx.mul_mat4(&mattx);
            let tx = Transform::from_matrix(mat);
            transform.translation += tx.translation;
            transform.rotation = if velocity.0 < 0.00001  { transform.rotation } else {tx.rotation};
        } else {
            orientation.0 += rotation.0;
            let  matrx = Mat4::from_quat(
                Quat::from_rotation_z(orientation.0 * TO_RAD ));
           transform.rotation = Transform::from_matrix(matrx).rotation;
        }
    }
}

/// This event handles mouse events
pub fn mouse_events_system(
    mut state: Local<State>,
    mouse_motion_events: Res<Events<MouseMotion>>,
    mouse_wheel_events: Res<Events<MouseWheel>>,
    mouse_input: Res<Input<MouseButton>>,
    mut tank_query: Query<(&mut Tank, &mut Rotation, &mut Velocity)>
) {
    
    for event in state.mouse_motion_event_reader.iter(&mouse_motion_events) {
        if mouse_input.pressed(MouseButton::Left) {
            for (tank, mut rotation, _
            ) in tank_query.iter_mut() {
                rotation.0 -= event.delta.x() * tank.rx_rate;
            }
        }
    }


    for event in state.mouse_wheel_event_reader.iter(&mouse_wheel_events) {
        for (tank, _rotation, mut velocity) in tank_query.iter_mut() {
            velocity.0 = (velocity.0 + event.y * tank.acc_rate).max(0.0);
        }
    }
}

