//! Visual Components and Modifiers which are to be composed into a single render cycle.

use ::events::Events;
use std::rc::Rc;
use std::cell::RefCell;
use std::slice::Iter;

/// A Modifier to define the width of a Component
pub struct Width {
   ///scalar
   pub scalar: f64,

   ///unit
   pub unit: String,
}
impl Width {
   ///Create a new Width Modifier
   pub fn new(scalar: f64, unit: &str) -> Modifier {
      Modifier::Width(Width { scalar:scalar, unit:unit.to_owned() })
   }
}

/// A Modifier to define the height of a Component
pub struct Height {
   ///scalar
   pub scalar: f64,

   ///unit
   pub unit: String,
}
impl Height {
   ///Create a new Height Modifier
   pub fn new(scalar: f64, unit: &str) -> Modifier {
      Modifier::Height(Height { scalar:scalar, unit:unit.to_owned() })
   }
}

/// A Modifier to define the Center of Gravity of a Component
pub struct CenterOfGravity {
   ///horizontal center: [0,1]
   pub horizontal: f64,

   ///vertical center: [0,1]
   pub vertical: f64,
}
impl CenterOfGravity {
   ///Create a new Center of Gravity Modifier
   pub fn new(horizontal: f64, vertical: f64) -> Modifier {
      Modifier::CenterOfGravity(CenterOfGravity { horizontal:horizontal, vertical:vertical })
   }
}

/// A Modifier to define the Horizontal Offset of a Component
pub struct TranslateX {
   ///scalar
   pub scalar: f64,

   ///unit
   pub unit: String,
}
impl TranslateX {
   ///Create a new TranslateX Modifier
   pub fn new(scalar: f64, unit: &str) -> Modifier {
      Modifier::TranslateX(TranslateX { scalar:scalar, unit:unit.to_owned() })
   }
}

/// A Modifier to define the Vertical Offset of a Component
pub struct TranslateY {
   ///scalar
   pub scalar: f64,

   ///unit
   pub unit: String,
}
impl TranslateY {
   ///Create a new TranslateY Modifier
   pub fn new(scalar: f64, unit: &str) -> Modifier {
      Modifier::TranslateY(TranslateY { scalar:scalar, unit:unit.to_owned() })
   }
}

/// A Modifier to define the Line Height of a Component
pub struct Scale {
   ///scalar
   pub scalar: f64,

   ///unit
   pub unit: String
}
impl Scale {
   ///Create a new Scale Modifier
   pub fn new(scalar: f64, unit: &str) -> Modifier {
      Modifier::Scale(Scale { scalar:scalar, unit:unit.to_owned() })
   }
}

/// A Modifier to define the Color of a Component
pub struct Color {
   ///rgba
   pub rgba: [f64; 4],
}
impl Color {
   ///Create a new Color Modifier
   pub fn new(rgba: [f64; 4]) -> Modifier {
      Modifier::Color(Color { rgba:rgba })
   }
}

/// A Modifier to define the View-Bound State of a Component
pub struct State {
   ///state
   pub state: String
}
impl State {
   ///Create a new State Modifier
   pub fn new(state: &str) -> Modifier {
      Modifier::State(State { state:state.to_owned() })
   }
}

/// A Modifier to define the Border of a Component
pub struct Border {
   ///rgba
   pub rgba: [f64; 4],

   ///scalar
   pub scalar: f64,

   ///unit
   pub unit: String,
}
impl Border {
   ///Create a new Border Modifier
   pub fn new(rgba: [f64; 4], scalar: f64, unit: &str) -> Modifier {
      Modifier::Border(Border { rgba:rgba, scalar:scalar, unit:unit.to_owned() })
   }
}

/// A Modifier to define the Shadow of a Component
pub struct Shadow {
   ///[left, top, right, bottom] offsets
   pub boxed: [i64; 4],

   ///rgba
   pub rgba: [f64; 4],
}
impl Shadow {
   ///Create a new Shadow Modifier
   pub fn new(boxed: [i64; 4], rgba: [f64; 4]) -> Modifier {
      Modifier::Shadow(Shadow { boxed:boxed, rgba:rgba })
   }
}

/// A Component to describe an Image to be rendered
pub struct Image {
   ///Asset Name
   pub name: String,

   ///Component Modifiers
   pub modifiers: Vec<Modifier>,

   ///Event Handlers
   pub events: Vec<(Event, Rc<RefCell<FnMut(&mut Events)>>)>,
}
impl Image {
   ///Create a new Image Component
   pub fn new(name: &str) -> Component {
      Component::Image(Image { name: name.to_owned(), modifiers:Vec::new(), events:Vec::new() })
   }
}

/// A Component to describe Text to be rendered
pub struct Text {
   ///Text Content
   pub content: String,

