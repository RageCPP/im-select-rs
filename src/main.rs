use std::env;
use std::process;
use windows::Win32::Foundation::*;
// use windows::Win32::UI::Input::KeyboardAndMouse::GetKeyboardLayout;
use windows::Win32::UI::WindowsAndMessaging::GetForegroundWindow;
// use windows::Win32::UI::WindowsAndMessaging::GetWindowThreadProcessId;
use windows::Win32::UI::WindowsAndMessaging::PostMessageW;
use windows::Win32::UI::WindowsAndMessaging::WM_INPUTLANGCHANGEREQUEST;
/// 取得当前前台窗口的输入法 Locale ID
// unsafe fn get_input_method() -> i32 {
//     unsafe {
//         let hwnd = GetForegroundWindow();
//         if hwnd.is_invalid() {
//             return 0;
//         }
//         let tid = GetWindowThreadProcessId(hwnd, None);
//         // https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getkeyboardlayout
//         // https://learn.microsoft.com/en-us/windows/win32/intl/language-identifiers
//         let hkl = GetKeyboardLayout(tid);
//         // HKL 的低 16 位是 LANGID
//         // 在 Windows API 的 Rust 绑定中，HKL（输入法句柄）通常被定义为一个新类型（例如 struct HKL(pub isize);），它的 .0 字段本质上就是一个整数（isize），而不是指针。
//         (hkl.0 as u32 & 0xFFFF) as i32
//     }
// }

/// 请求切换输入法
fn switch_input_method(locale: i32) {
    unsafe {
        let hwnd = GetForegroundWindow();
        if hwnd.is_invalid() {
            panic!("No foreground window found");
        }
        let lparam = LPARAM(locale as isize);
        // 发送语言切换请求
        let _ = PostMessageW(Some(hwnd), WM_INPUTLANGCHANGEREQUEST, WPARAM(0), lparam);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        // 有参数：解析并切换
        let locale: i32 = args[1].parse().unwrap_or_else(|_| {
            if args[1] != "2052" && args[1] != "1033" {
                eprintln!("Invalid number: {}, only 2052 or 1033 are allowed", args[1]);
            }
            process::exit(1);
        });
        switch_input_method(locale);
        return;
    }
    println!("Usage: im-select 2052 | 1033");
}
