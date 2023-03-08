use std::mem;
use std::os::windows::ffi::OsStringExt;
use winapi::shared::windef::HWND;
use winapi::um::winuser::{EnumWindows, GetWindowRect, GetWindowTextW, IsWindowVisible};
fn main() {
    unsafe {
        let mut x = String::from("[");
        //定义一个字符串x
        EnumWindows(Some(enum_windows_callback), &mut x as *mut _ as isize);
        //传递x的指针作为参数
        x.pop();
        x.pop();
        println!("{}]", x); //打印x
    }
}
unsafe extern "system" fn enum_windows_callback(hwnd: HWND, l_param: isize) -> i32 {
    if IsWindowVisible(hwnd) == 0 {
        return 1;
    }
    let mut rect = mem::zeroed();
    GetWindowRect(hwnd, &mut rect);
    let mut buffer: [u16; 512] = [0; 512];
    GetWindowTextW(hwnd, buffer.as_mut_ptr(), 512);
    let title = String::from_utf16_lossy(&buffer);

    //将打印的文字拼接到x上
    let x = &mut *(l_param as *mut String);
    *x += "{\n\"title\": \"";
    *x += &title;
    *x += "\",\n";

    *x += "\"x\": ";
    *x += &rect.left.to_string();
    *x += ",\n\"y\": ";
    *x += &rect.top.to_string();
    *x += ",\n";

    *x += "\"width\": ";
    *x += &(rect.right - rect.left).to_string();
    *x += ",\n\"height\": ";
    *x += &(rect.bottom - rect.top).to_string();
    *x += "\n},\n";

    1
}
