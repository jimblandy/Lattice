use ::events::{Events};
use ::view::{View, Component, Modifier};

extern crate rusttype;
use self::rusttype::{FontCollection, Scale, point, PositionedGlyph};

extern crate unicode_normalization;

extern crate sdl2;
use self::sdl2::pixels::Color;
use self::sdl2::event::Event;
use self::sdl2::keyboard::Keycode;
use self::sdl2::pixels::PixelFormatEnum;
use self::sdl2::rect::Rect;
use self::sdl2::render::{Texture, BlendMode};

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
      canvas.set_blend_mode(BlendMode::Blend);
      let texture_creator = canvas.texture_creator();

      let mut textures = HashMap::new();
      let mut fonts = HashMap::new();
      let mut glyphs: HashMap<(char,usize),(usize,Texture)> = HashMap::new();
      for ai in 0..self.assets.len() {
         let (ref name,ref buf) = self.assets[ai];
         let ns = name.to_string();
         if ns.ends_with(".png") {
            let png = image::load_from_memory_with_format(buf, image::ImageFormat::PNG).expect("Couldn't load image");
            let (dx,dy) = png.dimensions();
            let mut texture = texture_creator.create_texture_streaming(PixelFormatEnum::RGBA8888, dx, dy).unwrap();
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
         canvas.set_draw_color(Color::RGB(0, 0, 0));
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

                  let height: f32 = 100.4;
                  let pixel_height = height.ceil() as usize;
                  let scale = Scale { x: height*2.0, y: height };

                  let v_metrics = font.v_metrics(scale);
                  let offset = point(0.0, v_metrics.ascent);

                  for c in text.content.as_str().chars() {
                     if !glyphs.contains_key(&(c, pixel_height)) {
                        let ctxt = format!("{}", c);
                        let gl: Vec<PositionedGlyph> = font.layout(ctxt.as_str(), scale, offset).collect();
                        let width = gl.iter().rev()
                                   .filter_map(|g| g.pixel_bounding_box()
                                   .map(|b| b.min.x as f32 + g.unpositioned().h_metrics().advance_width))
                                   .next().unwrap_or(height * 0.7).ceil() as usize;
                        let mut rasterized_glyph = vec![0u32; width * pixel_height];
                        for g in gl {
                           if let Some(bb) = g.pixel_bounding_box() {
                              g.draw(|x, y, v| {
                                 let w = (v * 255.0) as u32;
                                 let x = x as i32 + bb.min.x;
                                 let y = y as i32 + bb.min.y;
                                 let width = width as i32;
                                 if x >= 0 && x < width as i32 && y >= 0 && y < pixel_height as i32 {
                                    rasterized_glyph[(x + y * width) as usize] = w;
                                 }
                              })
                           }
                        }

                        let mut texture = texture_creator.create_texture_streaming(PixelFormatEnum::RGBA8888, width as u32, pixel_height as u32)
                                           .expect("Expect glyph texture");
                        texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
                           for x in 0..width {
                              for y in 0..pixel_height {
                                 let pitch = pitch;
                                 let offset = (y*pitch + 4*x) as usize;
                                 buffer[offset+0] = 255 as u8;
                                 buffer[offset+1] = 255 as u8;
                                 buffer[offset+2] = 255 as u8;
                                 buffer[offset+3] = rasterized_glyph[(x + y * width) as usize] as u8;
                              }
                           }
                        }).expect("texture with_lock");
                        texture.set_blend_mode(BlendMode::Blend);
                        glyphs.insert((c, pixel_height as usize), (width as usize, texture));
                     };
                  }

                  let width = 99999 as usize;
                  let line_height = pixel_height as usize;
                  let justify = false;
                  let positioned = {
                     use self::unicode_normalization::UnicodeNormalization;
                     let mut result = Vec::new();
                     let mut caret = 0;
                     let mut height = 0;
                     for c in text.content.as_str().nfc() {
                        if c.is_control() {
                            match c {
                               '\r' => { caret = 0; height += pixel_height; }
                               '\n' => {},
                               _ => {}
                            }
                            continue;
                        }
                        let (glyph_width, ref base_glyph) = match glyphs.get(&(c,line_height)) {
                           Some(c) => {
                              let (w, ref g) = *c;
                              (w, g)
                           }
                           _ => { continue; }
                        };
                        if caret + glyph_width > width {
                           caret = 0; height += line_height;
                        }
                        result.push( (caret, height, c, line_height) );
                        caret += glyph_width;
                     }
                     if justify {
                        panic!("TODO: implement justify");
                     }
                     result
                  };

                  for pi in 0..positioned.len() {
                     let (caret, height, c, line_height) = positioned[pi];
                     let (glyph_width, ref base_glyph) = match glyphs.get(&(c,line_height)) {
                        Some(c) => {
                           let (w, ref g) = *c;
                           (w, g)
                        }
                        _ => { continue; }
                     };
                     let x = caret;
                     let y = height;
                     canvas.copy(base_glyph, None, Some(Rect::new((x as i32), (y as i32), (glyph_width as u32), (line_height as u32)))).unwrap();
                  }
               }
               _ => {}
            }
         }
         canvas.present();
      }
   }
}
