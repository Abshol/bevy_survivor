use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

pub const INVENTORY_SIZE: usize = 10;

use crate::{
    graphics::PlaceHolderGraphics, player::Player, GameCamera, RESOLUTION,
};

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
    pub(crate) item: ItemType,
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

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_inventory_ui)
            .add_system(player_pickup)
            .add_system(update_inventory_ui)
            .add_system(drop_item_test);
    }
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
                    text.sections[0].value = String::new();
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
