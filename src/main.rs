#![allow(unused)]

mod consts;
mod ui;
mod agent;
mod input;
mod util;

mod prelude {
    pub use std::f32::consts::PI;
    pub use bevy::prelude::*;
    pub use bevy::window::Window;
    pub use bevy_rapier2d::prelude::*;
    pub use bevy_egui::{egui, EguiContexts, EguiPlugin};
    pub use bevy_inspector_egui::quick::WorldInspectorPlugin;
    pub use bevy_prototype_lyon::prelude::*;
    pub use rand::{thread_rng, Rng};
    pub use image::open;
    pub use crate::consts::*;
    pub use crate::ui::*;
    pub use crate::agent::{create_agent_system, Agent, AgentPlugin};
    pub use crate::input::*;
    pub use crate::util::*;
}

use crate::prelude::*;
use bevy::window::WindowResolution;



fn main() {
    App::new()
        .add_event::<LMBEvent>()
        .add_plugins(DefaultPlugins.set(
            WindowPlugin {
                primary_window: Some(Window {
                    title: "HYBRID".to_string(),
                    resolution: WindowResolution::new(WIN_SIZE.x, WIN_SIZE.y),
                    position: WindowPosition::Centered(MonitorSelection::Primary),
                    ..default()
                }),
                ..default()
            }
        ))
        .add_plugin(EguiPlugin)
        //.add_plugin(WorldInspectorPlugin::new())
        .insert_resource(ClearColor(Color::rgb(0.,0.,0.)))
        .init_resource::<MousePosition>()
        .init_resource::<UiStatus>()
        .add_plugin(ShapePlugin)
        .insert_resource(RapierConfiguration{
            gravity: Vec2::ZERO,
            //gravity: Vect::Y * -0.81 * 10.0,
            timestep_mode: TimestepMode::Fixed { dt: 1.0/30.0, substeps: 1 },
            physics_pipeline_active: true,
            query_pipeline_active: true,
            scaled_shape_subdivision: 10,
            force_update_from_transform_changes: false,
        })
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin {
            enabled: false, 
            always_on_top: false, 
            mode: DebugRenderMode::COLLIDER_SHAPES,
            style: DebugRenderStyle::default()
        })
        .add_startup_system(setup_graphics_system)
        .add_plugin(UiPlugin)
        .add_plugin(AgentPlugin)
        .add_system(mouse_button_input)
        .run();
}

pub struct LMBEvent(Vec2);

#[derive(Resource)]
pub struct UiStatus {
    is_test_win_open: bool,
    is_time_win_open: bool,
    clicks_num: u32,
    lmb: bool,
    rmb: bool,
}

impl Default for UiStatus {
    fn default() -> UiStatus {
        UiStatus {
            clicks_num: 0,
            is_test_win_open: false,
            is_time_win_open: false,
            lmb: false,
            rmb: false
        }
    }
}

#[derive(Default, Resource)]
pub struct MousePosition{
    vec2: Vec2,
}

impl MousePosition {
    pub fn from_vec2(&mut self, pos: Vec2) {
        self.vec2 = pos;
    }
}

pub struct ThinkEvent {
    pub message: String,
}

fn setup_graphics_system(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    let font = assets.load("font\\FiraCode-Bold.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 16.0,
        color: Color::LIME_GREEN,
    };
    let text_align = TextAlignment::Center;
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("HYBRID", text_style.clone())
            .with_alignment(text_align),
            transform: Transform::from_translation(Vec3 {x:0.0, y:350.0, z:0.0}),
            ..default()
        },
    ));
}

fn events_listener_system(mut events: EventReader<ThinkEvent>) {
    for ev in events.iter() {
        info!("{}", ev.message);
    }
}