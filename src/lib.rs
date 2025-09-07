use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowEvent};

/// Configuration for the window reveal plugin.
/// 
/// - `frames_after_ready`: Number of frames to wait after the window is ready before revealing.
/// - `ms_after_ready`: Number of milliseconds to wait after the window is ready before revealing.
/// - `initial_clear`: Optional initial clear color for the window.
#[derive(Resource, Clone)]
pub struct WindowRevealConfig {
    pub frames_after_ready: u32,
    pub ms_after_ready: u64,
    pub initial_clear: Option<Color>,
}

impl Default for WindowRevealConfig {
    fn default() -> Self {
        Self {
            frames_after_ready: 2,
            ms_after_ready: 0,
            initial_clear: Some(Color::BLACK),
        }
    }
}

/// Bevy plugin for controlling the initial visibility of the primary window.
/// 
/// This plugin hides the window at startup and reveals it after a specified number of frames or milliseconds.
/// It can also set the initial clear color.
#[derive(Default)]
pub struct WindowRevealPlugin(pub WindowRevealConfig);

#[derive(Resource, Default)]
struct RevealState {
    status: RevealStatus,
    frames_after_ready: u32,
    ms_elapsed: u64,
}

#[derive(Default, PartialEq, Eq)]
enum RevealStatus {
    #[default]
    NotReady,
    Ready,
    Revealed,
}

impl Plugin for WindowRevealPlugin {
    fn build(&self, app: &mut App) {
        if let Some(c) = self.0.initial_clear {
            app.insert_resource(ClearColor(c));
        }

        app.insert_resource(self.0.clone());
        app.init_resource::<RevealState>();

        app.add_systems(Startup, startup);
        app.add_systems(
            Update,
            (catch_ready, accumulate, reveal_if_safe)
                .run_if(|state: Res<RevealState>| state.status != RevealStatus::Revealed),
        );
    }
}

/// System to hide the window at startup.
fn startup(mut q_window: Query<&mut Window, With<PrimaryWindow>>) {
    if let Ok(mut w) = q_window.single_mut() {
        w.visible = false;
        w.transparent = false;
    }
}

/// System to detect when the window is ready.
fn catch_ready(mut ev: EventReader<WindowEvent>, mut state: ResMut<RevealState>) {
    if state.status == RevealStatus::NotReady && ev.read().next().is_some() {
        state.status = RevealStatus::Ready;
        state.frames_after_ready = 0;
        state.ms_elapsed = 0;
    }
}

/// System to accumulate frames and time after the window is ready.
fn accumulate(time: Res<Time>, cfg: Res<WindowRevealConfig>, mut state: ResMut<RevealState>) {
    if state.status != RevealStatus::Ready {
        return;
    }
    state.frames_after_ready += 1;
    if cfg.ms_after_ready > 0 {
        state.ms_elapsed = (state.ms_elapsed as f64 + time.delta_secs_f64() * 1000.0) as u64;
    }
}

/// System to reveal the window when the configured conditions are met.
fn reveal_if_safe(
    cfg: Res<WindowRevealConfig>,
    mut state: ResMut<RevealState>,
    mut qwin: Query<&mut Window, With<PrimaryWindow>>,
) {
    if state.status != RevealStatus::Ready {
        return;
    }

    let frames_ok = state.frames_after_ready >= cfg.frames_after_ready;
    let time_ok = cfg.ms_after_ready > 0 && state.ms_elapsed >= cfg.ms_after_ready;

    if frames_ok || time_ok {
        if let Ok(mut w) = qwin.single_mut() {
            w.visible = true;
        }

        state.status = RevealStatus::Revealed;
    }
}
