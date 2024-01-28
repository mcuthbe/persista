use winapi::um::winuser::GetForegroundWindow;

pub fn get_foreground_window_handle() -> winapi::shared::windef::HWND {
    unsafe { GetForegroundWindow() }
}
