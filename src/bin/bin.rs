use tawlalib::{board, select_entity, color_point, color_checker};
use bevy::prelude::*;
use bevy_orbit_controls::OrbitCameraPlugin;
use bevy_mod_picking::*;


fn main() {
    App::build()
        .add_resource(Msaa{ samples: 4 })
        .add_resource(WindowDescriptor{
            width: 1000.0,
            height: 1000.0,
            title: "Tawla".to_string(),
            resizable: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(InteractablePickingPlugin)
        .add_plugin(PickingPlugin)
        .add_plugin(DebugPickingPlugin)
        .add_plugin(bevy_orbit_controls::OrbitCameraPlugin)
        .add_plugin(bevy_stl::StlPlugin)
        .init_resource::<tawlalib::TawlaAssets>()
        .add_startup_system(tawlalib::build_board.system())
        .init_resource::<board::SelectedEntity>()
        .add_system(select_entity.system())
        .add_system(color_point.system())
        .add_system(color_checker.system())
        .run();
}