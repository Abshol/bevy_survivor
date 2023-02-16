use bevy::{prelude::*, sprite};
use bevy_inspector_egui::Inspectable;

use crate::{graphics::{self, PlaceHolderGraphics}, inventory::Pickupable};


pub struct ItemPlugin;

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

pub struct ItemData {
    pub types: ItemType,
    pub name: String,
    pub graphics: usize,
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
    };
    commands.insert_resource(Items {
        flint: flint,

        current_num: 0,
        max_num: 100,
    });
}

fn spawn_items(mut commands: Commands, graphics: Res<PlaceHolderGraphics>, item: &ItemData, position:Vec2, sprite: TextureAtlasSprite) -> Entity {
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: sprite,
            texture_atlas: graphics.texture_atlas.clone(),
            transform: Transform {
                translation: position.extend(0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Pickupable {
            item: item.types,
        })
        .insert(Name::new(item.name.clone())).id()
}

fn spawn_flint(commands: Commands, graphics: Res<PlaceHolderGraphics>, mut items: ResMut<Items>) {
    let mut sprite = TextureAtlasSprite::new(
        *graphics
            .item_map
            .get(&items.flint.types)
            .expect("No graphic for item")
    );
    sprite.custom_size = Some(Vec2::splat(25.0));
    spawn_items(commands, graphics, &items.flint, Vec2::new(0.3, 0.3), sprite);
    items.current_num += 1;
}

fn spawn_data(items: Res<Items>) {
    while items.current_num <= items.max_num {
        spawn_flint;
    }
}