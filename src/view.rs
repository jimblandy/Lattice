use ::events::Events;

pub struct Width {
   pub scalar: f64,
   pub unit: String,
}
impl Width {
   pub fn new(scalar: f64, unit: String) -> Modifier {
      Modifier::Width(Width { scalar:scalar, unit:unit })
   }
}

pub struct Height {
   pub scalar: f64,
   pub unit: String,
}
impl Height {
   pub fn new(scalar: f64, unit: String) -> Modifier {
      Modifier::Height(Height { scalar:scalar, unit:unit })
   }
}

pub struct TranslateX {
   pub scalar: f64,
   pub unit: String,
}
impl TranslateX {
   pub fn new(scalar: f64, unit: String) -> Modifier {
      Modifier::TranslateX(TranslateX { scalar:scalar, unit:unit })
   }
}

pub struct TranslateY {
   pub scalar: f64,
   pub unit: String,
}
impl TranslateY {
   pub fn new(scalar: f64, unit: String) -> Modifier {
      Modifier::TranslateY(TranslateY { scalar:scalar, unit:unit })
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
   pub events: Vec<(Event, Box<FnMut(&mut Events, &mut MutableComponent)>)>,
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
   pub events: Vec<(Event, Box<FnMut(&mut Events, &mut MutableComponent)>)>,
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
   pub events: Vec<(Event, Box<FnMut(&mut Events, &mut MutableComponent)>)>,
}
impl Rectangle {
   pub fn new(w: f64, wunit: &str, h: f64, hunit: &str) -> Component {
      Component::Rectangle(Rectangle { width:w, wunit:wunit.to_owned(), height:h, hunit:hunit.to_owned(),
                                       modifiers:Vec::new(), events:Vec::new() })
   }
}

pub enum Component {
   Modifier(Modifier),
   Image(Image),
   Text(Text),
   Rectangle(Rectangle),
}
impl Component {
   pub fn width(mut self, scalar: f64, unit: &str) -> Component {
      match self {
         Component::Text(ref mut m) => { push_modifier!(m.modifiers, Width, (scalar, unit)) }
         Component::Image(ref mut m) => { push_modifier!(m.modifiers, Width, (scalar, unit)) }
         Component::Rectangle(ref mut m) => { push_modifier!(m.modifiers, Width, (scalar, unit)) }
         _ => {}
      }; self
   }
   pub fn height(mut self, scalar: f64, unit: &str) -> Component {
      match self {
         Component::Text(ref mut m) => { push_modifier!(m.modifiers, Height, (scalar, unit)) }
         Component::Image(ref mut m) => { push_modifier!(m.modifiers, Height, (scalar, unit)) }
         Component::Rectangle(ref mut m) => { push_modifier!(m.modifiers, Height, (scalar, unit)) }
         _ => {}
      }; self
   }
   pub fn translate_x(mut self, scalar: f64, unit: &str) -> Component {
      match self {
         Component::Text(ref mut m) => { push_modifier!(m.modifiers, TranslateX, (scalar, unit)) }
         Component::Image(ref mut m) => { push_modifier!(m.modifiers, TranslateX, (scalar, unit)) }
         Component::Rectangle(ref mut m) => { push_modifier!(m.modifiers, TranslateX, (scalar, unit)) }
         _ => {}
      }; self
   }
   pub fn translate_y(mut self, scalar: f64, unit: &str) -> Component {
      match self {
         Component::Text(ref mut m) => { push_modifier!(m.modifiers, TranslateY, (scalar, unit)) }
         Component::Image(ref mut m) => { push_modifier!(m.modifiers, TranslateY, (scalar, unit)) }
         Component::Rectangle(ref mut m) => { push_modifier!(m.modifiers, TranslateY, (scalar, unit)) }
         _ => {}
      }; self
   }
   pub fn color(mut self, rgba: [f64; 4]) -> Component {
      match self {
         Component::Text(ref mut m) => { push_modifier!(m.modifiers, Color, (rgba)) }
         Component::Image(ref mut m) => { push_modifier!(m.modifiers, Color, (rgba)) }
         Component::Rectangle(ref mut m) => { push_modifier!(m.modifiers, Color, (rgba)) }
         _ => {}
      }; self
   }
   pub fn scale(mut self, scale: f64, unit: &str) -> Component {
      match self {
         Component::Text(ref mut m) => { push_modifier!(m.modifiers, Scale, (scalar, unit)) }
         Component::Image(ref mut m) => { push_modifier!(m.modifiers, Scale, (scalar, unit)) }
         Component::Rectangle(ref mut m) => { push_modifier!(m.modifiers, Scale, (scalar, unit)) }
         _ => {}
      }; self
   }
   pub fn align(mut self, align: &str) -> Component {
      match self {
         Component::Text(ref mut m) => { m.align = align; }
         _ => {}
      }; self
   }
   pub fn shadow(mut self, d: [i64; 4], c: [f64; 4]) -> Component {
      match self {
         Component::Text(ref mut m) => { push_modifier!(m.modifiers, Shadow) }
         Component::Image(ref mut m) => { push_modifier!(m.modifiers, Shadow) }
         Component::Rectangle(ref mut m) => { push_modifier!(m.modifiers, Shadow) }
         _ => {}
      }; self
   }
   pub fn clicked<F>(mut self, f: F) -> Component 
          where F: 'static + FnMut(&mut Events, &mut MutableComponent) {
      match self {
         Component::Text(ref mut m) => { push_event!(m.events, Clicked, f); }
         Component::Image(ref mut m) => { push_event!(m.events, Clicked, f); }
         Component::Rectangle(ref mut m) => { push_event!(m.events, Clicked, f); }
         _ => {}
      }; self
   }
   pub fn hovered<F>(mut self, f: F) -> Component 
          where F: 'static + FnMut(&mut Events, &mut MutableComponent) {
      match self {
         Component::Text(ref mut m) => { push_event!(m.events, Hovered, f); }
         Component::Image(ref mut m) => { push_event!(m.events, Hovered, f); }
         Component::Rectangle(ref mut m) => { push_event!(m.events, Hovered, f); }
         _ => {}
      }; self
   }
   pub fn always<F>(mut self, f: F) -> Component 
          where F: 'static + FnMut(&mut Events, &mut MutableComponent) {
      match self {
         Component::Text(ref mut m) => { push_event!(m.events, Always, f); }
         Component::Image(ref mut m) => { push_event!(m.events, Always, f); }
         Component::Rectangle(ref mut m) => { push_event!(m.events, Always, f); }
         _ => {}
      }; self
   }
}

pub trait MutableComponent {
   fn shadow(&mut self, [i64; 4], [f64; 4]);
}
impl MutableComponent for Component {
   fn shadow(&mut self, d: [i64; 4], c: [f64; 4]) {
      match *self {
         Component::Text(ref mut m) => { m.modifiers.push(Shadow::new(d,c)) }
         _ => {}
      }
   }
}

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
