extern crate algum;
extern crate winapi;
extern crate windows;

use algum::frame;

fn hide_console_window() {
  use windows::window;
  window::get_console_window()
    .and_then(|w| Some(window::show_window(&w, winapi::winuser::SW_HIDE)));
  ()
}

fn start() -> windows::Result<()> {
  hide_console_window();

  let builder = frame::FrameBuilder::new()
    .title("My Rust Window");

  let _window = try!(builder.create());

  algum::run_loop();

  Ok(())
}

fn main() {
  match start() {
    Err(err) => print!("An error occured: {:?}", err),
    Ok(_) => ()
  }
}
