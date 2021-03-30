use tawlalib::board;
use bevy::prelude::*;



fn main() {
    App::build()
        .add_resource(Msaa{ samples: 4 })
        .add_resource(WindowDescriptor{
            width: 1000.0,
            height: 1000.0,
            title: "Tawla".to_string(),
            vsync: false,
            resizable: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_stl::StlPlugin)
        .add_plugin(bevy_mod_picking::PickingPlugin)
        .add_plugin(bevy_mod_picking::DebugPickingPlugin)
        .init_resource::<tawlalib::TawlaAssets>()
        .add_startup_system(tawlalib::build_board.system())
        .run();
}