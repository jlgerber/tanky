
// forward speed
pub struct Velocity(pub f32);

// total rotation about z 
pub struct Orientation(pub f32);

// rate of orientation change
pub struct Rotation(pub f32);


pub struct Tank {
    pub rx_rate: f32,
    pub acc_rate: f32,
}
