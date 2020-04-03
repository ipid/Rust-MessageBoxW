#![windows_subsystem = "windows"]

use winapi::{
    shared::windef::HWND,
    um::{
        winuser::{MessageBoxW, MB_ICONINFORMATION, MB_OK},
    }
};
use std::iter::once;

fn win_str(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(once(0)).collect()
}

fn main() {
    unsafe {
        MessageBoxW(
            0 as HWND,
            win_str("我好了").as_ptr(),
            win_str("🍊").as_ptr(),
            MB_ICONINFORMATION | MB_OK,
        );
    }
}
