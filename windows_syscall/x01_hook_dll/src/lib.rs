mod logger;

use std::ffi::c_void;
use log::debug;
use windows::Win32::Foundation::{HMODULE, LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{CallNextHookEx, HHOOK};
use crate::logger::init_logger;

const DLL_PROCESS_DETACH: usize = 0;
const DLL_PROCESS_ATTACH: usize = 1;
const DLL_THREAD_ATTACH: usize = 2;
const DLL_THREAD_DETACH: usize = 3;

/// This method gets called whenever the DLL is un/loaded into memory.
/// The `fdwReason` argument shows the state.
/// See: https://learn.microsoft.com/en-us/windows/win32/dlls/dllmain
#[no_mangle]
pub unsafe extern "stdcall" fn DllMain(_: HMODULE, fdw_reason: usize, lpv_reserved: *mut c_void) -> usize {
    if fdw_reason == DLL_PROCESS_ATTACH {
        // initialize once for each new process
        bootstrap();
    } else if fdw_reason == DLL_THREAD_ATTACH {
        // do thread-specific initialization
    } else if fdw_reason == DLL_THREAD_DETACH {
        // do thread-specific cleanup
    } else if fdw_reason == DLL_PROCESS_DETACH {
        if lpv_reserved.is_null() {
            // perform any necessary cleanup
            cleanup();
        }
        // else do not do cleanup if process termination scenario
    }

    // return 0 (FALSE) to fail DLL load
    1
}


#[no_mangle]
pub unsafe extern "C" fn trigger(code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {

    // call next middleware and return
    CallNextHookEx(HHOOK::default(), code, w_param, l_param)
}

fn bootstrap() {
    init_logger();

    debug!("the hook is ready");
}

fn cleanup() {
    // do whatever is required for cleaning up
    debug!("the hook is cleaned up");
}