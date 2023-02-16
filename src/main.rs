#![allow(clippy::redundant_field_names)]
#![allow(clippy::type_complexity)]
use bevy::{prelude::*, render::camera::CameraProjection, utils::HashMap};
use bevy_inspector_egui::{
    Inspectable, InspectorPlugin, RegisterInspectable, WorldInspectorPlugin,
};

pub const HEIGHT: f32 = 900.0;

pub const RESOLUTION: f32 = 16.0 / 9.0;

pub const INVENTORY_SIZE: usize = 10;

#[derive(Component)]
pub struct GameCamera;

#[derive(Component, Inspectable)]
pub struct Player {
    speed: f32,
    arm_length: f32,
}

#[derive(Component, Default, Inspectable)]
pub struct Inventory {
    items: [InventoryEntry; INVENTORY_SIZE],
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

#[derive(Component)]
pub struct UiCountText {
    slot: usize,
}

#[derive(Component)]
pub struct UiBox {
    slot: usize,
}

#[derive(Component)]
pub struct UiBoxContents;

#[derive(Default, Inspectable, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ItemType {
    #[default]
    None,
    Flint,
    Axe,
    Twig,
    Grass,
    Wood,
    PineCone,
    Fire,
    ChoppedPineCone,

    Default,
}

pub struct PlaceHolderGraphics {
    texture_atlas: Handle<TextureAtlas>,
    player_index: usize,
    box_index: usize,
    item_map: HashMap<ItemType, usize>,
    axe_index: usize,
    pinecone_index: usize,
    twig_index: usize,
    wood_index: usize,
    fire_index: usize,
    chopped_pinecone_index: usize,

    default_index: usize,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.3, 0.5, 0.3)))
        .insert_resource(WindowDescriptor {
            width: HEIGHT * RESOLUTION,
            height: HEIGHT,
            title: "Survival Bevy".to_string(),
            vsync: true,
            resizable: false,
            ..Default::default()
        })
        .add_startup_system_to_stage(StartupStage::PreStartup, load_graphics)
        .add_startup_system_to_stage(StartupStage::PreStartup, spawn_camera)
        .add_startup_system(spawn_player.label("player"))
        .add_startup_system(spawn_flint.before("player"))
        .add_startup_system(spawn_inventory_ui)
        .add_system(player_movement)
        .add_system(camera_follow)
        .add_system(player_pickup)
        .add_system(update_inventory_ui)
        .add_system(drop_item_test)
        .add_plugin(WorldInspectorPlugin::new())
        .register_inspectable::<Inventory>()
        .register_inspectable::<Player>()
        .register_inspectable::<Pickupable>()
        .run();
}

fn drop_item_test(keyboard: Res<Input<KeyCode>>, mut inventory_query: Query<&mut Inventory>) {
    let mut inventory = inventory_query.single_mut();
    if keyboard.just_pressed(KeyCode::Q) {
        inventory.items[0].count = 0;
        inventory.items[0].item = ItemType::None;
    }
}

fn update_inventory_ui(
    mut commands: Commands,
    inventory_query: Query<&Inventory>,
    graphics: Res<PlaceHolderGraphics>,
    box_query: Query<(Entity, Option<&Children>, &UiBox)>,
    mut box_contents_query: Query<&mut TextureAtlasSprite, With<UiBoxContents>>,
    mut text_query: Query<(&UiCountText, &mut Text)>,
) {
    let inventory = inventory_query.single();
    for (i, slot) in inventory.items.iter().enumerate() {
        for (text_count, mut text) in text_query.iter_mut() {
            if text_count.slot == i {
                if slot.count > 0 {
                    text.sections[0].value = format!("{}", slot.count);
                } else {
                    text.sections[0].value = format!("");
                }
            }
        }
        for (box_ent, children, ui_box) in box_query.iter() {
            if ui_box.slot == i {
                //Change graphic if there is a graphic
                if slot.count != 0 {
                    match children {
                        Some(children) => {
                            for child in children.iter() {
                                let mut sprite = box_contents_query
                                    .get_mut(*child)
                                    .expect("Nonsprite child of box");

                                sprite.index = *graphics
                                    .item_map
                                    .get(&slot.item)
                                    .expect("Error: No graphics for item");
                            }
                        }
                        None => {
                            let mut sprite = TextureAtlasSprite::new(
                                *graphics
                                    .item_map
                                    .get(&slot.item)
                                    .expect("Error: No graphics for item"),
                            );
                            sprite.custom_size = Some(Vec2::splat(25.0));
                            let graphic = commands
                                .spawn_bundle(SpriteSheetBundle {
                                    sprite: sprite,
                                    texture_atlas: graphics.texture_atlas.clone(),
                                    ..Default::default()
                                })
                                .insert(Name::new("ItemGraphic"))
                                .insert(UiBoxContents)
                                .id();
                            commands.entity(box_ent).add_child(graphic);
                        }
                    }
                } else if let Some(children) = children {
                        //Slot empty, we despawn the children
                        for child in children.iter() {
                            if box_contents_query.get(*child).is_ok() {
                                commands.entity(*child).despawn_recursive();
                            }
                        }
                    }
                }
            }
        }
    }

