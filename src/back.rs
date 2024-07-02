use winapi::um::winuser::GetCursorPos;
use winapi::shared::windef::POINT;
use winapi::um::winuser::{SetWindowsHookExW, UnhookWindowsHookEx, CallNextHookEx};
use winapi::shared::windef::HHOOK;
use winapi::shared::minwindef::{LPARAM, WPARAM, LRESULT, DWORD};

unsafe extern "system" fn keyboard_proc(nCode: i32, wParam: WPARAM, lParam: LPARAM) -> LRESULT {
    if nCode >= 0 {
        println!("key pressed: {}", wParam);
    }
    CallNextHookEx(0 as HHOOK, nCode, wParam, lParam)
}

fn main() {
    let mut point = POINT { x: 0, y: 0 };
    unsafe {
        GetCursorPos(&mut point);
        println!("x: {}, y: {}", point.x, point.y);
    }

    unsafe {
        let hook = SetWindowsHookExW(
            winapi::um::winuser::WH_KEYBOARD_LL,
            Some(keyboard_proc),
            std::ptr::null_mut(),
        0
        );
        loop {
            let msg = winapi::um::winuser::GetMessageW(std::ptr::null_mut(), 0, 0, 0);
            if msg == 0 {
                break;
            } else {
                winapi::um::winuser::TranslateMessage(&msg);
                winapi::um::winuser::DispatchMessageW(&msg);
            }
        }
        UnhookWindowsHookEx(hook);
    }
}
