use bevy::prelude::*;
use bevy_window_reveal::{WindowRevealPlugin, WindowRevealConfig};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                visible: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(WindowRevealPlugin(WindowRevealConfig {
            frames_after_ready: 2,
            ms_after_ready: 500,
            initial_clear: Some(Color::BLACK),
        }))
        .run();
}
