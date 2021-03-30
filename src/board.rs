use super::tawla_logic;
use bevy::prelude::*;
use bevy_mod_picking;
use bevy_mod_picking::PickableMesh;

#[derive(Debug)]
pub struct TawlaAssets {
    checker_mesh: Handle<Mesh>,
    board_paint: Handle<Texture>,
    board_brown: Handle<StandardMaterial>,
    checker_black : Handle<StandardMaterial>,
    checker_white : Handle<StandardMaterial>
}
impl FromResources for TawlaAssets{
    fn from_resources(resources: &Resources) -> Self{
        let asset_loader = resources.get::<AssetServer>().unwrap();
        let checker_mesh: Handle<Mesh> = asset_loader.load("./mesh/checker.stl");
        let board_paint: Handle<Texture> = asset_loader.load("./art/board.png");
        let mut materials = resources.get_mut::<Assets<StandardMaterial>>().unwrap();
        println!("assets loaded",);
        TawlaAssets{
            checker_mesh,
            board_paint,
            board_brown : materials.add(StandardMaterial::from(Color::rgb_u8(92, 64, 51))),
            checker_black : materials.add(StandardMaterial::from(Color::rgb_u8(2, 2, 2))),
            checker_white : materials.add(StandardMaterial::from(Color::rgb_u8(253, 253,253 )))
        }
    }
    }

pub fn load_assets(commands: &mut Commands, asset_server: Res<AssetServer>, mut materials: ResMut<Assets<StandardMaterial>>) {
    let checker_mesh: Handle<Mesh> = asset_server.load("mesh/checker.stl");
    let board_paint: Handle<Texture> = asset_server.load("art/board.png");
    commands.insert_resource(TawlaAssets {
        checker_mesh,
        board_paint,
        board_brown : materials.add(StandardMaterial::from(Color::rgb_u8(92, 64, 51))),
        checker_black : materials.add(StandardMaterial::from(Color::rgb_u8(2, 2, 2))),
        checker_white : materials.add(StandardMaterial::from(Color::rgb_u8(253, 253,253 )))
    });
}

 pub fn build_board(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    tawla_assets: Res<TawlaAssets>
) {
     println!("here");
    let board_paint: Handle<StandardMaterial> = materials.add(StandardMaterial {
        albedo_texture: Some(tawla_assets.board_paint.clone()),
        shaded: false,
        ..Default::default()
    });
     println!("here too");
    println!("{:?}",tawla_assets);
     /* Build Container Box*/
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Box::new(27., 31., 0.4))),
        //material: tawla_assets.board_brown.clone(),
        material : materials.add(StandardMaterial::from(Color::rgb_u8(10,10,5))),
        ..Default::default()
    });
     println!("after box");

     for i in (-1..=1).step_by(2) {
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(0.5, 31., 0.4))),
            material: tawla_assets.board_brown.clone(),
            transform: Transform::from_translation(Vec3::new(i as f32 * 13.25, 0., 0.4)),
            ..Default::default()
        });
    }

    for i in (-1..=1).step_by(2) {
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(27., 0.5, 0.4))),
            material: tawla_assets.board_brown.clone(),
            transform: Transform::from_translation(Vec3::new(0., i as f32 * 15.25, 0.4)),
            ..Default::default()
        });
    }
    /*End Box*/
    /*Paint Board*/
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(26., 30.)))),
        material: board_paint.clone(),
        transform: Transform::from_translation(Vec3::new(0., 0., 0.21)),
        ..Default::default()
    });
    /*End Paint*/
    /* Spawn Invisible Checkers at Points Top Vertex*/
    let mut position: u8 = 0;
    for j in (-1..=1).step_by(2) {
        for k in (-1..=1).step_by(2) {
            for i in 0..=5 {
                position += 1;
                commands
                    .spawn(PbrBundle {
                        mesh: tawla_assets.checker_mesh.clone(),
                        visible: Visible {
                            is_transparent: true,
                            is_visible: true,
                        },
                        transform: Transform::from_translation(Vec3::new(
                            (k * (12 - 2 * i)) as f32,
                            (4 * j) as f32,
                            0.4,
                        )),
                        ..Default::default()
                    })
                    .with(PickableMesh::default())
                    .with(tawla_logic::Point::new(position));
            }
        }
    }
    /* Points Spawned*/
}

// fn spawn_checker(
//     commands: &mut Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
//     tawla_assets: Res<TawlaAssets>,
//     point_position : u8
// ) {
//
// }
