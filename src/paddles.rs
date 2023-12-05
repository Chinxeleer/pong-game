use bevy::prelude::*;
use crate::settings::*;
use crate::walls::{TOP_WALL,BOTTOM_WALL,WALL_THICKNESS};

pub const LEFT_PEDDAL_LOCATION:f32= -1.0*((WINDOW_WIDTH/2.0) * 0.75)   ;
pub const RIGHT_PEDDAL_LOCATION:f32= 1.0*((WINDOW_WIDTH/2.0) * 0.75);
pub const PADDLE_SIZE:Vec2=Vec2::new(10., 50.);
pub const PADDLE_SPEED:f32 = 400.0;
pub const PADDLE_COLOR:Color=Color::rgb(0.25, 0.25, 0.75);


pub struct PaddlesPlugin;

impl Plugin for PaddlesPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_paddles);
        app.add_systems(Update, paddles_movement);
        
    }
}


pub enum Position{
    Left,
    Right,
}

#[derive(Component)]
pub struct Paddle{
    pub side: Position,
}
 
pub fn paddles_movement(
    mut query: Query<(&mut Transform,&Paddle)>,
    keyboard: Res<Input<KeyCode>>,
    time:Res<Time>
){
    for (mut transform,paddle) in query.iter_mut(){
        match paddle.side{
            Position::Left => {
                if keyboard.pressed(KeyCode::W){
                   
                    transform.translation.y += PADDLE_SPEED * time.delta_seconds();
                }
                if keyboard.pressed(KeyCode::S){
                    transform.translation.y -= PADDLE_SPEED * time.delta_seconds();
                }
            },
            Position::Right => {
                if keyboard.pressed(KeyCode::Up){
                    transform.translation.y += PADDLE_SPEED * time.delta_seconds();
                }
                if keyboard.pressed(KeyCode::Down){
                    transform.translation.y -= PADDLE_SPEED * time.delta_seconds();
                }
            },
        }

        let bottom_bound = BOTTOM_WALL+2.3*WALL_THICKNESS;
        let top_bound = TOP_WALL - 2.3*WALL_THICKNESS;

        transform.translation.y =transform.translation.y.clamp(bottom_bound,top_bound);

    }

}

fn spawn_paddles(mut commands:Commands){
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(LEFT_PEDDAL_LOCATION, 0.0, 0.0),
                scale: PADDLE_SIZE.extend(1.0),
                ..default()
            },
            sprite: Sprite {
                color: PADDLE_COLOR,
                ..default()
            },
            ..default()
        },
        Paddle{side:Position::Left},
        Collider,
    ));
    
    
    
    
    // Paddle side == Right
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(RIGHT_PEDDAL_LOCATION, 0.0, 0.0),
                scale: PADDLE_SIZE.extend(1.0),
                ..default()
            },
            sprite: Sprite {
                color: PADDLE_COLOR,
                ..default()
            },
            ..default()
        },
        Paddle{side:Position::Right},
        Collider,
    ));
}