#![windows_subsystem = "windows"]

use winapi::{
    shared::windef::HWND,
    um::{
        winuser::{MessageBoxW, MB_ICONINFORMATION, MB_OK},
    }
};

fn win_str(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

fn main() {
    unsafe {
        MessageBoxW(
            0 as HWND,
            win_str("世界！").as_ptr(),
            win_str("ハロー").as_ptr(),
            MB_ICONINFORMATION | MB_OK,
        );
    }
}
