use ::events::{Events};
use ::view::{View};

extern crate sdl2;
use self::sdl2::pixels::Color;
use self::sdl2::event::Event;
use self::sdl2::keyboard::Keycode;
use self::sdl2::pixels::PixelFormatEnum;
use self::sdl2::rect::Rect;

extern crate image;
use self::image::*;

use std::path::Path;
use std::fs::File;
use std::io::{Write, Read, Seek, SeekFrom};

pub struct Window {
   title: String,
   fullscreen: bool,
   assets: bool
}

impl Window {
   pub fn new() -> Window {
      Window {
         title: "Lattice Window".to_string(),
         fullscreen: false,
         assets: false
      }
   }
   pub fn set_title(&mut self, title: String) -> &mut Window {
      self.title = title; self
   }
   pub fn set_fullscreen(&mut self, fullscreen: bool) -> &mut Window {
      self.fullscreen = fullscreen; self
   }
   pub fn with_assets(&mut self) -> &mut Window {
      self.assets = true; self
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
      let texture_creator = canvas.texture_creator();

      let mut texture = texture_creator.create_texture_streaming(PixelFormatEnum::RGBA8888, 256, 256).unwrap();
      if self.assets {
         let buf = include_bytes!("assets/startscreen.png");
         let png = image::load_from_memory_with_format(buf, image::ImageFormat::PNG).expect("Couldn't load image");
         let (dx,dy) = png.dimensions();
         texture = texture_creator.create_texture_streaming(PixelFormatEnum::RGBA8888, dx, dy).unwrap();
         texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
            for y in 0..dy {
               for x in 0..dx {
                  let offset = (y*(pitch as u32) + x*3) as usize;
                  let p = png.get_pixel(x, y);
                  buffer[offset] = p.data[0] as u8;
                  buffer[offset + 1] = p.data[1] as u8;
                  buffer[offset + 2] = p.data[2] as u8;
                  buffer[offset + 3] = p.data[3] as u8;
               }
            }
         }).unwrap();
      }

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

         canvas.clear();
         canvas.copy(&texture, None, Some(Rect::new(100, 100, 256, 256))).unwrap();
         canvas.copy_ex(&texture, None,
            Some(Rect::new(450, 100, 256, 256)), 30.0, None, false, false).unwrap();
         canvas.present();
      }
   }
}
