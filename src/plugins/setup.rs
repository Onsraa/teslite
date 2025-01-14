use crate::plugins::car::CarPlugin;
use crate::plugins::ui::UIPlugin;

use bevy::{
    core::FrameCount,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::{PresentMode, WindowMode},
};

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Teslite".into(),
                    name: Some("bevy.app".into()),
                    present_mode: PresentMode::AutoNoVsync,
                    mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current),
                    enabled_buttons: bevy::window::EnabledButtons {
                        maximize: false,
                        ..Default::default()
                    },
                    visible: false,
                    ..default()
                }),
                ..default()
            }),
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin,
            UIPlugin,
            CarPlugin,
        ));
        app.add_systems(Startup, setup);
        app.add_systems(Update, (make_visible, exit_game));
    }
}


fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn make_visible(mut window: Single<&mut Window>, frames: Res<FrameCount>) {
    if frames.0 == 3 {
        window.visible = true;
    }
}

fn exit_game(keyboard_input: Res<ButtonInput<KeyCode>>, mut app_exit_events: EventWriter<AppExit>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_events.send(AppExit::Success);
    }
}