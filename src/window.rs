use ::events::{Events};
use ::view::{View, Component, Modifier};

extern crate rusttype;
use self::rusttype::{FontCollection, Scale, point, PositionedGlyph};

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
   assets: Vec<(String,Vec<u8>)>
}

impl Window {
   pub fn new(title: &str) -> Window {
      Window {
         title: title.to_owned(),
         fullscreen: false,
         assets: Vec::new()
      }
   }
   pub fn set_fullscreen(mut self, fullscreen: bool) -> Window {
      self.fullscreen = fullscreen; self
   }
   pub fn load_assets(&mut self, mut assets: Vec<(&str,Vec<u8>)>) {
      while let Some((path,contents)) = assets.pop() {
         self.assets.push((path.to_string(), contents));
      }
   }
   pub fn start<F>(&self, cl: F) 
       where F: Fn(&mut Events) -> View {

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
      let mut fonts = HashMap::new();
      for ai in 0..self.assets.len() {
         let (ref name,ref buf) = self.assets[ai];
         let ns = name.to_string();
         if ns.ends_with(".png") {
            let png = image::load_from_memory_with_format(buf, image::ImageFormat::PNG).expect("Couldn't load image");
            let (dx,dy) = png.dimensions();
            let mut texture = texture_creator.create_texture_streaming(PixelFormatEnum::RGBA8888, dx, dy).unwrap();
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
            }).expect("texture with_lock");
            textures.insert(name.as_str(), (dx, dy, texture));
         }
         else if ns.ends_with(".ttf") {
            let font = FontCollection::from_bytes(buf as &[u8]).into_font().expect("single ttf font file");
            fonts.insert(name.as_str(), font);
         } else {
            panic!("Unrecognized asset file format: {}", ns)
         }
      }

      let mut tick = 0;
      let mut event_pump = sdl_context.event_pump().unwrap();

      let mut events = Events::new();

      'running: loop {
         for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                _ => {}
            }
         }

         let (width_px, height_px) = {
            let mut window = canvas.window_mut();
            window.drawable_size()
         };

         let v = cl(&mut events);
         canvas.clear();

         for ci in 0..v.components.len() {
            let ref c = v.components[ci];
            match *c {
               Component::Image(ref image) => {
                  let mut x = 0;
                  let mut y = 0;
                  let mut w = -1i64;
                  let mut h = -1i64;

                  for mi in 0..image.modifiers().len() {
                     let ref m = image.modifiers()[mi];
                     match *m {
                        Modifier::SizeWidthDynamic(ref wd) => { 
                           match wd.unit.as_str() {
                              "%" => { w = ((width_px as f64) * wd.scalar / 100.0) as i64; }
                              u => { panic!("Invalid unit: {}", u); }
                           }
                        }
                        Modifier::SizeHeightDynamic(ref hd) => { 
                           match hd.unit.as_str() {
                              "%" => { h = ((height_px as f64) * hd.scalar / 100.0) as i64; }
                              u => { panic!("Invalid unit: {}", u); }
                           }
                        }
                        _ => {}
                     }
                  }

                  let (tx, ty, ref texture) = *textures.get(image.name.as_str())
                                              .expect(format!("no texture named: {}", image.name).as_str());
                  if w<0 { w=(tx as i64) };
                  if h<0 { h=(ty as i64) };
                  canvas.copy(texture, None, Some(Rect::new(x, y, (w as u32), (h as u32)))).unwrap();
               }
               Component::Text(ref text) => {
                  let font = fonts.get(text.font.as_str()).expect(format!("Could not find font: {}", text.font).as_str());

                  let height: f32 = 12.4;
                  let pixel_height = height.ceil() as usize;
                  let scale = Scale { x: height*2.0, y: height };

                  let v_metrics = font.v_metrics(scale);
                  let offset = point(0.0, v_metrics.ascent);

                  let glyphs: Vec<PositionedGlyph> = font.layout(text.content.as_str(), scale, offset).collect();
                  println!("loaded glyphs for rendering");
               }
               _ => {}
            }
         }
         canvas.present();
      }
   }
}
