use bevy_egui::egui::RichText;
use bevy_egui::egui::Color32;
use crate::MousePosition;
use crate::UiStatus;
use crate::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(main_panel_system);
        app.add_system(mouse_window_system);
        app.add_system(delta_time_system);
    }
}


fn mouse_window_system(
    mut status: ResMut<UiStatus>,
    mouse_position: Res<MousePosition>,
    mut contexts: EguiContexts
) {
    let ctx = contexts.ctx_mut();
    let lmb = status.lmb;
    let rmb = status.rmb;
    egui::Window::new("MOUSE").collapsible(true).open(&mut status.is_test_win_open).show(ctx, |ui| {
        ui.heading(RichText::new("POSITION").color(Color32::BLUE).size(14.0).strong());
        let mut mouse_pos_txt = String::from("X:");
        mouse_pos_txt.push_str(&mouse_position.vec2.x.to_string());
        mouse_pos_txt.push_str(&" | Y:");
        mouse_pos_txt.push_str(&mouse_position.vec2.y.to_string());
        ui.label(RichText::new(mouse_pos_txt).size(12.0));
        let mut btn_text = String::from("");
        if lmb {
            btn_text.push_str("[LBM] ");
        }
        if rmb {
            btn_text.push_str("[RBM]")
        }
        ui.label(RichText::new(btn_text).color(Color32::GREEN).strong().size(12.0));
    });
}

fn delta_time_system(
    mut status: ResMut<UiStatus>,
    time: Res<Time>,
    mut contexts: EguiContexts
) {
    let ctx = contexts.ctx_mut();
    egui::Window::new("TIME").collapsible(true).open(&mut status.is_time_win_open).show(ctx, |ui| {
        let dt = (time.delta_seconds_f64()*1000.0).round();
        let fps = (1000.0/dt).round() as i32;
        ui.heading(RichText::new("TIME").color(Color32::RED).size(12.0).strong());
        let mut delta = String::from("dT: ");
        delta.push_str(&dt.to_string());
        delta.push_str(&"ms");
        ui.label(RichText::new(delta).size(12.0));
        let mut frames = String::from("FPS: ");
        frames.push_str(&fps.to_string());
        ui.label(RichText::new(frames).size(12.0));
    });
}

fn main_panel_system(
    mut status: ResMut<UiStatus>,
    mut contexts: EguiContexts
) {
    let ctx = contexts.ctx_mut();
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            ui.heading(RichText::new( "H Y B R I D").color(Color32::GREEN).strong().heading());
            ui.add_space(5.0);
            ui.separator();
            ui.add_space(5.0);
            egui::menu::menu_button(ui, RichText::new("Simulation").strong(), |ui| {
                if ui.button("New Simulation").clicked() {
                }
                if ui.button("Load Simulation").clicked() {
                }
                if ui.button("Save Simulation").clicked() {
                }
                if ui.button(RichText::new("Quit").color(Color32::RED).strong()).clicked() {
                    std::process::exit(0);
                }
            });
            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);
            egui::menu::menu_button(ui, RichText::new("Tools").strong(), |ui| {
                if ui.button("Hybrid Library").clicked() {
                }
                if ui.button("Inspector").clicked() {
                }
                if ui.button("Hybrydizer").clicked() {
                }
                if ui.button("Creator").clicked() {
                }
            });
            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);
            egui::menu::menu_button(ui, RichText::new("View").strong(), |ui| {
                if ui.button(RichText::new("Mouse Window").color(Color32::BLUE).strong()).clicked() {
                    status.is_test_win_open = !status.is_test_win_open;
                }
                if ui.button(RichText::new("Time Window").color(Color32::RED).strong()).clicked() {
                    status.is_time_win_open = !status.is_time_win_open;
                }
                if ui.button("Zoom In").clicked() {
                }
                if ui.button("Zoom Out").clicked() {
                }
                if ui.button("MiniMap").clicked() {
                }
            });
            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);
            egui::menu::menu_button(ui, RichText::new("About").strong(), |ui| {
                if ui.button("Documentation").clicked() {
                }
                if ui.button("Changes Log").clicked() {
                }
                if ui.button("About").clicked() {
                }
            });
        });
    });
}