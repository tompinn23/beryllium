use beryllium::*;

fn main() {
  let sdl = SDL::init(InitFlags::Everything).expect("couldn't init SDL");
  let _win = sdl
    .create_gl_window(
      "Basic Window",
      WindowPosition::default(),
      800,
      600,
      WindowFlags::Shown,
    )
    .expect("couldn't open a window");

  loop {
    match sdl.poll_events().and_then(Result::ok) {
      Some(Event::Quit(QuitEvent { .. })) => break,
      _ => continue,
    }
  }
}