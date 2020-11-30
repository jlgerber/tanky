use bevy::{
    input::mouse::{MouseButtonInput, MouseMotion, MouseWheel},
    prelude::*,
    window::CursorMoved,
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
    max_rx_rate: f32,
    initialized: bool
}

const TO_RAD: f32 = std::f32::consts::PI / 180.0;

fn tank_movement_system(
    time: Res<Time>, 
    keyboard_input: Res<Input<KeyCode>>,
    mut tank_query: Query<(&mut Tank, &mut Transform)>
) {
    let delta_seconds =  time.delta_seconds;
    for (mut tank, mut transform) in tank_query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            tank.rotation += if keyboard_input.pressed(KeyCode::LShift) {1.0} else {0.1};
            tank.rotation = tank.rotation.min(tank.max_rx_rate);
        } else if keyboard_input.pressed(KeyCode::Right) {
            tank.rotation -= if keyboard_input.pressed(KeyCode::LShift) {1.0} else {0.1};
            tank.rotation = tank.rotation.max(-tank.max_rx_rate);
        }
        if keyboard_input.pressed(KeyCode::Down) {
            
            tank.velocity *= 0.9;
        } else if keyboard_input.pressed(KeyCode::Up) {
            tank.velocity = (tank.velocity * 1.2).min(5.0).max(0.01);
            
        }

        if tank.initialized {
            tank.orientation += tank.rotation;

            let  matrx = Mat4::from_quat(
                Quat::from_rotation_z( tank.orientation * TO_RAD ));

            let mattx = Mat4::from_translation( Vec3::unit_y()* tank.velocity * delta_seconds * 100.0);
            let mat = matrx.mul_mat4(&mattx);
            let tx = Transform::from_matrix(mat);
            transform.translation += tx.translation;
            transform.rotation = if tank.velocity < 0.00001  { transform.rotation } else {tx.rotation};
        } else {
            let  matrx = Mat4::from_quat(
                Quat::from_rotation_z(tank.orientation * TO_RAD + tank.rotation * TO_RAD));
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

    let far = 4000.0;
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
            max_rx_rate: 10.0, 
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
    mouse_button_event_reader: EventReader<MouseButtonInput>,
    mouse_motion_event_reader: EventReader<MouseMotion>,
    cursor_moved_event_reader: EventReader<CursorMoved>,
    mouse_wheel_event_reader: EventReader<MouseWheel>,
}

/// This system prints out all mouse events as they come in
fn print_mouse_events_system(
    mut state: Local<State>,
    mouse_button_input_events: Res<Events<MouseButtonInput>>,
    mouse_motion_events: Res<Events<MouseMotion>>,
    cursor_moved_events: Res<Events<CursorMoved>>,
    mouse_wheel_events: Res<Events<MouseWheel>>,
) {
    for event in state
        .mouse_button_event_reader
        .iter(&mouse_button_input_events)
    {
        println!("{:?}", event);
    }

    for event in state.mouse_motion_event_reader.iter(&mouse_motion_events) {
        println!("{:?}", event);
    }

    for event in state.cursor_moved_event_reader.iter(&cursor_moved_events) {
        println!("{:?}", event);
    }

    for event in state.mouse_wheel_event_reader.iter(&mouse_wheel_events) {
        println!("{:?}", event);
    }
}
