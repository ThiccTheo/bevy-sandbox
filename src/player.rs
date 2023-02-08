use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Resource)]
struct PlayerTexture(Handle<Image>);
pub struct PlayerPlugin;

impl PlayerPlugin {
    fn load_texture(mut cmds: Commands, asset_server: Res<AssetServer>) {
        let img = asset_server.load("images/player.png");
        cmds.insert_resource(PlayerTexture(img));
    }

    fn spawn(mut cmds: Commands, tex: Res<PlayerTexture>) {
        cmds.spawn((
            Player,
            SpriteBundle {
                texture: tex.0.clone(),
                ..default()
            },
        ));
    }

    fn input(mut qry: Query<&mut Transform, With<Player>>, time: Res<Time>, key_code: Res<Input<KeyCode>>) {
        let mut tf = qry.single_mut();
        let dt = time.delta_seconds();
        let offset = 100. * dt;

        if key_code.pressed(KeyCode::W) {
            tf.translation.y += offset;
        }
        if key_code.pressed(KeyCode::A) {
            tf.translation.x -= offset;
        }
        if key_code.pressed(KeyCode::S) {
            tf.translation.y -= offset;
        }
        if key_code.pressed(KeyCode::D) {
            tf.translation.x += offset;
        }
    }
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, Self::load_texture)
            .add_startup_system(Self::spawn)
            .add_system(Self::input);
    }
}
