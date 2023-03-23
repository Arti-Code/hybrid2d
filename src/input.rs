use bevy::window::PrimaryWindow;

use crate::{prelude::*, MousePosition, UiStatus, LMBEvent};

/* pub fn mouse_button_input(buttons: Res<Input<MouseButton>>, windows: Res<Windows>) {
    let window = windows.get_primary().unwrap();
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(mouse_pos) = window.cursor_position() {
            info!("LEFT BUTTON");
        } else {
            info!("LEFT BUTTON");
        }
    }
    if buttons.just_pressed(MouseButton::Right) {
        info!("RIGHT BUTTON");
    }
} */

pub fn mouse_button_input(
    buttons: Res<Input<MouseButton>>, 
    windows: Query<&Window, With<PrimaryWindow>>, 
    mut mouse_position: ResMut<MousePosition>, 
    mut status: ResMut<UiStatus>,
    mut lmb_event: EventWriter<LMBEvent>
) {
    let window = windows.get_single().unwrap();
    if let Some(mouse_pos) = window.cursor_position() {
        mouse_position.from_vec2(mouse_pos);
    }
    if buttons.just_pressed(MouseButton::Left) {
        status.lmb = true;
        lmb_event.send(LMBEvent(mouse_position.vec2));
    }
    if buttons.just_released(MouseButton::Left) {
        status.lmb = false;
    }
    if buttons.just_pressed(MouseButton::Right) {
        status.rmb = true;
    }
    if buttons.just_released(MouseButton::Right) {
        status.rmb = false;
    }
}