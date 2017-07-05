use ::events::Events;
use std::rc::Rc;
use std::cell::RefCell;
use std::slice::Iter;

#[derive(Debug)]
/// A typesafe unit for viewable components and modifiers
pub enum ViewUnit {
   ///em
   Em,

   ///Axis Percent: %
   Percent,

   ///Horizontal Percent: h%
   HorizontalPercent,

   ///Vertical Percent: v%
   VerticalPercent,

   ///min(Vertical,Horizontal) Percent: <%
   MinPercent,

   ///max(Vertical,Horizontal) Percent: >%
   MaxPercent,

   ///Centered in Container: =
   Center,

   ///Pixel: px
   Pixel,
}
impl ViewUnit {
   /// Convert a raw string to ViewUnit
   pub fn new(s: &str) -> ViewUnit {
      match s {
         "em" => { ViewUnit::Em }
         "%" => { ViewUnit::Percent }
         "h%" => { ViewUnit::HorizontalPercent }
         "v%" => { ViewUnit::VerticalPercent }
         "<%" => { ViewUnit::MinPercent }
         ">%" => { ViewUnit::MaxPercent }
         "=" => { ViewUnit::Center }
         "px" => { ViewUnit::Pixel }
         u => { panic!("Invalid View Unit: {}", u) }
      }
   }
}
impl<'a> Into<ViewUnit> for &'a str {
    fn into(self) -> ViewUnit {
       ViewUnit::new(self)
    }
}

#[derive(Debug)]
/// A typesafe unit for component alignment in containers
pub enum AlignUnit {
   ///left
   Left,

   ///center
   Center,

   ///right
   Right,

   ///justify
   Justify,
}
impl AlignUnit {
   /// Convert a raw string to AlignUnit
   pub fn new(s: &str) -> AlignUnit {
      match s {
         "left" => { AlignUnit::Left }
         "center" => { AlignUnit::Center }
         "right" => { AlignUnit::Right }
         "justify" => { AlignUnit::Justify }
         u => { panic!("Invalid Align Unit: {}", u) }
      }
   }
}
impl<'a> Into<AlignUnit> for &'a str {
    fn into(self) -> AlignUnit {
       AlignUnit::new(self)
    }
}


#[derive(Debug)]
/// A typesafe unit for Angle units
pub enum AngleUnit {
   ///degree
   Degree,

   ///radian
   Radian,

   ///hour
   Hour,
}
impl AngleUnit {
   /// Convert a raw string to AngleUnit
   pub fn new(s: &str) -> AngleUnit {
      match s {
         "degree" => { AngleUnit::Degree }
         "radian" => { AngleUnit::Radian }
         "hour" => { AngleUnit::Hour }
         u => { panic!("Invalid Angle Unit: {}", u) }
      }
   }
}
impl<'a> Into<AngleUnit> for &'a str {
    fn into(self) -> AngleUnit {
       AngleUnit::new(self)
    }
}


/// A Modifier to define the width of a Component
pub struct Width {
   ///scalar
   pub scalar: f64,

   ///unit
   pub unit: ViewUnit,
}
impl Width {
   ///Create a new Width Modifier
   pub fn new(scalar: f64, unit: ViewUnit) -> Modifier {
      Modifier::Width(Width { scalar:scalar, unit:unit })
   }
}

/// A Modifier to define the height of a Component
pub struct Height {
   ///scalar
   pub scalar: f64,

   ///unit
   pub unit: ViewUnit,
}
impl Height {
   ///Create a new Height Modifier
   pub fn new(scalar: f64, unit: ViewUnit) -> Modifier {
      Modifier::Height(Height { scalar:scalar, unit:unit })
   }
}

/// A Modifier to define the angle of a Component
pub struct Angle {
   ///scalar
   pub scalar: f64,

