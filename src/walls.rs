use bevy::prelude::*;
use crate::settings::*;


pub const LEFT_WALL:f32 = -1.0*((WINDOW_WIDTH/2.0) * 0.8);
pub const RIGHT_WALL:f32 =1.0*((WINDOW_WIDTH/2.0) * 0.8);
pub const TOP_WALL:f32 =1.0*((WINDOW_HEIGHT/2.0) * 0.8);
pub const BOTTOM_WALL:f32 =-1.0*((WINDOW_HEIGHT/2.0) * 0.8);
pub const WALL_THICKNESS:f32 = 15.0;
pub const WALL_COLOR:Color = Color::rgb(1.0, 0.5, 0.5);


pub struct WallPlugin;

impl Plugin for WallPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_walls);
    }
}



#[derive(Bundle)]
struct WallBundle{
    sprite_bundle:SpriteBundle,
    collider:Collider,
}

#[derive(Debug,PartialEq, Eq)]
pub enum WallLocation{
    Right,
    Left,
    Bottom,
    Top
}

#[derive(Component,Debug,PartialEq, Eq)]
pub struct Points(pub WallLocation);


impl WallLocation {
     fn position(&self) -> Vec2 {
            match self{
                WallLocation::Right => Vec2::new(RIGHT_WALL,0.0),
                WallLocation::Left => Vec2::new(LEFT_WALL, 0.0),
                WallLocation::Bottom => Vec2::new(0.0,BOTTOM_WALL),
                WallLocation::Top => Vec2::new(0.0,TOP_WALL),
            }
    }

    pub fn size(&self) -> Vec2 {
        let arena_height = WINDOW_HEIGHT*0.8;
        let arena_width:f32 = WINDOW_WIDTH*0.8;
        assert!(arena_width>0.0);
        assert!(arena_height>0.0);

        match self{
            WallLocation::Right | WallLocation::Left =>{
                Vec2::new(WALL_THICKNESS, arena_height+WALL_THICKNESS)
            }
            WallLocation::Bottom | WallLocation::Top =>{
                Vec2::new( arena_width+WALL_THICKNESS,WALL_THICKNESS)
            }
            
            
        }
    }
    
}

impl WallBundle{
    pub fn new(location:WallLocation) -> WallBundle{
        WallBundle {
            sprite_bundle: SpriteBundle{
                transform:Transform { translation: location.position().extend(0.0),
                      scale: location.size().extend(1.0) ,
                      ..default()
                    },
                  sprite: Sprite{
                    color: WALL_COLOR,
                    ..default()
                  },
                  ..default()
            },
            collider:Collider

        }
    }
}

fn setup_walls(mut commands:Commands){

commands.spawn((WallBundle::new(WallLocation::Left),Points(WallLocation::Left)));
commands.spawn((WallBundle::new(WallLocation::Right),Points(WallLocation::Right)));
commands.spawn((WallBundle::new(WallLocation::Top),Points(WallLocation::Top)));
commands.spawn((WallBundle::new(WallLocation::Bottom),Points(WallLocation::Bottom)));
}