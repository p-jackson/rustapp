extern crate winapi;
extern crate user32;
extern crate kernel32;

use std;
use super::Result;
use super::apihelpers::to_wstring;
use super::get_last_error;
use winapi::winnt::LPCWSTR;
use std::ptr::null_mut;

define_handle_wrapper!(Window, winapi::windef::HWND, |h| user32::DestroyWindow(h));

pub fn create_window(exstyle: u32, class_name: &str, title: &str, style: u32,
  x: i32, y: i32, width: i32, height: i32, parent: Option<&Window>,
  menu: Option<winapi::windef::HMENU>, hinst: winapi::minwindef::HINSTANCE
) -> Result<Window> {

  let wide = to_wstring(class_name);
  let class_ptr = wide.as_ptr() as LPCWSTR;

  create_window_impl(exstyle, class_ptr, title, style, x, y, width, height,
    parent, menu, hinst
  )
}

pub fn create_window_with_atom(exstyle: u32, atom: winapi::minwindef::ATOM, title: &str,
  style: u32, x: i32, y: i32, width: i32, height: i32, parent: Option<&Window>,
  menu: Option<winapi::windef::HMENU>, hinst: winapi::minwindef::HINSTANCE
) -> Result<Window> {

  create_window_impl(exstyle, atom as LPCWSTR, title, style, x, y, width, height,
    parent, menu, hinst
  )
}


fn create_window_impl(exstyle: u32, class_ptr: LPCWSTR, title: &str, style: u32,
  x: i32, y: i32, width: i32, height: i32, parent: Option<&Window>,
  menu: Option<winapi::windef::HMENU>, hinst: winapi::minwindef::HINSTANCE
) -> Result<Window> {

  unsafe {

    let hwnd = user32::CreateWindowExW(exstyle, class_ptr, to_wstring(title).as_ptr() as LPCWSTR,
      style, x, y, width, height, parent.map_or(null_mut(), |s| s.get_handle()),
      menu.unwrap_or(null_mut()), hinst, null_mut()
    );

    if hwnd == null_mut() {
      Err(get_last_error())
    }
    else {
      Ok(Window::Owned(hwnd))
    }

  }
}

pub fn get_desktop_window() -> Window {
  unsafe {
    Window::Ref(user32::GetDesktopWindow())
  }
}

pub fn get_console_window() -> Option<Window> {
  unsafe {
    let h = kernel32::GetConsoleWindow();
    if h == null_mut() {
      None
    }
    else {
      Some(Window::Ref(h))
    }
  }
}

pub fn show_window(window: &Window, show_command: i32) -> bool {
  unsafe {
    user32::ShowWindow(window.get_handle(), show_command) != 0
  }
}
