use ::events::{Events};
use ::view::{View};

extern crate sdl2;
use self::sdl2::pixels::Color;
use self::sdl2::event::Event;
use self::sdl2::keyboard::Keycode;

pub struct Window {
   title: String,
   fullscreen: bool
}

impl Window {
   pub fn new() -> Window {
      Window {
         title: "Lattice Window".to_string(),
         fullscreen: false
      }
   }
   pub fn set_title(&mut self, title: String) -> &mut Window {
      self.title = title; self
   }
   pub fn set_fullscreen(&mut self, fullscreen: bool) -> &mut Window {
      self.fullscreen = fullscreen; self
   }
   pub fn start<F>(&self, cl: F) 
       where F: Fn(Events) -> View {

      let sdl_context = sdl2::init().unwrap();
      let video_subsystem = sdl_context.video().unwrap();

      let ref mut window = video_subsystem
         .window(self.title.as_str(), 800, 600);
      let ref mut window = if self.fullscreen { window.fullscreen().maximized() } else { window };
      let window = window.build().unwrap();

      let mut canvas = window.into_canvas().present_vsync().build().unwrap();

      let mut tick = 0;
      let mut event_pump = sdl_context.event_pump().unwrap();

      'running: loop {
         for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                _ => {}
            }
         }

         {
            let mut window = canvas.window_mut();
            let position = window.position();
            let size = window.size();
            tick += 1;
         }

         canvas.set_draw_color(Color::RGB(0, 0, 0));
         canvas.clear();
         canvas.present();
      }
   }
}
