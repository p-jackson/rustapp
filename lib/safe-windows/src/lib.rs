extern crate kernel32;
extern crate user32;
extern crate winapi;
extern crate encoding;

#[macro_use]
mod macros;

use std::ptr::null_mut;
use winapi::winnt::LPCWSTR;

mod error;
pub use self::error::*;

pub mod window;

mod apihelpers;
pub use self::apihelpers::ResourceName;

pub fn get_exe_module() -> Result<winapi::minwindef::HMODULE> {
  unsafe {
    handle_null!(kernel32::GetModuleHandleW(null_mut()))
  }
}

pub fn register_class(
  style: u32,
  window_proc: winapi::winuser::WNDPROC,
  hinst: winapi::minwindef::HINSTANCE,
  icon: Option<winapi::windef::HICON>,
  cursor: Option<winapi::windef::HCURSOR>,
  background: Option<winapi::windef::HBRUSH>,
  menu: Option<&str>,
  class_name: &str,
  small_icon: Option<winapi::windef::HICON>
) -> Result<winapi::minwindef::ATOM> {

  let menu_ptr = match menu {
    Some(s) => apihelpers::to_wstring(s).as_ptr() as LPCWSTR,
    None => 0 as LPCWSTR
  };

  unsafe {

    let wnd = winapi::winuser::WNDCLASSEXW {
      cbSize: std::mem::size_of::<winapi::winuser::WNDCLASSEXW>() as u32,
      style: style,
      lpfnWndProc: window_proc,
      cbClsExtra: 0,
      cbWndExtra: 0,
      hInstance: hinst,
      hIcon: icon.unwrap_or(null_mut()),
      hCursor: cursor.unwrap_or(null_mut()),
      hbrBackground: background.unwrap_or(null_mut()),
      lpszMenuName: menu_ptr,
      lpszClassName: apihelpers::to_wstring(class_name).as_ptr() as LPCWSTR,
      hIconSm: small_icon.unwrap_or(null_mut())
    };

    let atom = user32::RegisterClassExW(&wnd);

    if atom == 0 {
      Err(get_last_error())
    }
    else {
      Ok(atom)
    }
  }
}

pub fn load_standard_icon(name: ResourceName) -> Result<winapi::windef::HICON> {
  unsafe {
    handle_null!(user32::LoadIconW(null_mut(), name.as_lpcwstr()))
  }
}

pub fn load_standard_cursor(name: ResourceName) -> Result<winapi::windef::HCURSOR> {
  unsafe {
    handle_null!(user32::LoadCursorW(null_mut(), name.as_lpcwstr()))
  }
}

pub fn run_loop() {
  use winapi::windef::HWND;

  unsafe {

    let mut msg = winapi::winuser::MSG {
      hwnd: 0 as HWND,
      message: 0,
      wParam: 0,
      lParam: 0,
      time: 0,
      pt: winapi::windef::POINT { x: 0, y: 0, },
    };

    loop {
      let pm = user32::PeekMessageW(&mut msg, 0 as HWND, 0, 0, winapi::winuser::PM_REMOVE);
      if pm == 0 {
        continue;
      }

      if msg.message == winapi::winuser::WM_QUIT {
        break;
      }

      user32::TranslateMessage(&mut msg);
      user32::DispatchMessageW(&mut msg);
    }

  }
}
