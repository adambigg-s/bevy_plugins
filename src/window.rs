use bevy::prelude::*;
use bevy::window::CursorGrabMode;
use bevy::window::PrimaryWindow;

pub struct WindowManagerPlugin;

impl Plugin for WindowManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, window_status);
        app.add_systems(Update, window_lock);
    }
}

fn window_status(mut event: EventWriter<AppExit>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::Escape) {
        event.write(AppExit::Success);
    }
}

fn window_lock(mut window: Query<&mut Window, With<PrimaryWindow>>, keys: Res<ButtonInput<KeyCode>>) {
    let Ok(mut window) = window.single_mut()
    else {
        return;
    };

    if keys.just_pressed(KeyCode::KeyL) {
        match window.cursor_options.grab_mode {
            | CursorGrabMode::None => window.cursor_options.grab_mode = CursorGrabMode::Confined,
            | _ => window.cursor_options.grab_mode = CursorGrabMode::None,
        }
    }
}
