use super::tawla_logic;
use crate::tawla_logic::{Checker, Point};
use bevy::prelude::*;
use bevy::render::camera::visible_entities_system;
use bevy_mod_picking;
use bevy_mod_picking::{Group, HighlightablePickMesh, PickSource, PickState, PickableMesh};
use bevy_orbit_controls::OrbitCamera;

#[derive(Debug)]
pub struct TawlaAssets {
    checker_mesh: Handle<Mesh>,
    board_paint: Handle<Texture>,
    board_brown: Handle<StandardMaterial>,
    checker_black: Color,
    checker_white: Color,
}
impl FromResources for TawlaAssets {
    fn from_resources(resources: &Resources) -> Self {
        let asset_loader = resources.get::<AssetServer>().unwrap();
        let checker_mesh: Handle<Mesh> = asset_loader.load("mesh/checker.stl");
        let board_paint: Handle<Texture> = asset_loader.load("art/smallboard.png");
        let mut materials = resources.get_mut::<Assets<StandardMaterial>>().unwrap();
        println!("assets loaded",);
        TawlaAssets {
            checker_mesh,
            board_paint,
            board_brown: materials.add(StandardMaterial::from(Color::rgb_u8(92, 64, 51))),
            checker_black: Color::rgb(0.2, 0.2, 0.2),
            checker_white: Color::rgb(0.8, 0.8, 0.8),
        }
    }
}
#[derive(Default)]
pub struct SelectedEntity {
    entity: Option<Entity>,
}
pub fn select_entity(
    pick_state: Res<PickState>,
    mouse_click: Res<Input<MouseButton>>,
    mut selected_entity: ResMut<SelectedEntity>,
) {
    if mouse_click.just_pressed(MouseButton::Left) == false {
        return;
    }
    selected_entity.entity = match pick_state.top(Group::default()) {
        Some((entity, _intersection)) => Some(*entity),
        None => None,
    };
}

pub fn color_point(
    pick_state: Res<PickState>,
    selected_point: Res<SelectedEntity>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    points: Query<(Entity, &Point, &Handle<StandardMaterial>)>,
) {
    let hovered_over = match pick_state.top(Group::default()) {
        Some((entity, _intersection)) => Some(*entity),
        None => None,
    };
    for (entity, _, material_handle) in points.iter() {
        let material = materials.get_mut(material_handle).unwrap();
        if Some(entity) == selected_point.entity {
            material.albedo = Color::rgb_u8(0, 102, 255);
        } else if Some(entity) == hovered_over {
            material.albedo = Color::rgb_u8(51, 204, 255);
        } else {
            material.albedo = Color::rgb(0.4, 0.4, 0.4);
        }
    }
}
pub fn color_checker(
    selected_checker: Res<SelectedEntity>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    checkers: Query<(Entity, &Checker, &Handle<StandardMaterial>)>,
    tawla_assets: Res<TawlaAssets>,
) {
    for (entity, checker, material_handle) in checkers.iter() {
        let material = materials.get_mut(material_handle).unwrap();
        if Some(entity) == selected_checker.entity {
            material.albedo = Color::rgb_u8(51, 204, 51);
        } else {
            if checker.owner == tawla_logic::CheckerColor::Black {
                material.albedo = tawla_assets.checker_black;
            } else {
                material.albedo = tawla_assets.checker_white;
            }
        }
    }
}

pub fn build_board(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    tawla_assets: Res<TawlaAssets>,
) {
    let board_paint: Handle<StandardMaterial> = materials.add(StandardMaterial {
        albedo_texture: Some(tawla_assets.board_paint.clone()),
        shaded: false,
        ..Default::default()
    });
    /* Build Container Box*/
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Box::new(27., 31., 0.4))),
        material: tawla_assets.board_brown.clone(),
        ..Default::default()
    });

    for i in (-1..=1).step_by(2) {
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(0.5, 31., 1.5))),
            material: tawla_assets.board_brown.clone(),
            transform: Transform::from_translation(Vec3::new(i as f32 * 13.25, 0., 0.4)),
            ..Default::default()
        });
    }

    for i in (-1..=1).step_by(2) {
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(27., 0.5, 1.5))),
            material: tawla_assets.board_brown.clone(),
            transform: Transform::from_translation(Vec3::new(0., i as f32 * 15.25, 0.4)),
            ..Default::default()
        });
    }
    // /*End Box*/
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
                        material: materials.add(StandardMaterial::from(Color::rgb(0.4, 0.4, 0.4))),
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
                    .with(tawla_logic::Point::new(position))
                    .with(PickableMesh::default());
            }
        }
    }
    /* Points Spawned*/
    /*Spawn Checkers*/

    for &height in [0.4, 0.75, 1.15].iter() {
        for y_location in 0..=4 {
            commands
                .spawn(PbrBundle {
                    mesh: tawla_assets.checker_mesh.clone(),
                    material: materials.add(StandardMaterial::from(tawla_assets.checker_black)),
                    transform: Transform::from_translation(Vec3::new(
                        -12.0,
                        (-14 + 2 * y_location) as f32,
                        height,
                    )),
                    ..Default::default()
                })
                .with(tawla_logic::Checker {
                    position: 1,
                    owner: tawla_logic::CheckerColor::Black,
                })
                .with(
                    PickableMesh::default().with_bounding_sphere(tawla_assets.checker_mesh.clone()),
                );
        }
    }
    for &height in [0.4, 0.75, 1.15].iter() {
        for y_location in 0..=4 {
            commands
                .spawn(PbrBundle {
                    mesh: tawla_assets.checker_mesh.clone(),
                    material: materials.add(StandardMaterial::from(tawla_assets.checker_white)),
                    transform: Transform::from_translation(Vec3::new(
                        -12.0,
                        -(-14 + 2 * y_location) as f32,
                        height,
                    )),
                    ..Default::default()
                })
                .with(tawla_logic::Checker {
                    position: 24,
                    owner: tawla_logic::CheckerColor::White,
                })
                .with(PickableMesh::default());
        }
    }
    /*Checkers Spawned*/
    /* Camera and Light*/
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(00.0, 00.0, 50.0))
                .looking_at(Vec3::default(), Vec3::unit_y()),
            ..Default::default()
        })
        .with(PickSource::default())
        //     //.with(OrbitCamera::default())
        //     commands.spawn(Camera2dBundle{
        //         transform: Transform::from_translation(Vec3::new(0.0,0.0,10.)).looking_at(Vec3::default(), Vec3::unit_y()),
        //         ..Default::default()
        //     })
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(00.0, 100.0, 100.0))
                .looking_at(Vec3::default(), Vec3::unit_y()),
            ..Default::default()
        });
    /*End Camera and Light*/
}
