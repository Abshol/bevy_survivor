use bevy::{input::keyboard, prelude::*};

use crate::{inventory::Inventory, items::ItemType};

#[derive(Clone)]
pub struct CraftingRecipe {
    needed: Vec<ItemAndCount>,
    produces: ItemType,
}

#[derive(Clone, Copy)]
pub struct ItemAndCount {
    item: ItemType,
    count: usize,
}

pub struct CraftingBook {
    recipes: Vec<CraftingRecipe>,
}

pub struct CraftingPlugin;

impl Plugin for CraftingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CraftingBook {
            recipes: vec![CraftingRecipe {
                needed: vec![
                    ItemAndCount {
                        item: ItemType::Twig,
                        count: 1,
                    },
                    ItemAndCount {
                        item: ItemType::Flint,
                        count: 1,
                    },
                ],
                produces: ItemType::Axe,
            }],
        })
        .add_system(test_crafting);
    }
}

fn can_craft(inventory: &Inventory, recipe: &CraftingRecipe) -> bool {
    let mut found_item = false;
    for item_and_count in recipe.needed.iter() {
        for slot in inventory.items.iter() {
            if slot.item.types == item_and_count.item && slot.count >= item_and_count.count {
                found_item = true;
            }
            else {
                found_item = false;
            }
        }
    }
    if !found_item {
        return false;
    } else {
        return true;
    }
}

fn test_crafting(
    mut inventory_query: Query<&Inventory>,
    crafting_book: Res<CraftingBook>,
    keyboard: Res<Input<KeyCode>>,
) {
    let mut inventory = inventory_query.single_mut();
    if keyboard.just_pressed(KeyCode::F) && can_craft(inventory, &crafting_book.recipes[0]) {
        println!("Can craft axe!")
    }
}
