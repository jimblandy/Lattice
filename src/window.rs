use ::events::{Events};
use ::view::{View, Component, Modifier};

extern crate rusttype;
use self::rusttype::{FontCollection, Scale, point, PositionedGlyph};

extern crate unicode_normalization;

extern crate sdl2;
use self::sdl2::pixels::Color;
use self::sdl2::event::Event;
use self::sdl2::mouse::MouseButton;
use self::sdl2::keyboard::Keycode;
use self::sdl2::pixels::PixelFormatEnum;
use self::sdl2::rect::Rect;
use self::sdl2::render::{Texture, BlendMode};

extern crate image;
use self::image::*;
use std::collections::{HashMap};

///A configurable window
pub struct Window {
   title: String,
   fullscreen: bool,
   assets: Vec<(String,Vec<u8>)>,
}

impl Window {
   ///Creates a new configurable window
   pub fn new(title: &str) -> Window {
      Window {
         title: title.to_owned(),
         fullscreen: false,
         assets: Vec::new(),
      }
   }
   ///Changes the fullscreen mode of the Window
   pub fn set_fullscreen(mut self, fullscreen: bool) -> Window {
      self.fullscreen = fullscreen; self
   }
   ///Loads assets. Is called in the with_assets! macro.
   pub fn load_assets(&mut self, mut assets: Vec<(&str,Vec<u8>)>) {
      while let Some((path,contents)) = assets.pop() {
         self.assets.push((path.to_string(), contents));
      }
   }
   ///Opens the window and begins the render cycle
   pub fn start<F>(&self, mut cl: F) 
       where F: FnMut(&mut Events) -> View {

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
      let em = 22.0f64;
      for ai in 0..self.assets.len() {
         let (ref name,ref buf) = self.assets[ai];
         let ns = name.to_string();
         if ns.ends_with(".png") {
            let png = image::load_from_memory_with_format(buf, image::ImageFormat::PNG).expect("Couldn't load image");
            let (dx,dy) = png.dimensions();
            let mut texture = texture_creator.create_texture_streaming(PixelFormatEnum::RGBA8888, dx, dy).unwrap();
            texture.set_blend_mode(BlendMode::Blend);
            texture.set_alpha_mod(255);
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

      let mut event_pump = sdl_context.event_pump().unwrap();
      let mut events = Events::new();

      'running: loop {
         let mut click = false;
         for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                Event::MouseButtonDown { mouse_btn: MouseButton::Left, .. } => { click = true; }
                _ => {}
            }
         }

         let (width_px, height_px, dpi) = {
            let window = canvas.window_mut();
            let (rw,_) = window.size();
            let (w,h) = window.drawable_size();
            let dpi = w/rw;
            (w as usize, h as usize, dpi as usize)
         };
         let width_pct = (width_px as f64) / 100.0;
         let height_pct = (height_px as f64) / 100.0;

         let cursor = event_pump.mouse_state();
         let cursor_x = (cursor.x() as usize) * (dpi as usize);
         let cursor_y = (cursor.y() as usize) * (dpi as usize);

         let mut v = cl(&mut events);
         canvas.set_draw_color(Color::RGB(0, 0, 0));
         canvas.clear();

         for ci in 0..v.components.len() {
            let ref mut c = v.components[ci];

            let bbox: (usize,usize,usize,usize) = {

               let mut pixel_height = em as usize;
               let mut width = width_px as usize;
               let mut height = height_px as usize;
               let mut pos_x = 0 as usize;
               let mut pos_y = 0 as usize;
               let mut cog: (f64,f64) = (0.0, 0.0);
               let mut color = [1.0, 1.0, 1.0, 1.0];
               let mut shadow = ([0,0,0,0],[0.0,0.0,0.0,0.0]);
               let mut border_width = 0;
               let mut border_color = [0.0, 0.0, 0.0, 0.0];

               for m in c.modifiers() {
                  match *m {
                     Modifier::State(ref s) => {
                        events.state = s.state.clone();
                     }
                     Modifier::Shadow(ref s) => {
                        shadow = (s.boxed.clone(), s.rgba.clone());
                     }
                     Modifier::Color(ref s) => {
                        color = s.rgba.clone();
                     }
                     Modifier::Border(ref s) => {
                        border_width = match s.unit.as_str() {
                          "em" => { (em * s.scalar).ceil() as usize }
                          "w%" => { (width_pct * s.scalar).ceil() as usize }
                          "h%" => { (height_pct * s.scalar).ceil() as usize }
                          ">%" => { let pct = if height_pct>width_pct { height_pct } else { width_pct }; (pct * s.scalar).ceil() as usize }
                          "<%" => { let pct = if height_pct<width_pct { height_pct } else { width_pct }; (pct * s.scalar).ceil() as usize }
                          "px" => { (s.scalar) as usize }
                           u => { panic!("Invalid unit: {}", u) }
                        };
                        border_color = s.rgba.clone();
                     }
                     Modifier::Scale(ref s) => {
                        match s.unit.as_str() {
                          "em" => { pixel_height = (em * s.scalar).ceil() as usize; }
                          "%" => { pixel_height = (height_pct * s.scalar).ceil() as usize; }
                          "w%" => { pixel_height = (width_pct * s.scalar).ceil() as usize; }
                          "h%" => { pixel_height = (height_pct * s.scalar).ceil() as usize; }
                          ">%" => { let pct = if height_pct>width_pct { height_pct } else { width_pct }; pixel_height = (pct * s.scalar).ceil() as usize; }
                          "<%" => { let pct = if height_pct<width_pct { height_pct } else { width_pct }; pixel_height = (pct * s.scalar).ceil() as usize; }
                          "px" => { pixel_height = (s.scalar) as usize; }
                           u => { panic!("Invalid unit: {}", u); }
                        }
                     }
                     Modifier::Width(ref s) => {
                        match s.unit.as_str() {
                          "em" => { width = (em * s.scalar).ceil() as usize; }
                          "%" => { width = (width_pct * s.scalar).ceil() as usize; }
                          "w%" => { width = (width_pct * s.scalar).ceil() as usize; }
                          "h%" => { width = (height_pct * s.scalar).ceil() as usize; }
                          ">%" => { let pct = if height_pct>width_pct { height_pct } else { width_pct }; width = (pct * s.scalar).ceil() as usize; }
                          "<%" => { let pct = if height_pct<width_pct { height_pct } else { width_pct }; width = (pct * s.scalar).ceil() as usize; }
                          "px" => { width = (s.scalar) as usize; }
                           u => { panic!("Invalid unit: {}", u); }
                        }
                     }
                     Modifier::Height(ref s) => {
                        match s.unit.as_str() {
                          "em" => { height = (em * s.scalar).ceil() as usize; }
                          "%" => { height = (height_pct * s.scalar).ceil() as usize; }
                          "w%" => { height = (width_pct * s.scalar).ceil() as usize; }
                          "h%" => { height = (height_pct * s.scalar).ceil() as usize; }
                          ">%" => { let pct = if height_pct>width_pct { height_pct } else { width_pct }; height = (pct * s.scalar).ceil() as usize; }
                          "<%" => { let pct = if height_pct<width_pct { height_pct } else { width_pct }; height = (pct * s.scalar).ceil() as usize; }
                          "px" => { height = (s.scalar) as usize; }
                           u => { panic!("Invalid unit: {}", u); }
                        }
                     }
                     Modifier::CenterOfGravity(ref s) => {
                        cog = (s.horizontal, s.vertical);
                     }
                     Modifier::TranslateX(ref s) => {
                        match s.unit.as_str() {
                          "em" => { pos_x = (em * s.scalar).ceil() as usize; }
                          "%" => { pos_x = (width_pct * s.scalar).ceil() as usize; }
                          "w%" => { pos_x = (width_pct * s.scalar).ceil() as usize; }
                          "h%" => { pos_x = (height_pct * s.scalar).ceil() as usize; }
                          ">%" => { let pct = if height_pct>width_pct { height_pct } else { width_pct }; pos_x = (pct * s.scalar).ceil() as usize; }
                          "<%" => { let pct = if height_pct<width_pct { height_pct } else { width_pct }; pos_x = (pct * s.scalar).ceil() as usize; }
                          "=" => { }
                          "px" => { pos_x = (s.scalar) as usize; }
                          u => { panic!("Invalid unit: {}", u); }
                        }
                     }
                     Modifier::TranslateY(ref s) => {
                        match s.unit.as_str() {
                          "em" => { pos_y = (em * s.scalar).ceil() as usize; }
                          "%" => { pos_y = (height_pct * s.scalar).ceil() as usize; }
                          "w%" => { pos_y = (width_pct * s.scalar).ceil() as usize; }
                          "h%" => { pos_y = (height_pct * s.scalar).ceil() as usize; }
                          ">%" => { let pct = if height_pct>width_pct { height_pct } else { width_pct }; pos_y = (pct * s.scalar).ceil() as usize; }
                          "<%" => { let pct = if height_pct<width_pct { height_pct } else { width_pct }; pos_y = (pct * s.scalar).ceil() as usize; }
                          "=" => { }
                          "px" => { pos_y = (s.scalar) as usize; }
                          u => { panic!("Invalid unit: {}", u); }
                        }
                     }
                  }
               }
               for m in c.modifiers() {
                  match *m {
                     Modifier::TranslateX(ref t) => {
                        match t.unit.as_str() {
                          "=" => { pos_x = (((width_px - width) as f64)*0.5).ceil() as usize; }
                          _ => { }
                        }
                     }
                     Modifier::TranslateY(ref t) => {
                        match t.unit.as_str() {
                          "=" => { pos_y = (((height_px - height) as f64)*0.5).ceil() as usize; }
                          _ => { }
                        }
                     }
                     _ => {}
                  }
               }

            pos_x -= (cog.0 * (width as f64)).ceil() as usize;
            pos_y -= (cog.1 * (height as f64)).ceil() as usize;

            if border_width > 0 {
               let clr = Color::RGBA((border_color[0]*255.0) as u8,
                                     (border_color[1]*255.0) as u8,
                                     (border_color[2]*255.0) as u8,
                                     (border_color[3]*255.0) as u8);

               canvas.set_draw_color(clr);
               canvas.fill_rect(Rect::new((pos_x-border_width) as i32, (pos_y-border_width) as i32,
                                          (width+2*border_width) as u32, (height+2*border_width) as u32)).ok();
            }

            match *c {
               Component::Rectangle(_) => {
                  let clr = Color::RGBA((color[0]*255.0) as u8,
                                        (color[1]*255.0) as u8,
                                        (color[2]*255.0) as u8,
                                        (color[3]*255.0) as u8);


                  canvas.set_draw_color(clr);
                  canvas.fill_rect(Rect::new(pos_x as i32, pos_y as i32, width as u32, height as u32)).ok();

                  (pos_x, pos_y, pos_x+width, pos_y+height)
               }
               Component::Image(ref image) => {
                  let (_, _, ref texture) = *textures.get(image.name.as_str())
                                              .expect(format!("no texture named: {}", image.name).as_str());
                  canvas.copy(texture, None, Some(Rect::new(pos_x as i32, pos_y as i32, width as u32, height as u32))).unwrap();

                  (pos_x, pos_y, pos_x+width, pos_y+height)
               }
               Component::Text(ref mut text) => {
                  let font = fonts.get(text.font.as_str()).expect(format!("Could not find font: {}", text.font).as_str());

                  let scale = Scale { x: (pixel_height) as f32, y: (pixel_height as f32) };

                  let v_metrics = font.v_metrics(scale);
                  let offset = point(0.0, v_metrics.ascent);

                  for c in text.content.as_str().chars() {
                     if !glyphs.contains_key(&(c, pixel_height)) {
                        let ctxt = format!("{}", c);
                        let gl: Vec<PositionedGlyph> = font.layout(ctxt.as_str(), scale, offset).collect();
                        let width = gl.iter().rev()
                                   .filter_map(|g| g.pixel_bounding_box()
                                   .map(|b| b.min.x as f32 + g.unpositioned().h_metrics().advance_width))
                                   .next().unwrap_or((em as f32) * 2.0).ceil() as usize;
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
                        texture.set_blend_mode(BlendMode::Blend);
                        texture.set_alpha_mod(255);
                        texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
                           for x in 0..width {
                              for y in 0..pixel_height {
                                 let pitch = pitch;
                                 let offset = (y*pitch + 4*x) as usize;
                                 buffer[offset+0] = rasterized_glyph[(x + y * width) as usize] as u8;
                                 buffer[offset+1] = 255 as u8;
                                 buffer[offset+2] = 255 as u8;
                                 buffer[offset+3] = 255 as u8;
                              }
                           }
                        }).expect("texture with_lock");
                        glyphs.insert((c, pixel_height as usize), (width as usize, texture));
                     };
                  }

                  let line_height = pixel_height as usize;
                  let positioned = {
                     use self::unicode_normalization::UnicodeNormalization;
                     let mut result = Vec::new();
                     let mut caret = 0;
                     let mut height = 0;
                     for c in text.content.as_str().nfc() {
                        if c.is_control() {
                            match c {
                               '\r' => { caret = 0; height += line_height; }
                               '\n' => { caret = 0; height += line_height; },
                               _ => {}
                            }
                            continue;
                        }
                        let (glyph_width, _) = match glyphs.get(&(c,line_height)) {
                           Some(c) => {
                              let (w, ref g) = *c;
                              (w, g)
                           }
                           _ => { panic!("Glyph not loaded: {}", c); }
                        };
                        if caret + glyph_width > width {
                           caret = 0; height += line_height;
                        }
                        result.push( (caret, height, c, line_height, glyph_width) );
                        caret += glyph_width;
                     }
                     if text.align.as_str() == "justify" {
                        let just_width = width;
                        let mut prev_line = 0;
                        for ri in 0..(result.len()+1) {
                           if ri==result.len() || result[ri].0 == 0 {
                              let mut real_width = 0;
                              let mut char_count = 0;
                              for si in prev_line..ri {
                                 let (_, _, c, _, glyph_width) = result[si];
                                 if si != (ri-1) || (c != ' ' && c != '\t') {
                                    real_width += glyph_width;
                                    char_count += 1;
                                 }
                              }
                              let just_gap = ((just_width - real_width) as f64) / (char_count as f64);
                              for si in prev_line..ri {
                                 let (mut caret, height, c, line_height, glyph_width) = result[si];
                                 caret += (((si-prev_line) as f64) * just_gap).floor() as usize;
                                 result[si] = (caret, height, c, line_height, glyph_width);
                              }
                              prev_line = ri;
                           }
                        }
                     } else if text.align.as_str() == "center" {
                        let just_width = width;
                        let mut prev_line = 0;
                        for ri in 0..(result.len()+1) {
                           if ri==result.len() || result[ri].0 == 0 {
                              let mut real_width = 0;
                              for si in prev_line..ri {
                                 let (_, _, c, _, glyph_width) = result[si];
                                 if si != (ri-1) || (c != ' ' && c != '\t') {
                                    real_width += glyph_width;
                                 }
                              }
                              let center_gap = (((just_width - real_width) as f64) / 2.0).ceil() as usize;
                              for si in prev_line..ri {
                                 let (mut caret, height, c, line_height, glyph_width) = result[si];
                                 caret += center_gap;
                                 result[si] = (caret, height, c, line_height, glyph_width);
                              }
                              prev_line = ri;
                           }
                        }
                     } else if text.align.as_str() == "right" {
                        let just_width = width;
                        let mut prev_line = 0;
                        for ri in 0..(result.len()+1) {
                           if ri==result.len() || result[ri].0 == 0 {
                              let mut real_width = 0;
                              for si in prev_line..ri {
                                 let (_, _, c, _, glyph_width) = result[si];
                                 if si != (ri-1) || (c != ' ' && c != '\t') {
                                    real_width += glyph_width;
                                 }
                              }
                              let right_gap = ((just_width - real_width) as f64).ceil() as usize;
                              for si in prev_line..ri {
                                 let (mut caret, height, c, line_height, glyph_width) = result[si];
                                 caret += right_gap;
                                 result[si] = (caret, height, c, line_height, glyph_width);
                              }
                              prev_line = ri;
                           }
                        }
                     }
                     result
                  };

                  let mut max_x = 0;
                  let mut max_y = 0;
                  for pi in 0..positioned.len() {
                     let (caret, height, c, line_height, glyph_width) = positioned[pi];
                     let ref mut base_glyph = glyphs.get_mut(&(c,line_height)).expect("glyph").1;
                     let x = pos_x + caret;
                     let y = pos_y + height;
                     let (shadow_box, sc) = shadow;
                     if shadow_box[0]<shadow_box[2] || shadow_box[1]<shadow_box[2] {
                        base_glyph.set_color_mod((sc[0]*255.0) as u8, (sc[1]*255.0) as u8, (sc[2]*255.0) as u8);
                        base_glyph.set_alpha_mod((sc[3]*255.0) as u8);
                        for sx in (shadow_box[0]-1) .. shadow_box[2] {
                           let x = (x as i64) + sx; if x<0 { continue; }; let x = x as i32;
                        for sy in (shadow_box[1]-1) .. shadow_box[3] {
                           let y = (y as i64) + sy; if y<0 { continue; }; let y = y as i32;
                           canvas.copy(base_glyph, None, Some(Rect::new(x, y, (glyph_width as u32), (line_height as u32)))).unwrap();
                        }}
                     }
                     if ((x as usize) + (glyph_width as usize)) > max_x { max_x = (x as usize) + (glyph_width as usize) }
                     if ((y as usize) + (line_height as usize)) > max_y { max_y = (y as usize) + (line_height as usize) }
                     base_glyph.set_color_mod((color[0]*255.0) as u8, (color[1]*255.0) as u8, (color[2]*255.0) as u8);
                     base_glyph.set_alpha_mod((color[3]*255.0) as u8);
                     canvas.copy(base_glyph, None, Some(Rect::new((x as i32), (y as i32), (glyph_width as u32), (line_height as u32)))).unwrap();
                  }

                  (pos_x, pos_y, max_x, max_y)
               }
               _ => { (0,0,0,0) }
            }};
            let evs = match *c {
               Component::Text(ref mut m) => { let mut v = Vec::new(); v.extend(m.events.iter().cloned()); v }
               Component::Image(ref mut m) => { let mut v = Vec::new(); v.extend(m.events.iter().cloned()); v }
               Component::Rectangle(ref mut m) => { let mut v = Vec::new(); v.extend(m.events.iter().cloned()); v }
               _ => { panic!("unexpected Component") }
            };
            for ev in evs {
               match ev {
                  (::view::Event::Always, f) => {
                     let mut callback = f.borrow_mut();
                     (&mut *callback)(&mut events);
                  }
                  (::view::Event::Hovered, f) => {
                     if bbox.0 <= cursor_x && cursor_x < bbox.2 &&
                        bbox.1 <= cursor_y && cursor_y < bbox.3 {
                        let mut callback = f.borrow_mut();
                        (&mut *callback)(&mut events);
                     }
                  }
                  (::view::Event::Clicked, f) => {
                     if click && bbox.0 <= cursor_x && cursor_x < bbox.2 &&
                        bbox.1 <= cursor_y && cursor_y < bbox.3 {
                        let mut callback = f.borrow_mut();
                        (&mut *callback)(&mut events);
                     }
                  }
               }
            }
         }
         canvas.present();
      }
   }
}
