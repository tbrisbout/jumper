use bevy::{
    prelude::*,
    core_pipeline::clear_color::ClearColorConfig,
    ecs::system::{Commands, Res},
    time::Time,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()).build())
        .add_systems(Startup, setup)
        .add_systems(Update, character_movement)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::GRAY),
        },
        ..default()
    });

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(100.0, 100.0)),
                ..default()
            },
            texture: asset_server.load("beard.png"),
            ..default()
        }, Player { speed: 200.0 },
    ));
}

fn character_movement(
    mut characters: Query<(&mut Transform, &Player)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, player) in &mut characters {
        let distance = player.speed * time.delta_seconds();

        //
        // Support arrows, WASD style, Vim HJKL style
        //

        if input.any_pressed([KeyCode::Up, KeyCode::W, KeyCode::K]) {
            transform.translation.y += distance;
        }

        if input.any_pressed([KeyCode::Down, KeyCode::S, KeyCode::J]) {
            transform.translation.y -= distance;
        }

        if input.any_pressed([KeyCode::Right, KeyCode::D, KeyCode::L]) {
            transform.translation.x += distance;
        }

        if input.any_pressed([KeyCode::Left, KeyCode::A, KeyCode::H]) {
            transform.translation.x -= distance;
        }

        if input.pressed(KeyCode::R) {
            // reset position
            transform.translation.x = 0.0;
            transform.translation.y = 0.0;
        }
    }
}

#[derive(Component)]
struct Player {
    pub speed: f32,
}
