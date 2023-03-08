use std::mem;
use winapi::um::winuser::{EnumWindows, GetWindowRect, GetWindowTextW, IsWindowVisible};
use winapi::shared::windef::HWND;
use std::os::windows::ffi::OsStringExt;
fn main() {
    unsafe {
        EnumWindows(Some(enum_windows_callback), 0);
    }
}
unsafe extern "system" fn enum_windows_callback(hwnd: HWND, _l_param: isize) -> i32 {
    if IsWindowVisible(hwnd) == 0 {
        return 1;
    }
    let mut rect = mem::zeroed();
    GetWindowRect(hwnd, &mut rect);
    let mut buffer: [u16; 512] = [0; 512];
    GetWindowTextW(hwnd, buffer.as_mut_ptr(), 512);
    let title = String::from_utf16_lossy(&buffer);
    println!("Window title: {}", title);
    println!("Window position: ({}, {})", rect.left, rect.top);
    println!("Window size: {} x {}", rect.right - rect.left, rect.bottom - rect.top);
    println!("");
    1
}