   ///unit
   pub unit: AngleUnit,
}
impl Angle {
   ///Create a new Angle Modifier
   pub fn new(scalar: f64, unit: AngleUnit) -> Modifier {
      Modifier::Angle(Angle { scalar:scalar, unit:unit })
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
   pub unit: ViewUnit,
}
impl TranslateX {
   ///Create a new TranslateX Modifier
   pub fn new(scalar: f64, unit: ViewUnit) -> Modifier {
      Modifier::TranslateX(TranslateX { scalar:scalar, unit:unit })
   }
}

/// A Modifier to define the Vertical Offset of a Component
pub struct TranslateY {
   ///scalar
   pub scalar: f64,

   ///unit
   pub unit: ViewUnit,
}
impl TranslateY {
   ///Create a new TranslateY Modifier
   pub fn new(scalar: f64, unit: ViewUnit) -> Modifier {
      Modifier::TranslateY(TranslateY { scalar:scalar, unit:unit })
   }
}

/// A Modifier to define the Line Height of a Component
pub struct Scale {
   ///scalar
   pub scalar: f64,

   ///unit
   pub unit: ViewUnit
}
impl Scale {
   ///Create a new Scale Modifier
   pub fn new(scalar: f64, unit: ViewUnit) -> Modifier {
      Modifier::Scale(Scale { scalar:scalar, unit:unit })
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

/// A Modifier to define the Border of a Component
pub struct Border {
   ///rgba
   pub rgba: [f64; 4],

   ///scalar
   pub scalar: f64,

   ///unit
   pub unit: ViewUnit,
}
impl Border {
   ///Create a new Border Modifier
   pub fn new(rgba: [f64; 4], scalar: f64, unit: ViewUnit) -> Modifier {
      Modifier::Border(Border { rgba:rgba, scalar:scalar, unit:unit })
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

/// A Modifier to hold state of this individual Component
pub struct State {
   ///Value of state
   pub val: String,
}
impl State {
   ///Create a new State Modifier
   pub fn new(val: &str) -> Modifier {
      Modifier::State(State { val:val.to_owned() })
   }
}

/// A Modifier to describe Conditions of what to render
pub struct Conditional {
   ///Key to bind conditional value to
   pub key: String,

   ///Value bound to conditional objects
   pub val: String,
}
impl Conditional {
   ///Create a new Conditional Modifier
   pub fn new(key: &str, val: &str) -> Modifier {
      Modifier::Conditional(Conditional { key:key.to_owned(), val:val.to_owned() })
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
   pub align: AlignUnit,

   ///Component Modifiers
   pub modifiers: Vec<Modifier>,

   ///Event Handlers
   pub events: Vec<(Event, Rc<RefCell<FnMut(&mut Events)>>)>,
}
impl Text {
   ///Create a new Text Component
   pub fn new(font: &str, cs: &str) -> Component {
      Component::Text(Text { font:font.to_owned(), content: cs.to_owned(),
                             align: AlignUnit::Left, modifiers:Vec::new(), events:Vec::new() })
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
   pub fn new<T>(w: f64, wunit: T, h: f64, hunit: T) -> Component
      where T: Into<ViewUnit> {
      Component::Rectangle(Rectangle { modifiers:Vec::new(), events:Vec::new() })
      .width(w, wunit.into())
      .height(h, hunit.into())
   }
}

macro_rules! push_event {
   ($base: expr, $cls: ident, $fnx: ident) => {{
      let ref mut m = $base;
      m.push( (Event::$cls, Rc::new(RefCell::new($fnx))) );
   }};
}
macro_rules! push_modifier {
   ($base: expr, $cls: ident, ( $($arg:expr ,)* ) ) => {{
      let ref mut m = $base;
      m.push( $cls::new( $($arg),* ) );
   }}
}

///A renderable View Component
pub enum Component {
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
      }
   }

   ///Add a Width Modifier to this Component
   pub fn width<T>(mut self, scalar: f64, unit: T) -> Component
      where T: Into<ViewUnit> {
      match self {
         Component::Text(ref mut m) => { push_modifier!(m.modifiers, Width, (scalar, unit.into(),)) }
         Component::Image(ref mut m) => { push_modifier!(m.modifiers, Width, (scalar, unit.into(),)) }
         Component::Rectangle(ref mut m) => { push_modifier!(m.modifiers, Width, (scalar, unit.into(),)) }
      }; self
   }

   ///Add a Height Modifier to this Component
   pub fn height<T>(mut self, scalar: f64, unit: T) -> Component
      where T: Into<ViewUnit> {
      match self {
         Component::Text(ref mut m) => { push_modifier!(m.modifiers, Height, (scalar, unit.into(),)) }
         Component::Image(ref mut m) => { push_modifier!(m.modifiers, Height, (scalar, unit.into(),)) }
         Component::Rectangle(ref mut m) => { push_modifier!(m.modifiers, Height, (scalar, unit.into(),)) }
      }; self
   }

   ///Add an Angle Modifier to this Component
   pub fn rotate<T>(mut self, scalar: f64, unit: T) -> Component
      where T: Into<AngleUnit> {
      match self {
         Component::Text(ref mut m) => { push_modifier!(m.modifiers, Angle, (scalar, unit.into(),)) }
         Component::Image(ref mut m) => { push_modifier!(m.modifiers, Angle, (scalar, unit.into(),)) }
         Component::Rectangle(ref mut m) => { push_modifier!(m.modifiers, Angle, (scalar, unit.into(),)) }
      }; self
   }

   ///Add a Center of Gravity Modifier to this Component
   pub fn cog(mut self, x: f64, y: f64) -> Component {
      match self {
         Component::Text(ref mut m) => { push_modifier!(m.modifiers, CenterOfGravity, (x, y,)) }
         Component::Image(ref mut m) => { push_modifier!(m.modifiers, CenterOfGravity, (x, y,)) }
         Component::Rectangle(ref mut m) => { push_modifier!(m.modifiers, CenterOfGravity, (x, y,)) }
      }; self
   }

   ///Add a TranslateX Modifier to this Component
   pub fn translate_x<T>(mut self, scalar: f64, unit: T) -> Component
      where T: Into<ViewUnit> {
      match self {
         Component::Text(ref mut m) => { push_modifier!(m.modifiers, TranslateX, (scalar, unit.into(),)) }
         Component::Image(ref mut m) => { push_modifier!(m.modifiers, TranslateX, (scalar, unit.into(),)) }
         Component::Rectangle(ref mut m) => { push_modifier!(m.modifiers, TranslateX, (scalar, unit.into(),)) }
      }; self
   }

   ///Add a TranslateY Modifier to this Component
   pub fn translate_y<T>(mut self, scalar: f64, unit: T) -> Component
      where T: Into<ViewUnit> {
      match self {
         Component::Text(ref mut m) => { push_modifier!(m.modifiers, TranslateY, (scalar, unit.into(),)) }
         Component::Image(ref mut m) => { push_modifier!(m.modifiers, TranslateY, (scalar, unit.into(),)) }
         Component::Rectangle(ref mut m) => { push_modifier!(m.modifiers, TranslateY, (scalar, unit.into(),)) }
      }; self
   }

   ///Add a Color Modifier to this Component
   pub fn color(mut self, rgba: [f64; 4]) -> Component {
      match self {
         Component::Text(ref mut m) => { push_modifier!(m.modifiers, Color, (rgba,)) }
         Component::Image(ref mut m) => { push_modifier!(m.modifiers, Color, (rgba,)) }
         Component::Rectangle(ref mut m) => { push_modifier!(m.modifiers, Color, (rgba,)) }
      }; self
   }

   ///Add a Scale Modifier to this Component
   pub fn scale<T>(mut self, scalar: f64, unit: T) -> Component
      where T: Into<ViewUnit> {
      match self {
         Component::Text(ref mut m) => { push_modifier!(m.modifiers, Scale, (scalar, unit.into(),)) }
         Component::Image(ref mut m) => { push_modifier!(m.modifiers, Scale, (scalar, unit.into(),)) }
         Component::Rectangle(ref mut m) => { push_modifier!(m.modifiers, Scale, (scalar, unit.into(),)) }
      }; self
   }

   ///Add an Align Modifier to this Component
   pub fn align<T>(mut self, align: T) -> Component
      where T: Into<AlignUnit> {
      match self {
         Component::Text(ref mut m) => { m.align = align.into(); }
         _ => {}
      }; self
   }

   ///Add a Border Modifier to this Component
   pub fn border<T>(mut self, clr: [f64; 4], scalar: f64, unit: T) -> Component
      where T: Into<ViewUnit> {
      match self {
         Component::Text(ref mut m) => { push_modifier!(m.modifiers, Border, (clr, scalar, unit.into(),)) }
         Component::Image(ref mut m) => { push_modifier!(m.modifiers, Border, (clr, scalar, unit.into(),)) }
         Component::Rectangle(ref mut m) => { push_modifier!(m.modifiers, Border, (clr, scalar, unit.into(),)) }
      }; self
   }

   ///Add a Shadow Modifier to this Component
   pub fn shadow(mut self, d: [i64; 4], c: [f64; 4]) -> Component {
      match self {
         Component::Text(ref mut m) => { push_modifier!(m.modifiers, Shadow, (d, c,)) }
         Component::Image(ref mut m) => { push_modifier!(m.modifiers, Shadow, (d, c,)) }
         Component::Rectangle(ref mut m) => { push_modifier!(m.modifiers, Shadow, (d, c,)) }
      }; self
   }

   ///Add a State Modifier to this Component
   pub fn state(mut self, val: &str) -> Component {
      match self {
         Component::Text(ref mut m) => { push_modifier!(m.modifiers, State, (val,)); }
         Component::Image(ref mut m) => { push_modifier!(m.modifiers, State, (val,)); }
         Component::Rectangle(ref mut m) => { push_modifier!(m.modifiers, State, (val,)); }
      }; self
   }

   ///Add a Condition Modifier to this Component
   pub fn condition(mut self, key: &str, val: &str) -> Component {
      match self {
         Component::Text(ref mut m) => { push_modifier!(m.modifiers, Conditional, (key, val,)); }
         Component::Image(ref mut m) => { push_modifier!(m.modifiers, Conditional, (key, val,)); }
         Component::Rectangle(ref mut m) => { push_modifier!(m.modifiers, Conditional, (key, val,)); }
      }; self
   }

   ///Add a Clicked event listener to this Component
   pub fn clicked<F>(mut self, f: F) -> Component 
          where F: 'static + FnMut(&mut Events) {
      match self {
         Component::Text(ref mut m) => { push_event!(m.events, Clicked, f); }
         Component::Image(ref mut m) => { push_event!(m.events, Clicked, f); }
         Component::Rectangle(ref mut m) => { push_event!(m.events, Clicked, f); }
      }; self
   }

   ///Add a Hovered event listener to this Component
   pub fn hovered<F>(mut self, f: F) -> Component 
          where F: 'static + FnMut(&mut Events) {
      match self {
         Component::Text(ref mut m) => { push_event!(m.events, Hovered, f); }
         Component::Image(ref mut m) => { push_event!(m.events, Hovered, f); }
         Component::Rectangle(ref mut m) => { push_event!(m.events, Hovered, f); }
      }; self
   }

   ///Add an Always event listener to this Component
   pub fn always<F>(mut self, f: F) -> Component 
          where F: 'static + FnMut(&mut Events) {
      match self {
         Component::Text(ref mut m) => { push_event!(m.events, Always, f); }
         Component::Image(ref mut m) => { push_event!(m.events, Always, f); }
         Component::Rectangle(ref mut m) => { push_event!(m.events, Always, f); }
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

   ///Modifier::Angle
   Angle(Angle),

   ///Modifier::Shadow
   Shadow(Shadow),

   ///Modifier::Conditional
   Conditional(Conditional),

   ///Modifier::State
   State(State),
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
