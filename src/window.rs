extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;
use gfx::traits::FactoryExt;
use gfx::Device;
use self::glutin::GlContext;

extern crate time;
extern crate unicode_normalization;
use std::collections::{HashMap};
use std::rc::Rc;
use std::f64::consts::{PI};

extern crate rusttype;
use self::rusttype::{FontCollection, Scale, point, PositionedGlyph};
extern crate image;
use self::image::*;

use ::events::{Events};
use ::view::{View, Component, Modifier, ViewUnit, AlignUnit, AngleUnit };

///gfx Rgba8 Color Format
pub type ColorFormat = gfx::format::Rgba8;

///gfx DepthStencil Format
pub type DepthFormat = gfx::format::DepthStencil;

gfx_defines!{
    vertex Vertex {
        pos: [f32; 2] = "a_Pos",
        uv: [f32; 2] = "a_Uv",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        vtex: gfx::TextureSampler<[f32; 4]> = "t_v0",
        out: gfx::RenderTarget<ColorFormat> = "Target0",
    }
}

/*
fn load_texture<R, F>(factory: &mut F, data: &[u8])
                -> Result<gfx::handle::ShaderResourceView<R, [f32; 4]>, String> where
                R: gfx::Resources, F: gfx::Factory<R> {
    use std::io::Cursor;
    use gfx::texture as t;
    let img = image::load(Cursor::new(data), image::PNG).unwrap().to_rgba();
    let (width, height) = img.dimensions();
    let kind = t::Kind::D2(width as t::Size, height as t::Size, t::AaMode::Single);
    let (_, view) = factory.create_texture_immutable_u8::<Rgba8>(kind, &[&img]).unwrap();
    Ok(view)
}
*/

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
      let epoch = time::precise_time_s();

      let mut events_loop = glutin::EventsLoop::new();
      let window_config = glutin::WindowBuilder::new()
         .with_title("Triangle example".to_string())
         .with_dimensions(1024, 768);
      let context = glutin::ContextBuilder::new()
         .with_vsync(true);
      let (window, mut device, mut factory, mut main_color, mut main_depth) =
         gfx_window_glutin::init::<gfx::format::Rgba8, gfx::format::DepthStencil>(window_config, context, &events_loop);
      let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

      let mut running = true;
      while running {

         // fetch events
         events_loop.poll_events(|event| {
            if let glutin::Event::WindowEvent { event, .. } = event {
               match event {
                  glutin::WindowEvent::KeyboardInput {
                     input: glutin::KeyboardInput {
                        virtual_keycode: Some(glutin::VirtualKeyCode::Escape),
                        .. },
                     ..
                  } | glutin::WindowEvent::Closed => { running = false },
                  glutin::WindowEvent::Resized(width, height) => {
                     window.resize(width, height);
                     gfx_window_glutin::update_views(&window, &mut main_color, &mut main_depth);
                  },
                  _ => (),
               }
            }
         });

         encoder.clear(&main_color, [0.1, 0.2, 0.3, 1.0]);
         encoder.flush(&mut device);
         window.swap_buffers().unwrap();
         device.cleanup();
      }

   }
}
