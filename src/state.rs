use bevy::{prelude::*, input::mouse::{MouseMotion, MouseWheel}};

#[derive(Default)]
pub struct State {
    pub mouse_motion_event_reader: EventReader<MouseMotion>,
    pub mouse_wheel_event_reader: EventReader<MouseWheel>,
}
