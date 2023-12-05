use bevy::{prelude::*, sprite::{collide_aabb::{collide, Collision}, MaterialMesh2dBundle}};
// use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use crate::settings::{Collider,CollisionEvent, Scoreboard};
use crate::walls::{Points,WallLocation};
// this will be changes later as I will make a ui to choose the first player.
pub const INITIAL_BALL_DIRECTION:Vec2 = Vec2::new(-0.5,0.5);
// pub const BALL_RADIUS :f32= 10.0;
pub const BALL_SPEED:f32 = 300.0;
pub const BALL_SIZE:Vec3=Vec3::new(10.0, 10.0, 0.0);





pub struct BallPlugin;

impl Plugin for BallPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ball);
        app.add_systems(Update, (ball_collisions,apply_velocity));

    }
}





#[derive(Component)]
pub struct Ball;

#[derive(Component,Deref,DerefMut)]
pub struct Velocity(pub Vec2);

pub fn ball_collisions(
    mut scoreboard:ResMut<Scoreboard>,
    mut ball_query:Query<(&mut Velocity,&Transform),With<Ball>>,
    collider_query:Query<(Entity,&Transform,Option<&Points>),With<Collider>>,
    mut collision_events:EventWriter<CollisionEvent>


){
    let (mut ball_velocity,ball_transform) = ball_query.single_mut();
    let ball_size = ball_transform.scale.truncate();

    for (_,transform,maybe_points) in &collider_query{
        let collision = collide(
            ball_transform.translation,
            ball_size,
            transform.translation,
            transform.scale.truncate()
        );

        if let Some(collision)= collision{



            collision_events.send_default();

            if maybe_points.is_some_and(|val| *val == Points(WallLocation::Left)) {
               scoreboard.score-=1;
            }

            if maybe_points.is_some_and(|val| *val == Points(WallLocation::Left)) {
               scoreboard.score+=1;
            }

            let mut reflect_x = false;
            let mut reflect_y =false;

               // only reflect if the ball's velocity is going in the opposite direction of the
            // collision
            match collision {
                Collision::Left => reflect_x = ball_velocity.x > 0.0,
                Collision::Right => reflect_x = ball_velocity.x < 0.0,
                Collision::Top => reflect_y = ball_velocity.y < 0.0,
                Collision::Bottom => reflect_y = ball_velocity.y > 0.0,
                Collision::Inside => { /* do nothing */ }
            }

            if reflect_x{
                ball_velocity.x = -ball_velocity.x;
            }

            if reflect_y{
                ball_velocity.y = -ball_velocity.y;
            }
    }

    }

}

pub fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}

fn spawn_ball(mut commands:Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
){
    commands.spawn((MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::default().into()).into(),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)).with_scale(BALL_SIZE),
        ..default()
    },
    Ball,
    Velocity(INITIAL_BALL_DIRECTION.normalize()*BALL_SPEED)
));
}