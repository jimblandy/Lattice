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

use std::collections::{HashMap};
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

      let ref mut window = video_subsystem.window(self.title.as_str(), 800, 600);
      let ref mut window = window.resizable();
      let ref mut window = window.maximized();
      let ref mut window = window.input_grabbed();
      let ref mut window = window.allow_highdpi();
      let ref mut window = if self.fullscreen { window.fullscreen_desktop() } else { window };
      let window = window.build().unwrap();

      let mut canvas = window.into_canvas().present_vsync().build().unwrap();
      let texture_creator = canvas.texture_creator();

      let mut textures = HashMap::new();
      if self.assets {
         let mut texture = texture_creator.create_texture_streaming(PixelFormatEnum::RGBA8888, 256, 256).unwrap();
         let buf = include_bytes!("assets/startscreen.png");
         let png = image::load_from_memory(buf).expect("Couldn't load image");
         let png = png.as_rgba8().expect("cast to rgba8");
         let (dx,dy) = png.dimensions();
         texture = texture_creator.create_texture_streaming(PixelFormatEnum::RGBA8888, dx, dy).unwrap();
         texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
            for x in 0..dx {
               for y in 0..dy {
                  let pitch = pitch as u32;
                  let offset = (y*pitch + 4*x) as usize;
                  let p = png.get_pixel(x, y);
                  buffer[offset+0] = p.data[3] as u8;
                  buffer[offset+1] = p.data[2] as u8;
                  buffer[offset+2] = p.data[1] as u8;
                  buffer[offset+3] = p.data[0] as u8;
               }
            }
         }).unwrap();
         textures.insert("startscreen.png", texture);
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

         let (w,h) = {
            let mut window = canvas.window_mut();
            window.drawable_size()
         };

         canvas.clear();
         let ref texture = textures.get("startscreen.png").expect("texture startscreen.png");
         canvas.copy(&texture, None, Some(Rect::new(0, 0, w, h))).unwrap();
         canvas.present();
      }
   }
}
