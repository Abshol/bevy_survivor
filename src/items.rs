use std::time::Duration;

use bevy::{
    ecs::system::EntityCommands,
    prelude::{self, Commands, *},
};
use bevy_inspector_egui::Inspectable;
use rand::{thread_rng, Rng};

use crate::{graphics::PlaceHolderGraphics, player::Player};

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

#[derive(Default, Inspectable, Clone)]
pub struct ItemData {
    pub types: ItemType,
    pub name: String,
    pub graphics: usize,
    pub pickupable: bool,

    pub max_num: u32,
    pub current_num: u32,
}

#[derive(Inspectable)]
pub struct Items {
    pub flint: ItemData,
}

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(get_item.label("getitems"))
            .add_system(natural_spawn_flint.after("getitems"));
    }
}

fn get_item(mut commands: Commands, graphics: Res<PlaceHolderGraphics>, mut time: Res<Time>) {
    let flint = ItemData {
        types: ItemType::Flint,
        name: "Flint".to_string(),
        graphics: graphics.flint_index,
        pickupable: true,

        current_num: 0,
        max_num: 100,
    };
    commands.insert_resource(Items { flint: flint });
    let mut timer = Timer::new(Duration::from_secs(1), true);
    timer.tick(time.delta());
}

pub fn spawn_item(
    item: &ItemData,
    position: Vec2,
    sprite: TextureAtlasSprite,
    commands: &mut Commands,
    graphics: &Res<PlaceHolderGraphics>,
) -> Entity {
    let mut sprite = commands.spawn_bundle(SpriteSheetBundle {
        sprite: sprite,
        texture_atlas: graphics.texture_atlas.clone(),
        transform: Transform {
            translation: position.extend(0.0),
            ..Default::default()
        },
        ..Default::default()
    });

    match item.pickupable {
        true => sprite.insert(Pickupable { item: item.types }),
        false => sprite.insert(Object { item: item.types }),
    };

    sprite.insert(Name::new(item.name.clone())).id()
}

pub fn dropped_item(
    mut commands: Commands,
    graphics: Res<PlaceHolderGraphics>,
    item: &ItemData,
    position: Vec2,
    sprite: TextureAtlasSprite,
) -> Entity {
    let mut sprite = commands.spawn_bundle(SpriteSheetBundle {
        sprite: sprite,
        texture_atlas: graphics.texture_atlas.clone(),
        transform: Transform {
            translation: position.extend(0.0),
            ..Default::default()
        },
        ..Default::default()
    });
    sprite.insert(Pickupable { item: item.types }).insert(Name::new(item.name.clone())).id()
}

fn natural_spawn_flint(
    mut commands: Commands,
    graphics: Res<PlaceHolderGraphics>,
    mut items: ResMut<Items>,
    player_query: Query<&Transform, With<Player>>,
    mut time: Res<Time>,
) {
    if items.flint.current_num < items.flint.max_num {
        let mut rng = thread_rng();
        let spawn = rng.gen_range(0..=100);
        println!("{}", spawn);
        if spawn <= 5{
            let player_transform = player_query.single().translation;
            let x_add: f32 = rng.gen_range(-467.0..=467.0);
            let y_add: f32 = rng.gen_range(-380.0..=380.0);
            let position = Vec2::new(player_transform.x + x_add, player_transform.y + y_add);

            let mut sprite = TextureAtlasSprite::new(
                *graphics
                    .item_map
                    .get(&items.flint.types)
                    .expect("No graphic for item"),
            );
            sprite.custom_size = Some(Vec2::splat(25.0));
            spawn_item(&items.flint, position, sprite, &mut commands, &graphics);
            items.flint.current_num += 1;
        }
    }
}

fn spawner() {
    natural_spawn_flint;
}
