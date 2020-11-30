use bevy::prelude::*;
use tanky::components::*;
use tanky::systems::*;

use std::collections::HashMap;
type MaterialsMap = HashMap<&'static str, Handle<Texture>>;

const  TXPATH: &str = env!("CARGO_MANIFEST_DIR");

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .add_startup_system(setup.system())
        .add_system(tank_movement_system.system())
        .add_system(tank_mouse_events_system.system())
        ;
    }
}

// setup a tank
fn setup_tank1(asset_server: &Res<AssetServer>,  matmap: &mut MaterialsMap)  {
    let mut tank1_path = std::path::PathBuf::from(TXPATH);
    let mut turret1_path = tank1_path.clone();
    let mut gun1_path = tank1_path.clone();

    tank1_path.push("assets/PNG/Hulls_Color_C/Hull_01.png");
    gun1_path.push("assets/PNG/Weapon_Color_C/Gun_01_A.png");
    turret1_path.push("assets/PNG/Weapon_Color_C/Gun_01_B.png");

    matmap.insert("tank1_body", asset_server.load(tank1_path));
    matmap.insert("tank1_turret",asset_server.load(turret1_path));
    matmap.insert("tank1_gun",asset_server.load(gun1_path));
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    
    let mut matmap = MaterialsMap::new();
    setup_tank1(&asset_server, &mut matmap);

    commands
        .spawn(Camera2dComponents::default())
        .spawn(SpriteComponents {
            material: materials.add(
                matmap.remove("tank1_body").unwrap().into()
        ),
            transform: Transform::from_scale(Vec3::new(0.5,0.5,0.5)),
            ..Default::default()
        })
        .with(Velocity(0.0))
        .with(Orientation(45.0))
        .with(Rotation(0.0))
        .with(Tank {
            rx_rate: 0.002, 
            acc_rate: 0.1,
            //initialized: false
        })
        .with_children(|parent| {
            parent.spawn(SpriteComponents {
                material: materials.add(
                    matmap.remove("tank1_turret").unwrap().into()
                ),
                transform: Transform::from_translation(Vec3::new(0.0,-20.0,2.0)),
                ..Default::default()
            })
            .with_children(|par| {
                par.spawn(SpriteComponents{
                    material: materials.add(
                        matmap.remove("tank1_gun").unwrap().into()
                    ),
                    transform: Transform::from_translation(Vec3::new(0.0,100.0,1.0)),
                    ..Default::default()
                });
            });
        });
}




fn main() {
    App::build()
    .add_plugins(DefaultPlugins)
    .add_plugin(HelloPlugin)
    .run()
}
