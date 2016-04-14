extern crate windows as win;
extern crate user32;
extern crate uuid;

use winapi;
use self::win::window;
use std::result::Result;

pub struct FrameOptions<'a> {
  title: &'a str,
  x: i32,
  y: i32,
  width: i32,
  height: i32
}

impl<'a> FrameOptions<'a> {
  pub fn new() -> FrameOptions<'a> {
    use winapi::winuser::CW_USEDEFAULT;

    FrameOptions {
      title: "",
      x: CW_USEDEFAULT,
      y: CW_USEDEFAULT,
      width: CW_USEDEFAULT,
      height: CW_USEDEFAULT
    }
  }
}

pub struct Frame {
  window: window::Window
}

impl Frame {
  pub fn new(options: &FrameOptions) -> Result<Frame, win::WindowsError> {
    use winapi::winuser::WS_OVERLAPPEDWINDOW;
    use winapi::winuser::WS_VISIBLE;

    let frame_atom = try!(create_frame_class());
    let hinst = try!(win::get_exe_module());

    let window = try!(window::create_window_with_atom(0, frame_atom, options.title,
      WS_VISIBLE | WS_OVERLAPPEDWINDOW, options.x, options.y, options.width,
      options.height, Some(&win::window::get_desktop_window()), None, hinst
    ));

    Ok(Frame { window: window })
  }
}

fn create_frame_class() -> Result<winapi::minwindef::ATOM, win::WindowsError> {
  use winapi::winuser::IDI_APPLICATION;

  let icon = try!(win::load_standard_icon(win::ResourceName::from_id(IDI_APPLICATION)));
  let cursor = try!(win::load_standard_cursor(win::ResourceName::from_id(IDI_APPLICATION)));

  // Uuid::new returns None if the version hasn't been implemented. So unwrap should be safe.
  let unique = uuid::Uuid::new(uuid::UuidVersion::Random).unwrap();

  win::register_class(0, Some(frame_proc), try!(win::get_exe_module()), Some(icon), Some(cursor),
    None, None, &unique.simple().to_string(), None
  )
}

use winapi::windef::HWND;
use winapi::minwindef::UINT;
use winapi::minwindef::WPARAM;
use winapi::minwindef::LPARAM;
use winapi::minwindef::LRESULT;

unsafe extern "system"
fn frame_proc(hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
  if msg == winapi::winuser::WM_DESTROY {
    user32::PostQuitMessage(0);
  }
  return user32::DefWindowProcW(hwnd, msg, wparam, lparam);
}
