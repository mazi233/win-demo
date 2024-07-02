use winapi::um::winuser::{CreateWindowExW, DefWindowProcW, RegisterClassW, MSG, WM_CHAR, WM_DESTROY, WM_SETFOCUS, WM_SIZE, WNDCLASSW, WS_BORDER, WS_CHILD, WS_EX_CLIENTEDGE, WS_VISIBLE, WS_VSCROLL};
use winapi::shared::windef::{HBRUSH, HMENU, HWND, RECT};
use winapi::shared::minwindef::{LPARAM, LRESULT, UINT, WPARAM};
use winapi::um::libloaderapi::GetModuleHandleW;
use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;

unsafe extern "system" fn wnd_proc(hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_SIZE => {
            let mut rect: RECT = std::mem::zeroed();
            winapi::um::winuser::GetClientRect(hwnd, &mut rect);
            let edit_hwnd = winapi::um::winuser::GetDlgItem(hwnd, 100);
            winapi::um::winuser::SetWindowPos(edit_hwnd, std::ptr::null_mut(), rect.left, rect.top, rect.right - rect.left, rect.bottom - rect.top, 0);
        },
        WM_DESTROY => {
            winapi::um::winuser::PostQuitMessage(0);
        },
        WM_SETFOCUS => {
            let edit_hwnd = winapi::um::winuser::GetDlgItem(hwnd, 100);
            winapi::um::winuser::SetFocus(edit_hwnd);
        },
        WM_CHAR => {
            let edit_hwnd = winapi::um::winuser::GetDlgItem(hwnd, 100);
            let mut text: [u16; 2] = [0; 2];
            let len = winapi::um::winuser::GetWindowTextW(edit_hwnd, text.as_mut_ptr(), 2);
            if len == 0 {
                winapi::um::winuser::SetWindowTextW(edit_hwnd, &wparam as *const _ as *const u16);
            } else {
                let mut buffer: Vec<u16> = Vec::with_capacity(len as usize + 1);
                buffer.set_len(len as usize);
                winapi::um::winuser::GetWindowTextW(edit_hwnd, buffer.as_mut_ptr(), len + 1);
                buffer.push(wparam as u16);
                winapi::um::winuser::SetWindowTextW(edit_hwnd, buffer.as_ptr());
            }
        },
        _ => return DefWindowProcW(hwnd, msg, wparam, lparam),
    }
    0
}

fn main() {
    unsafe {
        let hinstance = GetModuleHandleW(std::ptr::null());
        let class_name: Vec<u16> = OsStr::new("my_window_class").encode_wide().chain(once(0)).collect();
        let wndclass = WNDCLASSW {
            style: 0,
            lpfnWndProc: Some(wnd_proc),
            hInstance: hinstance,
            lpszClassName: class_name.as_ptr(),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hIcon: std::ptr::null_mut(),
            hCursor: std::ptr::null_mut(),
            hbrBackground: winapi::um::winuser::COLOR_WINDOW as HBRUSH,
            lpszMenuName: std::ptr::null_mut(),
        };
        RegisterClassW(&wndclass);
        let hwnd = CreateWindowExW(
            WS_EX_CLIENTEDGE,
            class_name.as_ptr(),
            OsStr::new("My Window").encode_wide().chain(once(0)).collect::<Vec<_>>().as_ptr(),
            WS_CHILD | WS_VISIBLE,
            0,
            0,
            0,
            0,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            hinstance,
            std::ptr::null_mut(),
        );
        let edit_hwnd = CreateWindowExW(
            0,
            OsStr::new("EDIT").encode_wide().chain(once(0)).collect::<Vec<_>>().as_ptr(),
            std::ptr::null_mut(),
            WS_CHILD | WS_VISIBLE | WS_BORDER | WS_VSCROLL | 0x800,
            0,
            0,
            0,
            0,
            hwnd,
            100 as HMENU,
            hinstance,
            std::ptr::null_mut(),
        );
        loop {
            let mut msg: MSG = std::mem::zeroed();
            if winapi::um::winuser::GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0) > 0 {
                winapi::um::winuser::TranslateMessage(&msg);
                winapi::um::winuser::DispatchMessageW(&msg);
            } else {
                break;
            }
        }
    }
}
