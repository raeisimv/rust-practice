use std::sync::{Arc, Mutex};
use windows::Win32::Foundation::{BOOL, HWND, LPARAM};
use windows::Win32::UI::WindowsAndMessaging::{
    EnumChildWindows, GetDesktopWindow, RealGetWindowClassA,
};

type GetTargetType = Arc<Mutex<Option<HWND>>>;
const TARGET_APP_TITLE: &str = "TaskManagerWindow";

fn main() {
    let Some(app_hwn) = find_app_hwnd() else {
        eprintln!("target app handle is not found. make sure the app is running");
        return;
    };

    assert_ne!(app_hwn, HWND::default(), "app handle must not be empty");
    println!("handle for '{}' is {:?}", TARGET_APP_TITLE, app_hwn);
}

fn find_app_hwnd() -> Option<HWND> {
    let storage: GetTargetType = Arc::new(Mutex::new(None));
    let l_param = LPARAM(&storage as *const GetTargetType as isize);

    unsafe {
        let desktop_hwnd = GetDesktopWindow();
        let _ = EnumChildWindows(desktop_hwnd, Some(find_target_process), l_param);
    }

    let state = storage.lock().unwrap();
    *state
}

unsafe extern "system" fn find_target_process(hwnd: HWND, l_param: LPARAM) -> BOOL {
    let mut buffer = [0_u8; 128];
    let read_len = RealGetWindowClassA(hwnd, &mut buffer);
    let proc_name = String::from_utf8_lossy(&buffer[..read_len as usize]);

    if proc_name != TARGET_APP_TITLE {
        return BOOL(1);
    }

    let storage = &*(l_param.0 as *const GetTargetType);
    let mut storage = storage.lock().unwrap();
    (*storage).replace(hwnd);

    BOOL(1)
}
