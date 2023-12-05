use bevy::prelude::*;

pub const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
pub const WINDOW_WIDTH:f32= 1200.0;
pub const WINDOW_HEIGHT:f32=600.0;
const SCORE_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);
const TEXT_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
const SCOREBOARD_FONT_SIZE: f32 = 40.0;
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);
pub struct SettingsPlugin;

impl Plugin for SettingsPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_scoreboard,setup_camera));

    }
}

#[derive(Component)]
pub struct Collider;

#[derive(Event, Default)]
pub struct CollisionEvent;


#[derive(Resource)]
pub struct Scoreboard{
    pub score:i32
}



fn setup_camera(mut commands:Commands){
    commands.spawn(
        Camera2dBundle::default()
    );

}

fn spawn_scoreboard(mut commands:Commands){


commands.spawn(
    TextBundle::from_sections([
        TextSection::new(
            "Score: ",
            TextStyle {
                font_size: SCOREBOARD_FONT_SIZE,
                color: TEXT_COLOR,
                ..default()
            },
        ),
        TextSection::from_style(TextStyle {
            font_size: SCOREBOARD_FONT_SIZE,
            color: SCORE_COLOR,
            ..default()
        }),
    ])
    .with_style(Style {
        position_type: PositionType::Absolute,
        top: SCOREBOARD_TEXT_PADDING,
        left: SCOREBOARD_TEXT_PADDING,
        ..default()
    }),
);

}