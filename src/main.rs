use bevy::prelude::*;
use player::PlayerPlugin;

mod player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_startup_system(spawn_camera)
        .run();
}

fn spawn_camera(mut cmds: Commands) {
    let cam = Camera2dBundle::default();
    cmds.spawn(cam);
}
