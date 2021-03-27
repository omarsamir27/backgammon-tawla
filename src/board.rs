use super::tawla_logic;
use bevy::prelude::*;
use bevy_mod_picking;
use bevy_mod_picking::PickableMesh;

pub struct TawlaAssets {
    checker_mesh: Handle<Mesh>,
    board_paint: Handle<Texture>,
}

fn load_assets(commands: &mut Commands, asset_server: Res<AssetServer>) {
    let checker_mesh: Handle<Mesh> = asset_server.load("mesh/checker.stl");
    let board_paint: Handle<Texture> = asset_server.load("art/board.png");
    commands.insert_resource(TawlaAssets {
        checker_mesh,
        board_paint,
    });
}

fn build_board(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    tawla_assets: Res<TawlaAssets>,
) {
    let board_paint: Handle<StandardMaterial> = materials.add(StandardMaterial {
        albedo_texture: Some(tawla_assets.board_paint.clone()),
        shaded: false,
        ..Default::default()
    });
    let brown = materials.add(StandardMaterial::from(Color::rgb_u8(92, 64, 51)));
    let checker_mesh: Handle<Mesh> = tawla_assets.checker_mesh.clone();

    /* Build Container Box*/
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Box::new(27., 31., 0.4))),
        material: brown.clone(),
        ..Default::default()
    });
    for i in (-1..=1).step_by(2) {
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(0.5, 31., 0.4))),
            material: brown.clone(),
            transform: Transform::from_translation(Vec3::new(i as f32 * 13.25, 0., 0.4)),
            ..Default::default()
        });
    }

    for i in (-1..=1).step_by(2) {
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(27., 0.5, 0.4))),
            material: brown.clone(),
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
                pos += 1;
                commands
                    .spawn(PbrBundle {
                        mesh: checker_mesh.clone(),
                        visible: Visible {
                            is_transparent: true,
                            is_visible: false,
                        },
                        transform: Transform::from_translation(Vec3::new(
                            (k * (12 - 2 * i)) as f32,
                            (4 * j) as f32,
                            0.4,
                        )),
                        ..Default::default()
                    })
                    .with(PickableMesh::default())
                    .with(tawla_logic::Point::new(position))
            }
        }
    }
    /* Points Spawned*/
}

fn spawn_checker(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    tawla_assets: Res<TawlaAssets>,
    point_position : u8
) {

}
