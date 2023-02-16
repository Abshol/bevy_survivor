#![allow(clippy::redundant_field_names)]
#![allow(clippy::type_complexity)]
use bevy::{prelude::*};
use bevy_inspector_egui::RegisterInspectable;

pub const HEIGHT: f32 = 900.0;
pub const RESOLUTION: f32 = 16.0 / 9.0;

#[derive(Component)]
pub struct GameCamera;

mod graphics;
mod inventory;
mod player;
mod items;

use bevy_inspector_egui::WorldInspectorPlugin;
use graphics::{PlaceHolderGraphics};
use inventory::{Inventory, Pickupable};
use items::ItemData;
use items::ItemType;
use items::Items;
use player::{Player};

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
        .add_startup_system_to_stage(StartupStage::PreStartup, spawn_camera)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(inventory::InventoryPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(graphics::GraphicsPlugin)
        .add_plugin(items::ItemPlugin)
        .register_inspectable::<Inventory>()
        .register_inspectable::<Player>()
        .register_inspectable::<Pickupable>()
        .run();
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
