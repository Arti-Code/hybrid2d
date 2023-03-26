use bevy_prototype_lyon::shapes::Circle;

use crate::{prelude::*, LMBEvent};

/* #[derive(Resource)]
pub struct MarkerId(Entity);

impl Default for MarkerId {
    pub fn default(mut commands: Commands) -> Self {
        let mark_circle = shapes::Circle {center: Vec2::ZERO, radius: 30.0};
        let marker = commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&mark_circle),
            ..Default::default()
        },
        Stroke::color(Color::SILVER),
        )).id();
        return Self {0: marker};
    }
} */

#[derive(Component)]
pub struct Agent{
    speed: f32,
    turn: f32,
    size: f32,
}

#[derive(Component)]
pub struct AgentCondition {
    max_eng: f32,
    eng: f32,
}

#[derive(Component)]
pub struct Neuro;

#[derive(Component)]
pub struct NeuroTimer {
    value: Timer,
}

impl Default for NeuroTimer {
    fn default() -> Self { 
        Self {value: Timer::from_seconds(0.35, TimerMode::Repeating)} 
    }
}

impl Neuro {
    pub fn behave_random(&self) -> Vec2 {
        let mut rnd = thread_rng();
        match rnd.gen_range(0..20) {
            n if n <= 1 => {
                return Vec2::new(rnd.gen_range(0.0..=1.0), 0.0);
            },
            n if n == 2 => {
                return Vec2::new(0.0, rnd.gen_range(-1.0..=1.0));
            },
            _ => {
                return Vec2::ZERO;
            }
        }
    }
}

#[derive(Component)]
pub struct ThinkTimer {
    timer: Timer,
}

impl Default for ThinkTimer {
    fn default() -> Self {
        ThinkTimer { timer: Timer::from_seconds(1.0, TimerMode::Repeating) }
    }
}

#[derive(Component)]
pub struct Selected {
    shape: ShapeBundle,
    stroke: Stroke,
}

impl Selected {
    pub fn new(color: Color, size: f32, tf: &Transform) -> Self {
        let circle = shapes::Circle {center: Vec2::ZERO, radius: size};
        let shape = ShapeBundle {
            path: GeometryBuilder::build_as(&circle),
            transform: *tf,
            ..Default::default()
        };
        let stroke = Stroke::new(color, 1.0);
        Self { shape: shape, stroke: stroke }
    }
}

#[derive(Resource)]
pub struct SelectedAgent(Vec<Entity>);

impl Default for SelectedAgent {
    fn default() -> Self {
        Self {0: vec![] }
    }
}

#[derive(Component)]
pub struct Marked;

pub struct AgentPlugin;

impl Plugin for AgentPlugin {
    fn build(&self, app: &mut App) {
        //app.insert_resource(MarkerId);
        app.add_startup_system(create_agent_system);
        //app.add_startup_system(spawn_marker_system);
        app.add_system(update_agent_system);
        app.add_system(wrap_elements);
        app.add_system(selected_by_mouse);
    }
}


fn random_size(min: u32, max: u32) -> u32 {
    let mut rnd = thread_rng();
    return rnd.gen_range(min..=max);
}

fn random_position(x0: f32, x1: f32, y0: f32, y1: f32) -> Vec2 {
    let mut rnd = thread_rng();
    return Vec2::new(
        rnd.gen_range(x0..x1),
        rnd.gen_range(y0..y1)
    );
}

