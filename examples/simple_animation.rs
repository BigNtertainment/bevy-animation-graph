use std::time::Duration;

use bevy::prelude::*;
use bevy_spritesheet_animation::{
    animation_graph::AnimationTransitionCondition,
    animation_manager::AnimationManager,
    spritesheet_animation::{
        AnimationBounds, SpritesheetAnimation, SpritesheetAnimationCollection,
    },
    AnimationGraphPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(AnimationGraphPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle =
        asset_server.load("kenney_toon-characters-1/robot/Tilesheet/character_robot_sheet.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(96.0, 128.0), 9, 5, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let animation_bounds = AnimationBounds::new(0, 3);

    commands.spawn(Camera2dBundle::default());

    let animation_collection =
        SpritesheetAnimationCollection::new(vec![SpritesheetAnimation::new(
            animation_bounds,
            Duration::from_millis(500),
        )]);

    println!("{:?}", animation_collection.animations.len());

    let mut animation_manager = AnimationManager::new(animation_collection.animations.len(), 0);

    animation_manager.add_graph_edge(0, 0, AnimationTransitionCondition::new(Box::new(|_| true)));

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite::new(animation_bounds.first_frame_index),
            ..default()
        },
        animation_collection,
        animation_manager,
    ));
}
