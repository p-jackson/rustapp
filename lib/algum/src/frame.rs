extern crate windows as win;
extern crate user32;
extern crate snowflake;

use winapi;
use self::win::window;
use std::result::Result;
use std::fmt;

pub struct FrameBuilder<'a> {
  title: &'a str,
  x: i32,
  y: i32,
  width: i32,
  height: i32
}

impl<'a> FrameBuilder<'a> {
  pub fn new() -> Self {
    use winapi::winuser::CW_USEDEFAULT;

    FrameBuilder {
      title: "",
      x: CW_USEDEFAULT,
      y: CW_USEDEFAULT,
      width: CW_USEDEFAULT,
      height: CW_USEDEFAULT
    }
  }

  pub fn title(self, title: &'a str) -> Self {
    FrameBuilder { title: title, .. self }
  }

  pub fn x(self, x: i32) -> Self {
    FrameBuilder { x: x, .. self }
  }

  pub fn y(self, y: i32) -> Self {
    FrameBuilder { y: y, .. self }
  }

  pub fn width(self, width: i32) -> Self {
    FrameBuilder { width: width, .. self }
  }

  pub fn height(self, height: i32) -> Self {
    FrameBuilder { height: height, .. self }
  }

  pub fn create(self) -> Result<Frame, win::WindowsError> {
    use winapi::winuser::WS_OVERLAPPEDWINDOW;
    use winapi::winuser::WS_VISIBLE;

    let frame_atom = try!(create_frame_class());
    let hinst = try!(win::get_exe_module());

    let window = try!(window::create_window_with_atom(0, frame_atom, self.title,
      WS_VISIBLE | WS_OVERLAPPEDWINDOW, self.x, self.y, self.width, self.height,
      Some(&win::window::get_desktop_window()), None, hinst
    ));

    Ok(Frame { window: window })
  }
}

pub struct Frame {
  window: window::Window
}

impl Frame {
  pub fn new(title: &str) -> Result<Frame, win::WindowsError> {
    FrameBuilder::new().title(title).create()
  }
}

fn create_frame_class() -> Result<winapi::minwindef::ATOM, win::WindowsError> {
  use winapi::winuser::IDI_APPLICATION;

  let icon = try!(win::load_standard_icon(win::ResourceName::from_id(IDI_APPLICATION)));
  let cursor = try!(win::load_standard_cursor(win::ResourceName::from_id(IDI_APPLICATION)));

  let class_name = fmt::format(format_args!("{:?}", snowflake::ProcessUniqueId::new()));

  win::register_class(0, Some(frame_proc), try!(win::get_exe_module()), Some(icon), Some(cursor),
    None, None, &class_name, None
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
