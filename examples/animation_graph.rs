use std::time::Duration;

use bevy::prelude::*;
use bevy_spritesheet_animation::{
    animation::{Animation, AnimationBounds},
    animation_graph::{AnimationTransitionCondition, AnimationTransitionMode},
    animation_manager::AnimationManager,
    SpritesheetAnimationPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(SpritesheetAnimationPlugin)
        .add_startup_system(setup)
		.add_systems((climbing, jumping))
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

    let mut animation_manager = AnimationManager::new(
        vec![
            // Idle
            Animation::new(
                texture_atlas_handle.clone(),
                AnimationBounds::new(0, 3),
                Duration::from_millis(500),
            ),
            // Climbing
            Animation::new(
                texture_atlas_handle.clone(),
                AnimationBounds::new(5, 6),
                Duration::from_millis(300),
            ),
            // Jumping
            Animation::new(
                texture_atlas_handle.clone(),
                AnimationBounds::new(7, 8),
                Duration::from_millis(500),
            ),
        ],
        0,
    );

	animation_manager.add_state("climbing".to_string(), false);
	animation_manager.add_state("jumping".to_string(), false);

    animation_manager.add_graph_edge(
        0,
        1,
        AnimationTransitionCondition::new(Some("climbing".to_string())),
    );
    animation_manager.add_graph_edge(
        1,
        0,
        AnimationTransitionCondition::new(Some("climbing".to_string())).negative(),
    );
    animation_manager.add_graph_edge(
        0,
        2,
        AnimationTransitionCondition::new(Some("jumping".to_string())).with_mode(AnimationTransitionMode::Immediate),
    );
    animation_manager.add_graph_edge(
        1,
        2,
        AnimationTransitionCondition::new(Some("jumping".to_string())).with_mode(AnimationTransitionMode::Immediate),
    );
    animation_manager.add_graph_edge(
        2,
        0,
        AnimationTransitionCondition::new(None),
    );
    animation_manager.add_graph_edge(0, 0, AnimationTransitionCondition::new(None));

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite::new(0),
            ..default()
        },
        animation_manager,
    ));
}

fn climbing(mut player_query: Query<&mut AnimationManager>, keyboard_input: Res<Input<KeyCode>>) {
	let mut animation_manager = player_query.single_mut();
	animation_manager.set_state("climbing".to_string(), keyboard_input.pressed(KeyCode::W)).unwrap();
	println!("{}", animation_manager.get_state("climbing".to_string()).unwrap());
}

fn jumping(mut player_query: Query<&mut AnimationManager>, keyboard_input: Res<Input<KeyCode>>) {
	let mut animation_manager = player_query.single_mut();
	animation_manager.set_state("jumping".to_string(), keyboard_input.pressed(KeyCode::Space)).unwrap();
}
