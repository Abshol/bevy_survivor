#![allow(clippy::redundant_field_names)]
#![allow(clippy::type_complexity)]
use bevy::{prelude::*, render::camera::CameraProjection};
use bevy_inspector_egui::{InspectorPlugin, WorldInspectorPlugin, Inspectable, RegisterInspectable};

#[derive(Component, Inspectable)]
pub struct Player {
    speed: f32,
    arm_length: f32,
}

#[derive(Component, Default, Inspectable)]
pub struct Inventory {
    items: [InventoryEntry; 5],
}

#[derive(Default, Inspectable)]
pub struct InventoryEntry {
    item: ItemType,
    count: usize,
}

#[derive(Component, Inspectable)]
pub struct Pickupable {
    item: ItemType,
}

#[derive(Default, Inspectable, PartialEq, Eq, Clone, Copy)]
pub enum ItemType {
    #[default]
    None,
    Flint,
    Axe,
    Twig,
    Grass,
    Wood,
    PineCone,
}

pub struct PlaceHolderGraphics {
    texture_atlas: Handle<TextureAtlas>,
    player_index: usize,
    flint_index: usize,
}
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
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
        .add_startup_system(spawn_flint)
        .add_startup_system(spawn_camera)
        .add_system(player_movement)
        .add_system(camera_follow)
        .add_system(player_pickup)
        .add_plugin(WorldInspectorPlugin::new())
        .register_inspectable::<Inventory>()
        .register_inspectable::<Player>()
        .register_inspectable::<Pickupable>()
        .run();
}

fn player_pickup(
    mut commands: Commands,
    keyboard: Res<Input<KeyCode>>,
    mut player_query: Query<(&Transform, &Player, &mut Inventory)>,
    pickupable_query: Query<(Entity, &Transform, &Pickupable), Without<Player>>,
) {
    //TODO Walk towards item when picking it up
    let (player_transform, player, mut inventory) = player_query.single_mut();
    if keyboard.just_pressed(KeyCode::E) {
        //TODO Pickup the nearest item not first
        for (ent, transform, pickup) in pickupable_query.iter() {
            if player.arm_length
                > Vec3::distance(transform.translation, player_transform.translation)
            {
                //Add to item count if item is already in inventory
                for mut slot in inventory.items.iter_mut() {
                    if slot.item == pickup.item {
                        slot.count += 1;
                        commands.entity(ent).despawn_recursive();
                        return;
                    }
                }
                //Add item to inventory if you don't have it
                for mut slot in inventory.items.iter_mut() {
                    if slot.item == ItemType::None {
                        slot.item = pickup.item;
                        slot.count = 1;
                        commands.entity(ent).despawn_recursive();
                        return;
                    }
                }
            }
        }
    }
}

fn spawn_flint(mut commands: Commands, graphics: Res<PlaceHolderGraphics>) {
    let mut sprite = TextureAtlasSprite::new(graphics.flint_index);
    sprite.custom_size = Some(Vec2::splat(25.0));
    commands
    .spawn_bundle(SpriteSheetBundle {
        sprite: sprite.clone(),
        texture_atlas: graphics.texture_atlas.clone(),
        ..Default::default()
    })
    .insert(Pickupable {
        item: ItemType::Flint,
    })
    .insert(Name::new("Flint"));

    commands
    .spawn_bundle(SpriteSheetBundle {
        sprite: sprite.clone(),
        texture_atlas: graphics.texture_atlas.clone(),
        ..Default::default()
    })
    .insert(Pickupable {
        item: ItemType::Axe,
    })
    .insert(Name::new("Axe"));

    commands
    .spawn_bundle(SpriteSheetBundle {
        sprite: sprite.clone(),
        texture_atlas: graphics.texture_atlas.clone(),
        ..Default::default()
    })
    .insert(Pickupable {
        item: ItemType::Twig,
    })
    .insert(Name::new("Twig"));

    commands
    .spawn_bundle(SpriteSheetBundle {
        sprite: sprite.clone(),
        texture_atlas: graphics.texture_atlas.clone(),
        ..Default::default()
    })
    .insert(Pickupable {
        item: ItemType::Grass,
    })
    .insert(Name::new("Grass"));

    commands
    .spawn_bundle(SpriteSheetBundle {
        sprite: sprite.clone(),
        texture_atlas: graphics.texture_atlas.clone(),
        ..Default::default()
    })
    .insert(Pickupable {
        item: ItemType::Wood,
    })
    .insert(Name::new("Wood"));

    commands
    .spawn_bundle(SpriteSheetBundle {
        sprite: sprite.clone(),
        texture_atlas: graphics.texture_atlas.clone(),
        ..Default::default()
    })
    .insert(Pickupable {
        item: ItemType::PineCone,
    })
    .insert(Name::new("PineCone"));
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
        .insert(Player {
            speed: 100.0,
            arm_length: 50.0,
        })
        .insert(Inventory::default())
        .insert(Name::new("Player"));
}

fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    let player_transform = player_query.single().translation;
    let mut camera_transform = camera_query.single_mut();

    camera_transform.translation.x = player_transform.x + 1.0;
    camera_transform.translation.y = player_transform.y + 1.0;
}

fn player_movement(
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &Player)>,
) {
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

    let flint_index = atlas.add_texture(bevy::sprite::Rect {
        min: Vec2::new(32.0, 0.0),
        max: Vec2::new(48.0, 16.0),
    });

    let atlas_handle = texture_assets.add(atlas);

    commands.insert_resource(PlaceHolderGraphics {
        texture_atlas: atlas_handle,
        player_index: player_index,
        flint_index: flint_index,
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
