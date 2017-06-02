use ::events::Events;
use std::rc::Rc;
use std::cell::RefCell;
use std::slice::Iter;

pub struct Width {
   pub scalar: f64,
   pub unit: String,
}
impl Width {
   pub fn new(scalar: f64, unit: &str) -> Modifier {
      Modifier::Width(Width { scalar:scalar, unit:unit.to_owned() })
   }
}

pub struct Height {
   pub scalar: f64,
   pub unit: String,
}
impl Height {
   pub fn new(scalar: f64, unit: &str) -> Modifier {
      Modifier::Height(Height { scalar:scalar, unit:unit.to_owned() })
   }
}

pub struct TranslateX {
   pub scalar: f64,
   pub unit: String,
}
impl TranslateX {
   pub fn new(scalar: f64, unit: &str) -> Modifier {
      Modifier::TranslateX(TranslateX { scalar:scalar, unit:unit.to_owned() })
   }
}

pub struct TranslateY {
   pub scalar: f64,
   pub unit: String,
}
impl TranslateY {
   pub fn new(scalar: f64, unit: &str) -> Modifier {
      Modifier::TranslateY(TranslateY { scalar:scalar, unit:unit.to_owned() })
   }
}

pub struct Scale {
   pub scalar: f64,
   pub unit: String
}
impl Scale {
   pub fn new(scalar: f64, unit: &str) -> Modifier {
      Modifier::Scale(Scale { scalar:scalar, unit:unit.to_owned() })
   }
}

pub struct Color {
   pub rgba: [f64; 4],
}
impl Color {
   pub fn new(rgba: [f64; 4]) -> Modifier {
      Modifier::Color(Color { rgba:rgba })
   }
}

pub struct Shadow {
   pub boxed: [i64; 4],
   pub rgba: [f64; 4],
}
impl Shadow {
   pub fn new(boxed: [i64; 4], rgba: [f64; 4]) -> Modifier {
      Modifier::Shadow(Shadow { boxed:boxed, rgba:rgba })
   }
}

pub struct Image {
   pub name: String,
   pub modifiers: Vec<Modifier>,
   pub events: Vec<(Event, Rc<RefCell<FnMut(&mut Events)>>)>,
}
impl Image {
   pub fn new(name: &str) -> Component {
      Component::Image(Image { name: name.to_owned(), modifiers:Vec::new(), events:Vec::new() })
   }
}

pub struct Text {
   pub content: String,
   pub font: String,
   pub align: String,
   pub modifiers: Vec<Modifier>,
   pub events: Vec<(Event, Rc<RefCell<FnMut(&mut Events)>>)>,
}
impl Text {
   pub fn new(font: &str, cs: &str) -> Component {
      Component::Text(Text { font:font.to_owned(), content: cs.to_owned(),
                             align: "left".to_owned(), modifiers:Vec::new(), events:Vec::new() })
   }
}

pub struct Rectangle {
   pub height: f64,
   pub hunit: String,
   pub width: f64,
   pub wunit: String,
   pub modifiers: Vec<Modifier>,
   pub events: Vec<(Event, Rc<RefCell<FnMut(&mut Events)>>)>,
}
impl Rectangle {
   pub fn new(w: f64, wunit: &str, h: f64, hunit: &str) -> Component {
      Component::Rectangle(Rectangle { width:w, wunit:wunit.to_owned(), height:h, hunit:hunit.to_owned(),
                                       modifiers:Vec::new(), events:Vec::new() })
   }
}

macro_rules! push_event {
   ($base: expr, $cls: ident, $fnx: ident) => {{
      let ref mut m = $base;
      loop {
         let mut found = false;
         for mi in 0..m.len() {
            match m[mi].0 {
               Event::$cls => {
                  m.remove(mi);
                  found = true;
                  break;
               }
               _ => {}
            }
         }
         if !found { break; }
      }
      m.push( (Event::$cls, Rc::new(RefCell::new($fnx))) );
   }};
}
macro_rules! push_modifier {
   ($base: expr, $cls: ident, ( $($arg:expr ,)* ) ) => {{
      let ref mut m = $base;
      loop {
         let mut found = false;
         for mi in 0..m.len() {
            match m[mi] {
               Modifier::$cls(_) => {
                  m.remove(mi);
                  found = true;
                  break;
               }
               _ => {}
            }
         }
         if !found { break; }
      }
      m.push( $cls::new( $($arg),* ) );
   }}
}