fn spawn_inventory_ui(
    mut commands: Commands,
    graphics: Res<PlaceHolderGraphics>,
    camera_query: Query<Entity, With<GameCamera>>,
    asset_server: Res<AssetServer>,
) {
    let camera_ent = camera_query.single();

    let mut boxes = Vec::new();
    let mut ui_texts = Vec::new();

    let spacing = 84.0;
    let spacing_percent = spacing / 3.6 / RESOLUTION / 2.0;

    let starting_x = (-(INVENTORY_SIZE as f32) / 2.0 + 0.5) * spacing;
    let starting_percent = 125.5 + starting_x / RESOLUTION / 2.0;

    let mut sprite = TextureAtlasSprite::new(graphics.box_index);
    sprite.custom_size = Some(Vec2::splat(50.0));
    for i in 0..INVENTORY_SIZE {
        ui_texts.push(
            commands
                .spawn_bundle(TextBundle {
                    style: Style {
                        align_self: AlignSelf::FlexEnd,
                        position_type: PositionType::Absolute,
                        position: Rect {
                            bottom: Val::Percent(11.0),
                            left: Val::Percent(starting_percent + spacing_percent * i as f32),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    text: Text::with_section(
                        "",
                        TextStyle {
                            font: asset_server.load("fonts/QuattrocentoSans-Regular.ttf"),
                            font_size: 25.0,
                            color: Color::BLACK,
                        },
                        TextAlignment {
                            horizontal: HorizontalAlign::Center,
                            ..Default::default()
                        },
                    ),
                    ..Default::default()
                })
                .insert(UiCountText { slot: i })
                .insert(Name::new("Inventory Count"))
                .id(),
        );

        boxes.push(
            commands
                .spawn_bundle(SpriteSheetBundle {
                    sprite: sprite.clone(),
                    texture_atlas: graphics.texture_atlas.clone(),
                    transform: Transform {
                        translation: Vec3::new(starting_x + spacing * i as f32, -260.0, -1.0),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(UiBox { slot: i })
                .id(),
        );
    }
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .push_children(&ui_texts)
        .insert(Name::new("Inventory Text"));
    commands.entity(camera_ent).push_children(&boxes);
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
    let mut sprite = TextureAtlasSprite::new(
        *graphics
            .item_map
            .get(&ItemType::Flint)
            .expect("No graphic for flint"),
    );
    sprite.custom_size = Some(Vec2::splat(25.0));
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: sprite,
            texture_atlas: graphics.texture_atlas.clone(),
            ..Default::default()
        })
        .insert(Pickupable {
            item: ItemType::Flint,
        })
        .insert(Name::new("Flint"));
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
    mut camera_query: Query<&mut Transform, (With<GameCamera>, Without<Player>)>,
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

    let axe_index = atlas.add_texture(bevy::sprite::Rect {
        min: Vec2::new(32.0, 18.0),
        max: Vec2::new(48.0, 32.0),
    });

    let box_index = atlas.add_texture(bevy::sprite::Rect {
        min: Vec2::new(0.0, 32.0),
        max: Vec2::new(32.0, 64.0),
    });

    let grass_index = atlas.add_texture(bevy::sprite::Rect {
        min: Vec2::new(50.0, 0.0),
        max: Vec2::new(64.0, 16.0),
    });

    let pinecone_index = atlas.add_texture(bevy::sprite::Rect {
        min: Vec2::new(0.0, 75.0),
        max: Vec2::new(32.0, 112.0),
    });

    let twig_index = atlas.add_texture(bevy::sprite::Rect {
        min: Vec2::new(49.0, 18.0),
        max: Vec2::new(65.0, 32.0),
    });

    let wood_index = atlas.add_texture(bevy::sprite::Rect {
        min: Vec2::new(34.0, 34.0),
        max: Vec2::new(50.0, 51.0),
    });

    let fire_index = atlas.add_texture(bevy::sprite::Rect {
        min: Vec2::new(32.0, 50.0),
        max: Vec2::new(64.0, 106.0),
    });

    let chopped_pinecone_index = atlas.add_texture(bevy::sprite::Rect {
        min: Vec2::new(0.0, 128.0),
        max: Vec2::new(32.0, 128.0 + 16.0),
    });

    let default_index = atlas.add_texture(bevy::sprite::Rect {
        min: Vec2::new(240.0, 240.0),
        max: Vec2::new(256.0, 256.0),
    });

    let mut item_map = HashMap::default();
    item_map.insert(ItemType::Flint, flint_index);
    item_map.insert(ItemType::Axe, axe_index);
    item_map.insert(ItemType::Grass, grass_index);
    item_map.insert(ItemType::PineCone, pinecone_index);
    item_map.insert(ItemType::Twig, twig_index);
    item_map.insert(ItemType::Wood, wood_index);
    item_map.insert(ItemType::Fire, fire_index);
    item_map.insert(ItemType::ChoppedPineCone, chopped_pinecone_index);
    item_map.insert(ItemType::Default, default_index);

    let atlas_handle = texture_assets.add(atlas);

    commands.insert_resource(PlaceHolderGraphics {
        texture_atlas: atlas_handle,
        player_index: player_index,
        box_index: box_index,
        item_map: item_map,
        axe_index: axe_index,
        pinecone_index: pinecone_index,
        twig_index: twig_index,
        wood_index: wood_index,
        fire_index: fire_index,
        chopped_pinecone_index: chopped_pinecone_index,

        default_index: default_index,
    })
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
    let mut camera = OrthographicCameraBundle::new_2d();

    camera.orthographic_projection.left = -1.0 * RESOLUTION;
    camera.orthographic_projection.right = 1.0 * RESOLUTION;
    camera.orthographic_projection.top = 1.0;
    camera.orthographic_projection.bottom = -1.0;

    commands.spawn_bundle(camera).insert(GameCamera);
}
