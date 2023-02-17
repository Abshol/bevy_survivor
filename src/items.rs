use bevy::{prelude::*, sprite, reflect::erased_serde::__private::serde::de::Expected};
use bevy_inspector_egui::Inspectable;

use crate::{graphics::{self, PlaceHolderGraphics}};

#[derive(Component, Inspectable)]
pub struct Pickupable {
    pub(crate) item: ItemType,
}

#[derive(Component, Inspectable)]
pub struct Object {
    pub(crate) item: ItemType,
}

pub struct ItemPlugin;

#[derive(Component, Default, Inspectable, PartialEq, Eq, Clone, Copy, Hash)]
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

#[derive(Default, Inspectable)]
pub struct ItemData {
    pub types: ItemType,
    pub name: String,
    pub graphics: usize,
    pub pickupable: bool,
}

pub struct Items {
    pub flint: ItemData,
    pub max_num: u32,
    pub current_num: u32,
}

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(get_item)
        .add_system(spawn_data);
    }
}

fn get_item(mut commands: Commands, graphics: Res<PlaceHolderGraphics>) {
    let flint = ItemData {
        types: ItemType::Flint,
        name: "Flint".to_string(),
        graphics: graphics.flint_index,
        pickupable: true,
    };
    commands.insert_resource(Items {
        flint: flint,

        current_num: 0,
        max_num: 100,
    });
}

pub fn spawn_item(mut commands: Commands, graphics: Res<PlaceHolderGraphics>, item: &ItemData, position:Vec2, sprite: TextureAtlasSprite) -> Entity {

    let mut sprite = commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: sprite,
            texture_atlas: graphics.texture_atlas.clone(),
            transform: Transform {
                translation: position.extend(0.0),
                ..Default::default()
            },
            ..Default::default()
        });

        match item.pickupable {
            true => sprite.insert(Pickupable {
                        item: item.types,
                    }),
            false => sprite.insert(Object {
                        item: item.types,
                    }),
        };

        sprite.insert(Name::new(item.name.clone())).id()
}

fn spawn_flint(commands: Commands, graphics: Res<PlaceHolderGraphics>, mut items: ResMut<Items>) {
    let mut sprite = TextureAtlasSprite::new(
        *graphics
            .item_map
            .get(&items.flint.types)
            .expect("No graphic for item")
    );
    sprite.custom_size = Some(Vec2::splat(25.0));
    spawn_item(commands, graphics, &items.flint, Vec2::new(0.3, 0.3), sprite);
    items.current_num += 1;
}

fn spawn_data(items: Res<Items>) {
    let mut i = 0;
    if i <= items.max_num {
        spawn_flint;
        i += 1;
    }
}