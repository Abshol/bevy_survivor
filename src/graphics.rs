use bevy::{prelude::*, utils::HashMap};

use crate::inventory::ItemType;

pub struct PlaceHolderGraphics {
    pub texture_atlas: Handle<TextureAtlas>,
    pub player_index: usize,
    pub box_index: usize,
    pub item_map: HashMap<ItemType, usize>,
    pub axe_index: usize,
    pub pinecone_index: usize,
    pub twig_index: usize,
    pub wood_index: usize,
    pub fire_index: usize,
    pub chopped_pinecone_index: usize,

    pub default_index: usize,
}

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_graphics);
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