fn build_hexagon(size: f32) -> Vec<Vec2> {
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

pub fn create_agent_system(mut commands: Commands) {
    let colors = ColorBox::new_with_colors();
    for _ in 0..AGENTS_NUM {
        let size = random_size(4, 10) as f32;
        let color = colors.choose_color_from_count(4);
        let pos = random_position(-WIN_SIZE.x/2.0+50.0, WIN_SIZE.x/2.0-50.0, -WIN_SIZE.y+50.0, WIN_SIZE.y/2.0-50.0);
        let hex_shape = build_hexagon(size);
        let hex = shapes::RegularPolygon {
            center: Vec2::ZERO,
            sides: 6,
            feature: RegularPolygonFeature::Radius(size)
        };
        match Collider::convex_hull(&hex_shape) {
            Some(collider) => {
                commands.spawn((
                    ShapeBundle {
                        path: GeometryBuilder::build_as(&hex),
                        ..Default::default()
                    },
                    Stroke::new(color, 2.0),
                    RigidBody::Dynamic,
                ))
                .insert(Agent {speed: 5.0, turn: 0.01, size: size})
                .insert(AgentCondition {max_eng: size.powi(2)*PI, eng: size.powi(2)*PI})
                .insert(Neuro)
                .insert(ThinkTimer::default())
                .insert(TransformBundle::from_transform(Transform::from_translation(Vec3::new(pos.x, pos.y, 0.0))))
                .insert(collider)
                .insert(Restitution::coefficient(0.5))
                .insert(Friction::coefficient(0.3))
                .insert(Sleeping::disabled())
                .insert(Damping{
                    linear_damping: 1.0,
                    angular_damping: 1.0
                })
                .insert(ExternalForce{
                    force: Vec2::ZERO,
                    torque: 0.0,
                }).id();
            },
            None => warn!("creating hexagon agent collider has been breaked!"),
        }
    }
}

pub fn update_agent_system(mut agents_query: Query<(Entity, &Neuro, &Agent, &Transform, &mut ExternalForce, &mut ThinkTimer), (With<Agent>)>, time: Res<Time>) {
    let dt = time.delta().as_secs_f32();
    for (_, neuro, hex_agent, trans, mut impulse, mut think_timer) in agents_query.iter_mut() {
        think_timer.timer.tick(time.delta());
        if think_timer.timer.just_finished() {
            let mut movement = neuro.behave_random();
            movement.x *= hex_agent.speed;
            movement.y *= hex_agent.turn;
            let mut rot = trans.rotation.z;
            rot += PI;
            let (ry, rx) = rot.sin_cos();       
            let go = Vec2::new(rx, ry)*movement.x;
            impulse.force = go*0.05;
            impulse.torque = movement.y*0.01;
        }
    }
}

pub fn wrap_elements(mut agents: Query<(Entity, &mut Transform), With<Agent>>) {
    for (_, mut trans) in agents.iter_mut() {
        if trans.translation.x > WIN_SIZE.x/2.0 {
            trans.translation.x = -WIN_SIZE.x/2.0;
        } else if trans.translation.x < -WIN_SIZE.x/2.0 {
            trans.translation.x = WIN_SIZE.x/2.0;
        }
        if trans.translation.y > WIN_SIZE.y/2.0 {
            trans.translation.y = -WIN_SIZE.y/2.0;
        } else if trans.translation.y < -WIN_SIZE.y/2.0 {
            trans.translation.y = WIN_SIZE.y/2.0;
        }
    }
}

pub fn selected_by_mouse(
    mut commands: Commands,
    mut query_selected: Query<(Entity, &mut Marked, &Children), (With<Marked>)>,
    mut query_agents: Query<(Entity, &Transform, &Collider, &Agent), (With<Agent>)>,
    mut lmb_events: EventReader<LMBEvent>,
) {
    for event in lmb_events.iter() {
        for (selected_entity, marked, children) in query_selected.iter_mut() {
            let last = children.last().unwrap();
            commands.entity(*last).despawn_recursive();
            commands.entity(selected_entity).remove::<Marked>();
        }
        let coords = event.0;
        let m = Vec2::new(WIN_SIZE.x/2.0, WIN_SIZE.y/2.0);
        for (entity, tf, collider, hex_agent) in query_agents.iter_mut() {
            let mut pos = Vec2::new(tf.translation.x, tf.translation.y);
            pos += m;
            let dist = pos.distance(coords);
            //println!("[coord] x:{} | y:{}\t<==>\t[pos] x:{} | y:{}\t[dist] {}", coords.x.round(), coords.y.round(), pos.x.round(), pos.y.round(), dist.round());
            if dist <= hex_agent.size*1.2 {
                commands.entity(entity).insert(Marked);
                let mark_circle = shapes::Circle {center: Vec2::ZERO, radius: hex_agent.size*2.0};
                let marker = commands.spawn((
                    ShapeBundle {
                        path: GeometryBuilder::build_as(&mark_circle),
                        ..Default::default()
                    },
                    Stroke::color(Color::SILVER),
                )).id();
                commands.entity(entity).insert_children(0,&[marker]);
                return;
            }
        }
    } 
}
