extern crate encoding;
extern crate user32;
extern crate winapi;

extern crate windows;
use windows::*;

use winapi::windef::HWND;
use winapi::windef::HBRUSH;

use winapi::minwindef::UINT;
use winapi::minwindef::WPARAM;
use winapi::minwindef::LPARAM;
use winapi::minwindef::LRESULT;

fn hide_console_window() {
  window::get_console_window()
    .and_then(|w| Some(window::show_window(&w, winapi::winuser::SW_HIDE)));
  ()
}

fn start() -> Result<()> {
  use winapi::winuser::CW_USEDEFAULT;
  use winapi::winuser::WS_OVERLAPPEDWINDOW;
  use winapi::winuser::WS_VISIBLE;

  hide_console_window();

  let hinst = try!(get_exe_module());
  let icon = try!(load_standard_icon(ResourceName::from_id(winapi::winuser::IDI_APPLICATION)));
  let cursor = try!(load_standard_cursor(ResourceName::from_id(winapi::winuser::IDI_APPLICATION)));

  let atom = try!(register_class(
    0,
    Some(wndproc),
    hinst,
    Some(icon),
    Some(cursor),
    Some(16 as HBRUSH),
    None,
    "windowClassName",
    None
  ));

  let _window = try!(window::create_window_with_atom(
    0,
    atom,
    "My Rust Window",
    WS_OVERLAPPEDWINDOW | WS_VISIBLE,
    CW_USEDEFAULT,
    CW_USEDEFAULT,
    CW_USEDEFAULT,
    CW_USEDEFAULT,
    Some(&window::get_desktop_window()),
    None,
    hinst
  ));

  run_loop();

  Ok(())
}

fn main() {
  match start() {
    Err(err) => print!("An error occured: {:?}", err),
    Ok(_) => ()
  }
}

pub unsafe extern "system"
fn wndproc(hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
  if msg == winapi::winuser::WM_DESTROY {
    user32::PostQuitMessage(0);
  }
  return user32::DefWindowProcW(hwnd, msg, wparam, lparam);
}
