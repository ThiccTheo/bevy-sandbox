mod player;

use bevy::prelude::*;
use player::{spawn_player, update_input};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_player)
        .add_system(update_input)
        .run();
}
