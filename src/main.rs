#![allow(clippy::redundant_field_names)]
use bevy::{prelude::*, render::camera::CameraProjection};

#[derive(Component)]
pub struct Player {
    speed: f32,
}
pub struct PlaceHolderGraphics {
    texture_atlas: Handle<TextureAtlas>,
    player_index: usize,
}
fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.3, 0.5, 0.3)))
        .insert_resource(WindowDescriptor {
            width: 1600.0,
            height: 900.0,
            title: "Survival Bevy".to_string(),
            vsync: true,
            resizable: false,
            ..Default::default()
        })
        .add_startup_system_to_stage(StartupStage::PreStartup, load_graphics)
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_camera)
        .add_system(player_movement)
        .add_plugins(DefaultPlugins)
        .run();
}

fn spawn_player(mut commands: Commands, graphics: Res<PlaceHolderGraphics>) {
    let mut sprite = TextureAtlasSprite::new(graphics.player_index);
    sprite.custom_size = Some(Vec2::splat(100.0));
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: sprite,
            texture_atlas: graphics.texture_atlas.clone(),
            ..Default::default()
        })
        .insert(Player { speed: 100.0 });
}

fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
)

fn player_movement(keyboard: Res<Input<KeyCode>>, time: Res<Time>, mut player_query: Query<(&mut Transform, &Player)>) {
    let (mut player_transform, player) = player_query.single_mut();
    if keyboard.pressed(KeyCode::A) {
        player_transform.translation.x -= player.speed * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::W) {
        player_transform.translation.y += player.speed * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::S) {
        player_transform.translation.y -= player.speed * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::D) { 
        player_transform.translation.x += player.speed * time.delta_seconds();
    }
}

fn load_graphics(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_assets: ResMut<Assets<TextureAtlas>>,
) {
    let image_handle = assets.load("placeholder.png");
    let mut atlas = TextureAtlas::new_empty(image_handle, Vec2::splat(256.0));
    let player_index = atlas.add_texture(bevy::sprite::Rect {
        min: Vec2::splat(0.0),
        max: Vec2::splat(32.0),
    });

    let atlas_handle = texture_assets.add(atlas);

    commands.insert_resource(PlaceHolderGraphics {
        texture_atlas: atlas_handle,
        player_index: player_index,
    })
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.left = -1.0 * 16.0 / 9.0;
    camera.orthographic_projection.right = 1.0 * 16.0 / 9.0;
    camera.orthographic_projection.top = 1.0;
    camera.orthographic_projection.bottom = -1.0;

    commands.spawn_bundle(camera);
}
