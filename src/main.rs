use bevy::prelude::*;
use bevy::window::{Window, WindowTheme};
mod paddles;
mod walls;
mod settings;
mod ball;

use paddles::*;
use walls::*;
use ball::*;
use settings::*;

fn main(){
    App::new()
    .add_plugins(
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Pong".into(),
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                window_theme: Some(WindowTheme::Dark),
                enabled_buttons: bevy::window::EnabledButtons {
                    maximize: false,
                    ..Default::default()
                },
                ..default()
            }),
            ..default()
            }
        )
    )

    .add_plugins(WallPlugin)
    .add_plugins(SettingsPlugin)
    .add_plugins(BallPlugin)
    .add_plugins(PaddlesPlugin)
    .add_event::<CollisionEvent>()
    .insert_resource(Scoreboard{score:0})
    .insert_resource(ClearColor(BACKGROUND_COLOR))
    .run();
}











