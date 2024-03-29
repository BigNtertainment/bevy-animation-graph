//! This example shows how to use the animation graph to create a simple state machine for a spritesheet animation.
//! 
//! Press W to climb, Space to jump.

use std::time::Duration;

use bevy::prelude::*;
use bevy_animation_graph::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AnimationGraphPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (climbing, jumping))
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

    commands.spawn(Camera2dBundle::default());

    let animation_collection = SpritesheetAnimationCollection::new(vec![
        // Idle
        SpritesheetAnimation::new(AnimationBounds::new(0, 3), Duration::from_millis(500)),
        // Climbing
        SpritesheetAnimation::new(AnimationBounds::new(5, 6), Duration::from_millis(300)),
        // Jumping
        SpritesheetAnimation::new(AnimationBounds::new(7, 8), Duration::from_millis(500)),
    ]);

    let mut animation_manager = AnimationManager::new(animation_collection.animations.len(), 0);

    animation_manager.add_state("climbing".to_string(), false);
    animation_manager.add_state("jumping".to_string(), false);

    animation_manager.add_graph_edge(
        0,
        1,
        AnimationTransitionCondition::new(Box::new(|state| state["climbing"])),
    );
    animation_manager.add_graph_edge(
        1,
        1,
        AnimationTransitionCondition::new(Box::new(|state| state["climbing"])),
    );
    animation_manager.add_graph_edge(
        1,
        0,
        AnimationTransitionCondition::new(Box::new(|state| !state["climbing"])),
    );
    animation_manager.add_graph_edge(
        0,
        2,
        AnimationTransitionCondition::new(Box::new(|state| state["jumping"]))
            .with_mode(AnimationTransitionMode::Immediate),
    );
    animation_manager.add_graph_edge(
        1,
        2,
        AnimationTransitionCondition::new(Box::new(|state| state["jumping"]))
            .with_mode(AnimationTransitionMode::Immediate),
    );
    animation_manager.add_graph_edge(2, 0, AnimationTransitionCondition::new(Box::new(|_| true)));
    animation_manager.add_graph_edge(0, 0, AnimationTransitionCondition::new(Box::new(|_| true)));

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite::new(0),
            ..default()
        },
        animation_collection,
        animation_manager,
    ));
}

fn climbing(mut player_query: Query<&mut AnimationManager>, keyboard_input: Res<Input<KeyCode>>) {
    let mut animation_manager = player_query.single_mut();
    animation_manager
        .set_state("climbing".to_string(), keyboard_input.pressed(KeyCode::W))
        .unwrap();
}

fn jumping(mut player_query: Query<&mut AnimationManager>, keyboard_input: Res<Input<KeyCode>>) {
    let mut animation_manager = player_query.single_mut();
    animation_manager
        .set_state(
            "jumping".to_string(),
            keyboard_input.pressed(KeyCode::Space),
        )
        .unwrap();
}
