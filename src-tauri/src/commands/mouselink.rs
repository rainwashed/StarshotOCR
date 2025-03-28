#[cfg(target_os = "windows")]
static MOUSE_CAPTURE_METHOD: &str = "win32";

use windows::Win32 as win32;

#[tauri::command]
pub fn mouse_position() -> [i32; 2] {
    unsafe {
        let mut mouse_pos = win32::Foundation::POINT{x: 0, y: 0};
        if win32::UI::WindowsAndMessaging::GetCursorPos(&mut mouse_pos).is_ok() {
            return [mouse_pos.x, mouse_pos.y];
        } else {
            eprintln!("error getting mouse position.");
            return [-9999, -9999];
        }
    }
}

#[tauri::command]
pub fn return_mouse_pos() -> [i32; 2] {
    println!("CALLLED THIS SHIT");
    return [222, 123];
}

#[tauri::command]
pub fn return_mouse_state() -> [bool; 3] {
    if MOUSE_CAPTURE_METHOD != "win32" {
        panic!("unsupported OS mouse capture method")
    }

    unsafe {
        let left_mouse_down = {
            win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState(win32::UI::Input::KeyboardAndMouse::VK_LBUTTON.0 as i32) < 0
        };
        let right_mouse_down = {
            win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState(win32::UI::Input::KeyboardAndMouse::VK_RBUTTON.0 as i32) < 0
        };
        let middle_mouse_down = {
            win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState(win32::UI::Input::KeyboardAndMouse::VK_MBUTTON.0 as i32) < 0
        };

        return [left_mouse_down, right_mouse_down, middle_mouse_down];
    }
}