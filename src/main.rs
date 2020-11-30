use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};


fn main() {
    App::build()
    .add_plugins(DefaultPlugins)
    .add_plugin(HelloPlugin)
    .add_system(print_mouse_events_system.system())
    .run()
}

const  TXPATH: &str = env!("CARGO_MANIFEST_DIR");
pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .add_startup_system(setup.system())
        .add_system(tank_movement_system.system())
        ;
    }
}

struct Tank {
    velocity: f32,
    orientation: f32,
    rotation: f32,
    rx_rate: f32,
    acc_rate: f32,
    initialized: bool
}

const TO_RAD: f32 = std::f32::consts::PI / 180.0;

fn tank_movement_system(
    time: Res<Time>, 
    mut tank_query: Query<(&mut Tank, &mut Transform)>
) {
    let delta_seconds =  time.delta_seconds;
    for (mut tank, mut transform) in tank_query.iter_mut() {
        if tank.initialized && tank.velocity > 0.0 {
            tank.orientation += tank.rotation;

            let  matrx = Mat4::from_quat(
                Quat::from_rotation_z( tank.orientation * TO_RAD ));

            let mattx = Mat4::from_translation( Vec3::unit_y()* tank.velocity * delta_seconds * 100.0);
            let mat = matrx.mul_mat4(&mattx);
            let tx = Transform::from_matrix(mat);
            transform.translation += tx.translation;
            transform.rotation = if tank.velocity < 0.00001  { transform.rotation } else {tx.rotation};
        } else {
            tank.orientation += tank.rotation;
            let  matrx = Mat4::from_quat(
                Quat::from_rotation_z(tank.orientation * TO_RAD ));
           transform.rotation = Transform::from_matrix(matrx).rotation;
           tank.initialized = true;
        }
    }
}


fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    
    let mut tank1_path = std::path::PathBuf::from(TXPATH);
    let mut turret1_path = tank1_path.clone();
    let mut gun1_path = tank1_path.clone();
    tank1_path.push("assets/PNG/Hulls_Color_C/Hull_01.png");
    gun1_path.push("assets/PNG/Weapon_Color_C/Gun_01_A.png");
    turret1_path.push("assets/PNG/Weapon_Color_C/Gun_01_B.png");
    let texture_handle = asset_server.load(tank1_path);
    let turret1_handle = asset_server.load(turret1_path);
    let gun1_handle = asset_server.load(gun1_path);

    //let far = 4000.0;
    commands
        .spawn(Camera2dComponents::default())
        .spawn(SpriteComponents {
            material: materials.add(texture_handle.into()),
            transform: Transform::from_scale(Vec3::new(0.5,0.5,0.5)),
            ..Default::default()
        })
        .with(Tank {
            velocity: 0.0, 
            orientation: 45.0, 
            rotation: 0.0, 
            rx_rate: 0.002, 
            acc_rate: 0.1,
            initialized: false
        })
        .with_children(|parent| {
            parent.spawn(SpriteComponents {
                material: materials.add(turret1_handle.into()),
                transform: Transform::from_translation(Vec3::new(0.0,-20.0,2.0)),
                ..Default::default()
            })
            .with_children(|par| {
                par.spawn(SpriteComponents{
                    material: materials.add(gun1_handle.into()),
                    transform: Transform::from_translation(Vec3::new(0.0,100.0,1.0)),
                    ..Default::default()
                });
            });
        })
        ;
}



#[derive(Default)]
struct State {
    mouse_motion_event_reader: EventReader<MouseMotion>,
    mouse_wheel_event_reader: EventReader<MouseWheel>,
}

/// This system prints out all mouse events as they come in
fn print_mouse_events_system(
    mut state: Local<State>,
    mouse_motion_events: Res<Events<MouseMotion>>,
    mouse_wheel_events: Res<Events<MouseWheel>>,
    mouse_input: Res<Input<MouseButton>>,
   mut tank_query: Query<(
       &mut Tank, 
    )>
) {
    
    for event in state.mouse_motion_event_reader.iter(&mouse_motion_events) {
        if mouse_input.pressed(MouseButton::Left) {
            for (mut tank, 
            ) in tank_query.iter_mut() {
                tank.rotation -= event.delta.x() * tank.rx_rate;
            }
        }
    }


    for event in state.mouse_wheel_event_reader.iter(&mouse_wheel_events) {
        for (mut tank,) in tank_query.iter_mut() {
            tank.velocity = (tank.velocity + event.y * tank.acc_rate).max(0.0);
        }
    }
}
