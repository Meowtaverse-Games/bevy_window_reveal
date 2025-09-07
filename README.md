# bevy_window_reveal

A Bevy plugin for controlling the initial visibility of the game window. This plugin allows you to hide the window at startup and reveal it after a specified number of frames or milliseconds, preventing flickering during initialization.

## Features

- Control when the window becomes visible
- Reveal after a set number of frames or milliseconds
- Set the initial clear color

## Installation

Add the following to your `Cargo.toml`:

```toml
bevy_window_reveal = { path = "path/to/bevy_window_reveal" }
```

## Usage

```rust
use bevy::prelude::*;
use bevy_window_reveal::{WindowRevealPlugin, WindowRevealConfig};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WindowRevealPlugin(WindowRevealConfig {
            frames_after_ready: 2,
            ms_after_ready: 500,
            initial_clear: Some(Color::BLACK),
        }))
        .run();
}
```

## Configuration

- `frames_after_ready`: Number of frames to wait after ready before revealing the window
- `ms_after_ready`: Milliseconds to wait after ready before revealing the window
- `initial_clear`: Initial clear color for the window

## License

MIT License