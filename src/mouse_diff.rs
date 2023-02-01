use glam::Vec2;
use minifb::{MouseMode, Window};

use crate::camera::Camera;

pub fn change_fov(window: &Window, camera: &mut Camera, dt: f32) {
    let dir = window.get_scroll_wheel().unwrap_or((0.0, 0.0));
    camera.add_fov(-dir.1 * dt);
}

fn set_mouse_pos(x: i32, y: i32) -> Result<(), String> {
    unsafe {
        if winapi::um::winuser::SetCursorPos(x, y) == 0 {
            Err("SetCursorPos failed".into())
        } else {
            Ok(())
        }
    }
}

pub fn mouse_diff_fn(
    window: &Window,
    ignore_mouse: &mut bool,
    mouse_last_frame: &mut (f32, f32),
    dt: f32,
) -> Vec2 {
    //window position top left
    let window_pos_tl_x = window.get_position().0 as i32;
    let window_pos_tl_y = window.get_position().1 as i32;

    //window position bottom right
    let window_pos_br_x = window.get_position().0 as i32 + window.get_size().0 as i32;
    let window_pos_br_y = window.get_position().1 as i32 + window.get_size().1 as i32;

    let rect = winapi::shared::windef::RECT {
        left: window_pos_tl_x,
        top: window_pos_tl_y,
        right: window_pos_br_x,
        bottom: window_pos_br_y,
    };
    //The code above doesn't need to be calculated every frame, only when window moves.

    // This doesn't really work, it always returns 32 on my machine...
    // Might cause bugs on another machine
    let cursor_width =
        unsafe { winapi::um::winuser::GetSystemMetrics(winapi::um::winuser::SM_CXCURSOR) };
    let cursor_height =
        unsafe { winapi::um::winuser::GetSystemMetrics(winapi::um::winuser::SM_CYCURSOR) };

    unsafe {
        //Keeps cursor WITHIN the confines of the window
        winapi::um::winuser::ClipCursor(&rect);
    }

    //mouse window space
    let mouse = window.get_mouse_pos(MouseMode::Pass).unwrap();

    //mouse screen space
    let mouse_ss_x = mouse.0 + window.get_position().0 as f32;
    let mouse_ss_y = mouse.1 + window.get_position().0 as f32;

    //Calculate the difference between mouse this frame and last frame
    let mut diff = Vec2::new(mouse_last_frame.0 - mouse.0, mouse_last_frame.1 - mouse.1) * dt;

    //Save info for the mouse this frame for next
    *mouse_last_frame = mouse;

    //If the mouse has warped last frame, don't take this frame into account because data is wrong.
    if *ignore_mouse {
        diff = Vec2::splat(0.0);
        *ignore_mouse = false;
    }

    if mouse_ss_x < window_pos_tl_x as f32 {
        set_mouse_pos(window_pos_br_x - cursor_width, mouse_ss_y as i32).unwrap();
        *ignore_mouse = true;
    }

    if mouse_ss_y < window_pos_tl_y as f32 {
        set_mouse_pos(mouse_ss_x as i32, window_pos_br_y - cursor_width).unwrap();
        *ignore_mouse = true;
    }

    if mouse_ss_x >= window_pos_br_x as f32 - cursor_width as f32 {
        set_mouse_pos(window_pos_tl_x + cursor_width, mouse_ss_y as i32).unwrap();
        *ignore_mouse = true;
    }

    if mouse_ss_y >= window_pos_br_y as f32 - cursor_width as f32 {
        set_mouse_pos(mouse_ss_x as i32, window_pos_tl_y + cursor_width).unwrap();
        *ignore_mouse = true;
    }

    diff
}