pub enum Component {
   Modifier(Modifier),
   Image(Image),
   Text(Text),
   Rectangle(Rectangle),
}
impl Component {
   pub fn modifiers(&mut self) -> Iter<Modifier> {
      match *self {
         Component::Text(ref m) => { m.modifiers.iter() }
         Component::Image(ref m) => { m.modifiers.iter() }
         Component::Rectangle(ref m) => { m.modifiers.iter() }
         _ => { panic!("No modifiers on component") }
      }
   }
   pub fn width(mut self, scalar: f64, unit: &str) -> Component {
      match self {
         Component::Text(ref mut m) => { push_modifier!(m.modifiers, Width, (scalar, unit,)) }
         Component::Image(ref mut m) => { push_modifier!(m.modifiers, Width, (scalar, unit,)) }
         Component::Rectangle(ref mut m) => { push_modifier!(m.modifiers, Width, (scalar, unit,)) }
         _ => {}
      }; self
   }
   pub fn height(mut self, scalar: f64, unit: &str) -> Component {
      match self {
         Component::Text(ref mut m) => { push_modifier!(m.modifiers, Height, (scalar, unit,)) }
         Component::Image(ref mut m) => { push_modifier!(m.modifiers, Height, (scalar, unit,)) }
         Component::Rectangle(ref mut m) => { push_modifier!(m.modifiers, Height, (scalar, unit,)) }
         _ => {}
      }; self
   }
   pub fn translate_x(mut self, scalar: f64, unit: &str) -> Component {
      match self {
         Component::Text(ref mut m) => { push_modifier!(m.modifiers, TranslateX, (scalar, unit,)) }
         Component::Image(ref mut m) => { push_modifier!(m.modifiers, TranslateX, (scalar, unit,)) }
         Component::Rectangle(ref mut m) => { push_modifier!(m.modifiers, TranslateX, (scalar, unit,)) }
         _ => {}
      }; self
   }
   pub fn translate_y(mut self, scalar: f64, unit: &str) -> Component {
      match self {
         Component::Text(ref mut m) => { push_modifier!(m.modifiers, TranslateY, (scalar, unit,)) }
         Component::Image(ref mut m) => { push_modifier!(m.modifiers, TranslateY, (scalar, unit,)) }
         Component::Rectangle(ref mut m) => { push_modifier!(m.modifiers, TranslateY, (scalar, unit,)) }
         _ => {}
      }; self
   }
   pub fn color(mut self, rgba: [f64; 4]) -> Component {
      match self {
         Component::Text(ref mut m) => { push_modifier!(m.modifiers, Color, (rgba,)) }
         Component::Image(ref mut m) => { push_modifier!(m.modifiers, Color, (rgba,)) }
         Component::Rectangle(ref mut m) => { push_modifier!(m.modifiers, Color, (rgba,)) }
         _ => {}
      }; self
   }
   pub fn scale(mut self, scalar: f64, unit: &str) -> Component {
      match self {
         Component::Text(ref mut m) => { push_modifier!(m.modifiers, Scale, (scalar, unit,)) }
         Component::Image(ref mut m) => { push_modifier!(m.modifiers, Scale, (scalar, unit,)) }
         Component::Rectangle(ref mut m) => { push_modifier!(m.modifiers, Scale, (scalar, unit,)) }
         _ => {}
      }; self
   }
   pub fn align(mut self, align: &str) -> Component {
      match self {
         Component::Text(ref mut m) => { m.align = align.to_owned(); }
         _ => {}
      }; self
   }
   pub fn shadow(mut self, d: [i64; 4], c: [f64; 4]) -> Component {
      match self {
         Component::Text(ref mut m) => { push_modifier!(m.modifiers, Shadow, (d, c,)) }
         Component::Image(ref mut m) => { push_modifier!(m.modifiers, Shadow, (d, c,)) }
         Component::Rectangle(ref mut m) => { push_modifier!(m.modifiers, Shadow, (d, c,)) }
         _ => {}
      }; self
   }
   pub fn clicked<F>(mut self, f: F) -> Component 
          where F: 'static + FnMut(&mut Events) {
      match self {
         Component::Text(ref mut m) => { push_event!(m.events, Clicked, f); }
         Component::Image(ref mut m) => { push_event!(m.events, Clicked, f); }
         Component::Rectangle(ref mut m) => { push_event!(m.events, Clicked, f); }
         _ => {}
      }; self
   }
   pub fn hovered<F>(mut self, f: F) -> Component 
          where F: 'static + FnMut(&mut Events) {
      match self {
         Component::Text(ref mut m) => { push_event!(m.events, Hovered, f); }
         Component::Image(ref mut m) => { push_event!(m.events, Hovered, f); }
         Component::Rectangle(ref mut m) => { push_event!(m.events, Hovered, f); }
         _ => {}
      }; self
   }
   pub fn always<F>(mut self, f: F) -> Component 
          where F: 'static + FnMut(&mut Events) {
      match self {
         Component::Text(ref mut m) => { push_event!(m.events, Always, f); }
         Component::Image(ref mut m) => { push_event!(m.events, Always, f); }
         Component::Rectangle(ref mut m) => { push_event!(m.events, Always, f); }
         _ => {}
      }; self
   }
}

#[derive(Debug, Clone)]
pub enum Event {
   Clicked,
   Hovered,
   Always,
}

pub enum Modifier {
   Color(Color),
   Scale(Scale),
   TranslateX(TranslateX),
   TranslateY(TranslateY),
   Width(Width),
   Height(Height),
   Shadow(Shadow),
}

pub struct View {
   pub components: Vec<Component>,
}
impl View {
   pub fn new() -> View {
      View {
         components: Vec::new()
      }
   }
   pub fn append(&mut self, c: Component) -> &mut View {
      self.components.push( c );
      self
   }
}
