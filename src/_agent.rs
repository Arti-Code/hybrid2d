use std::f32::consts::PI;

use crate::prelude::*;

pub struct AgentPlugin;

impl Plugin for AgentPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_agents_system);
        app.add_startup_system(spawn_ball_system);
        app.add_startup_system(spawn_hex_system);
    }
}


fn spawn_agents_system(mut commands: Commands) {
    let mut rnd = thread_rng();
    let shape = shapes::Circle {center: Vec2 { x: 0.0, y: 0.0 }, radius: 20.0};
    let y = rnd.gen_range(0.0..WIN_SIZE.y/2.0);
    let x = rnd.gen_range(-WIN_SIZE.x/2.0..WIN_SIZE.x/2.0);
    commands.spawn((ShapeBundle {
            path: GeometryBuilder::build_as(&shape),
            ..Default::default()
        },
        Stroke::new(Color::GRAY, 1.0),
        //Fill::color(Color::YELLOW),
    ))
    .insert(TransformBundle::from_transform(Transform::from_xyz(x, y, 0.0)))
    .insert(RigidBody::Dynamic)
    .insert(Collider::ball(24.0))
    .insert(Sleeping::disabled())
    .insert(Damping {linear_damping: 0.0, angular_damping: 0.5});
}

fn spawn_hex_system(mut commands: Commands) {
    let mut rnd = thread_rng();
    for _ in 0..7 {
        let hex = shapes::RegularPolygon {center: Vec2::ZERO, sides: 6, feature: RegularPolygonFeature::Radius(30.0)};
        let y = rnd.gen_range(100.0..WIN_SIZE.y/2.0);
        let x = rnd.gen_range(-WIN_SIZE.x/2.0+50.0..WIN_SIZE.x/2.0-50.0);
        //let collider_hex2 = Collider::from::<ShapeShape>()
        let collider_hex = Collider::polyline(build_hexagon(30.0), vec![5 as u32; 2]);
        match collider_hex {
            Some(c) => {
                commands.spawn((ShapeBundle {
                    path: GeometryBuilder::build_as(&hex),
                    ..Default::default()
                },
                Stroke::new(Color::GRAY, 0.0),
                //Fill::color(Color::YELLOW),
            ))
            .insert(TransformBundle::from_transform(Transform::from_xyz(x, y, 0.0)))
            .insert(RigidBody::Dynamic)
            .insert(c)
            .insert(Ccd::enabled())
            .insert(Sleeping::disabled());
            //.insert(Damping {linear_damping: 0.0, angular_damping: 0.5});
        },
        None => {
                warn!("hex collider building broken!");
            }
        };
    }
}

fn spawn_ball_system(mut commands: Commands) {
    let mut rnd = thread_rng();
    let shape = shapes::Circle {center: Vec2 { x: 0.0, y: 0.0 }, radius: 20.0};
    let y = rnd.gen_range(0.0..WIN_SIZE.y/2.0);
    let x = rnd.gen_range(-WIN_SIZE.x/2.0..WIN_SIZE.x/2.0);
    commands.spawn((ShapeBundle {
            path: GeometryBuilder::build_as(&shape),
            ..Default::default()
        },
        Stroke::new(Color::GRAY, 1.0),
        //Fill::color(Color::YELLOW),
    ))
    .insert(TransformBundle::from_transform(Transform::from_xyz(x, y, 0.0)))
    .insert(RigidBody::Dynamic)
    .insert(Collider::ball(24.0))
    //.insert(collider)
    .insert(Sleeping::disabled())
    .insert(Damping {linear_damping: 0.0, angular_damping: 0.5});
}

pub fn build_hexagon(size: f32) -> Vec<Vec2> {
    let mut vertices = vec![];
    let s = 2.0*PI/6.0;
    let mut a: f32 = s/2.0;
    //let mut a: f32 = 0.0;
    for _ in 0..6 {
        let x = a.sin()*size; let y = a.cos()*size;
        let v = Vec2::new(x, y);
        vertices.push(v);
        a += s;
    }
    return vertices;
}