   ///Text Font
   pub font: String,

   ///Text Alignment
   pub align: String,

   ///Component Modifiers
   pub modifiers: Vec<Modifier>,

   ///Event Handlers
   pub events: Vec<(Event, Rc<RefCell<FnMut(&mut Events)>>)>,
}
impl Text {
   ///Create a new Text Component
   pub fn new(font: &str, cs: &str) -> Component {
      Component::Text(Text { font:font.to_owned(), content: cs.to_owned(),
                             align: "left".to_owned(), modifiers:Vec::new(), events:Vec::new() })
   }
}

/// A Component to describe a Rectangle to be rendered
pub struct Rectangle {
   ///Component Modifiers
   pub modifiers: Vec<Modifier>,

   ///Event Handlers
   pub events: Vec<(Event, Rc<RefCell<FnMut(&mut Events)>>)>,
}
impl Rectangle {
   ///Create a new Rectangle Component
   pub fn new(w: f64, wunit: &str, h: f64, hunit: &str) -> Component {
      Component::Rectangle(Rectangle { modifiers:Vec::new(), events:Vec::new() })
      .width(w, wunit)
      .height(h, hunit)
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

///A renderable View Component
pub enum Component {
   ///Component::Modifier
   Modifier(Modifier),

   ///Component::Image
   Image(Image),

   ///Component::Text
   Text(Text),

   ///Component::Rectangle
   Rectangle(Rectangle),
}
impl Component {

   ///Iterate over Modifiers of this Component
   pub fn modifiers(&mut self) -> Iter<Modifier> {
      match *self {
         Component::Text(ref m) => { m.modifiers.iter() }
         Component::Image(ref m) => { m.modifiers.iter() }
         Component::Rectangle(ref m) => { m.modifiers.iter() }
         _ => { panic!("No modifiers on component") }
      }
   }

   ///Add a State Modifier to this Component
   pub fn state(mut self, state: &str) -> Component {
      match self {
         Component::Text(ref mut m) => { push_modifier!(m.modifiers, State, (state,)) }
         Component::Image(ref mut m) => { push_modifier!(m.modifiers, State, (state,)) }
         Component::Rectangle(ref mut m) => { push_modifier!(m.modifiers, State, (state,)) }
         _ => {}
      }; self
   }

   ///Add a Width Modifier to this Component
   pub fn width(mut self, scalar: f64, unit: &str) -> Component {
      match self {
         Component::Text(ref mut m) => { push_modifier!(m.modifiers, Width, (scalar, unit,)) }
         Component::Image(ref mut m) => { push_modifier!(m.modifiers, Width, (scalar, unit,)) }
         Component::Rectangle(ref mut m) => { push_modifier!(m.modifiers, Width, (scalar, unit,)) }
         _ => {}
      }; self
   }

   ///Add a Height Modifier to this Component
   pub fn height(mut self, scalar: f64, unit: &str) -> Component {
      match self {
         Component::Text(ref mut m) => { push_modifier!(m.modifiers, Height, (scalar, unit,)) }
         Component::Image(ref mut m) => { push_modifier!(m.modifiers, Height, (scalar, unit,)) }
         Component::Rectangle(ref mut m) => { push_modifier!(m.modifiers, Height, (scalar, unit,)) }
         _ => {}
      }; self
   }

   ///Add a Center of Gravity Modifier to this Component
   pub fn cog(mut self, x: f64, y: f64) -> Component {
      match self {
         Component::Text(ref mut m) => { push_modifier!(m.modifiers, CenterOfGravity, (x, y,)) }
         Component::Image(ref mut m) => { push_modifier!(m.modifiers, CenterOfGravity, (x, y,)) }
         Component::Rectangle(ref mut m) => { push_modifier!(m.modifiers, CenterOfGravity, (x, y,)) }
         _ => {}
      }; self
   }

   ///Add a TranslateX Modifier to this Component
   pub fn translate_x(mut self, scalar: f64, unit: &str) -> Component {
      match self {
         Component::Text(ref mut m) => { push_modifier!(m.modifiers, TranslateX, (scalar, unit,)) }
         Component::Image(ref mut m) => { push_modifier!(m.modifiers, TranslateX, (scalar, unit,)) }
         Component::Rectangle(ref mut m) => { push_modifier!(m.modifiers, TranslateX, (scalar, unit,)) }
         _ => {}
      }; self
   }

   ///Add a TranslateY Modifier to this Component
   pub fn translate_y(mut self, scalar: f64, unit: &str) -> Component {
      match self {
         Component::Text(ref mut m) => { push_modifier!(m.modifiers, TranslateY, (scalar, unit,)) }
         Component::Image(ref mut m) => { push_modifier!(m.modifiers, TranslateY, (scalar, unit,)) }
         Component::Rectangle(ref mut m) => { push_modifier!(m.modifiers, TranslateY, (scalar, unit,)) }
         _ => {}
      }; self
   }

   ///Add a Color Modifier to this Component
   pub fn color(mut self, rgba: [f64; 4]) -> Component {
      match self {
         Component::Text(ref mut m) => { push_modifier!(m.modifiers, Color, (rgba,)) }
         Component::Image(ref mut m) => { push_modifier!(m.modifiers, Color, (rgba,)) }
         Component::Rectangle(ref mut m) => { push_modifier!(m.modifiers, Color, (rgba,)) }
         _ => {}
      }; self
   }

   ///Add a Scale Modifier to this Component
   pub fn scale(mut self, scalar: f64, unit: &str) -> Component {
      match self {
         Component::Text(ref mut m) => { push_modifier!(m.modifiers, Scale, (scalar, unit,)) }
         Component::Image(ref mut m) => { push_modifier!(m.modifiers, Scale, (scalar, unit,)) }
         Component::Rectangle(ref mut m) => { push_modifier!(m.modifiers, Scale, (scalar, unit,)) }
         _ => {}
      }; self
   }

   ///Add an Align Modifier to this Component
   pub fn align(mut self, align: &str) -> Component {
      match self {
         Component::Text(ref mut m) => { m.align = align.to_owned(); }
         _ => {}
      }; self
   }

   ///Add a Border Modifier to this Component
   pub fn border(mut self, clr: [f64; 4], scalar: f64, unit: &str) -> Component {
      match self {
         Component::Text(ref mut m) => { push_modifier!(m.modifiers, Border, (clr, scalar, unit,)) }
         Component::Image(ref mut m) => { push_modifier!(m.modifiers, Border, (clr, scalar, unit,)) }
         Component::Rectangle(ref mut m) => { push_modifier!(m.modifiers, Border, (clr, scalar, unit,)) }
         _ => {}
      }; self
   }

   ///Add a Shadow Modifier to this Component
   pub fn shadow(mut self, d: [i64; 4], c: [f64; 4]) -> Component {
      match self {
         Component::Text(ref mut m) => { push_modifier!(m.modifiers, Shadow, (d, c,)) }
         Component::Image(ref mut m) => { push_modifier!(m.modifiers, Shadow, (d, c,)) }
         Component::Rectangle(ref mut m) => { push_modifier!(m.modifiers, Shadow, (d, c,)) }
         _ => {}
      }; self
   }

   ///Add a Clicked event listener to this Component
   pub fn clicked<F>(mut self, f: F) -> Component 
          where F: 'static + FnMut(&mut Events) {
      match self {
         Component::Text(ref mut m) => { push_event!(m.events, Clicked, f); }
         Component::Image(ref mut m) => { push_event!(m.events, Clicked, f); }
         Component::Rectangle(ref mut m) => { push_event!(m.events, Clicked, f); }
         _ => {}
      }; self
   }

   ///Add a Hovered event listener to this Component
   pub fn hovered<F>(mut self, f: F) -> Component 
          where F: 'static + FnMut(&mut Events) {
      match self {
         Component::Text(ref mut m) => { push_event!(m.events, Hovered, f); }
         Component::Image(ref mut m) => { push_event!(m.events, Hovered, f); }
         Component::Rectangle(ref mut m) => { push_event!(m.events, Hovered, f); }
         _ => {}
      }; self
   }

   ///Add an Always event listener to this Component
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
///User Events that Components can subscribe to
pub enum Event {
   ///Event::Clicked
   Clicked,

   ///Event::Hovered
   Hovered,

   ///Event::Always
   Always,
}

///Modifiers adjust the rendering qualities of Components
pub enum Modifier {
   ///Modifier::State
   State(State),

   ///Modifier::Color
   Color(Color),

   ///Modifier::Border
   Border(Border),

   ///Modifier::Scale
   Scale(Scale),

   ///Modifier::CenterOfGravity
   CenterOfGravity(CenterOfGravity),

   ///Modifier::TranslateX
   TranslateX(TranslateX),

   ///Modifier::TranslateY
   TranslateY(TranslateY),

   ///Modifier::Width
   Width(Width),

   ///Modifier::Height
   Height(Height),

   ///Modifier::Shadow
   Shadow(Shadow),
}

///The render queue
pub struct View {
   ///All scheduled components
   pub components: Vec<Component>,
}
impl View {
   ///A renderable view to contain all queued Components
   pub fn new() -> View {
      View {
         components: Vec::new()
      }
   }
   ///Put a Component onto the render queue
   pub fn append(&mut self, c: Component) -> &mut View {
      self.components.push( c );
      self
   }
}
