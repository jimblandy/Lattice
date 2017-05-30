pub struct SizeWidthDynamic {
   pub scalar: f64,
   pub unit: String,
}
impl SizeWidthDynamic {
   pub fn new(scalar: f64, unit: String) -> Modifier {
      Modifier::SizeWidthDynamic(SizeWidthDynamic { scalar:scalar, unit:unit })
   }
}

pub struct SizeHeightDynamic {
   pub scalar: f64,
   pub unit: String,
}
impl SizeHeightDynamic {
   pub fn new(scalar: f64, unit: String) -> Modifier {
      Modifier::SizeHeightDynamic(SizeHeightDynamic { scalar:scalar, unit:unit })
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

pub struct Color {
   pub rgba: [f64; 4],
}
impl Color {
   pub fn new(rgba: [f64; 4]) -> Modifier {
      Modifier::Color(Color { rgba:rgba })
   }
}

pub struct Scale {
   pub scale: f64,
   pub unit: String
}
impl Scale {
   pub fn new(scale: f64, unit: &str) -> Modifier {
      Modifier::Scale(Scale { scale:scale, unit:unit.to_owned() })
   }
}

pub struct Image {
   pub name: String,
   pub modifiers: Vec<Modifier>,
}
impl Image {
   pub fn new(name: &str) -> Component {
      Component::Image(Image { name: name.to_owned(), modifiers:Vec::new() })
   }
   pub fn modifiers(&self) -> &Vec<Modifier> {
      &self.modifiers
   }
   pub fn translate_x(&mut self, scalar: f64, unit: &str) {
      self.modifiers.push(TranslateX::new(scalar, unit.to_owned()));
   }
   pub fn translate_y(&mut self, scalar: f64, unit: &str) {
      self.modifiers.push(TranslateY::new(scalar, unit.to_owned()));
   }
   pub fn width(&mut self, scalar: f64, unit: &str) {
      self.modifiers.push(SizeWidthDynamic::new(scalar, unit.to_owned()));
   }
   pub fn height(&mut self, scalar: f64, unit: &str) {
      self.modifiers.push(SizeHeightDynamic::new(scalar, unit.to_owned()));
   }
}

pub struct Text {
   pub content: String,
   pub font: String,
   pub align: String,
   pub modifiers: Vec<Modifier>,
}
impl Text {
   pub fn new(font: &str, cs: &str) -> Component {
      Component::Text(Text { font:font.to_owned(), content: cs.to_owned(),
                             align: "left".to_owned(), modifiers:Vec::new() })
   }
   pub fn translate_x(&mut self, scalar: f64, unit: &str) {
      self.modifiers.push(TranslateX::new(scalar, unit.to_owned()));
   }
   pub fn translate_y(&mut self, scalar: f64, unit: &str) {
      self.modifiers.push(TranslateY::new(scalar, unit.to_owned()));
   }
   pub fn width(&mut self, scalar: f64, unit: &str) {
      self.modifiers.push(SizeWidthDynamic::new(scalar, unit.to_owned()));
   }
   pub fn height(&mut self, scalar: f64, unit: &str) {
      self.modifiers.push(SizeHeightDynamic::new(scalar, unit.to_owned()));
   }
   pub fn color(&mut self, rgba: [f64; 4]) {
      self.modifiers.push(Color::new(rgba));
   }
   pub fn scale(&mut self, scale: f64, unit: &str) {
      self.modifiers.push(Scale::new(scale, unit));
   }
   pub fn justify(&mut self) {
      self.align = "justify".to_owned()
   }
}

pub struct Rectangle {
   pub height: f64,
   pub hunit: String,
   pub width: f64,
   pub wunit: String,
   pub modifiers: Vec<Modifier>,
}
impl Rectangle {
   pub fn new(w: f64, wunit: &str, h: f64, hunit: &str) -> Component {
      Component::Rectangle(Rectangle { width:w, wunit:wunit.to_owned(), height:h, hunit:hunit.to_owned(), modifiers:Vec::new() })
   }
   pub fn translate_x(mut self, scalar: f64, unit: String) -> Rectangle {
      self.modifiers.push(TranslateX::new(scalar, unit.to_owned()));
      self
   }
   pub fn translate_y(mut self, scalar: f64, unit: &str) -> Rectangle {
      self.modifiers.push(TranslateY::new(scalar, unit.to_owned()));
      self
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
         Component::Image(ref mut m) => { m.width(scalar,unit); }
         Component::Text(ref mut m) => { m.width(scalar,unit); }
         _ => {}
      }; self
   }
   pub fn height(mut self, scalar: f64, unit: &str) -> Component {
      match self {
         Component::Image(ref mut m) => { m.height(scalar,unit); }
         Component::Text(ref mut m) => { m.height(scalar,unit); }
         _ => {}
      }; self
   }
   pub fn translate_x(mut self, scalar: f64, unit: &str) -> Component {
      match self {
         Component::Image(ref mut m) => { m.translate_x(scalar,unit); }
         Component::Text(ref mut m) => { m.translate_x(scalar,unit); }
         _ => {}
      }; self
   }
   pub fn translate_y(mut self, scalar: f64, unit: &str) -> Component {
      match self {
         Component::Image(ref mut m) => { m.translate_y(scalar,unit); }
         Component::Text(ref mut m) => { m.translate_y(scalar,unit); }
         _ => {}
      }; self
   }
   pub fn color(mut self, rgba: [f64; 4]) -> Component {
      match self {
         Component::Text(ref mut m) => { m.color(rgba); }
         _ => {}
      }; self
   }
   pub fn scale(mut self, scale: f64, unit: &str) -> Component {
      match self {
         Component::Text(ref mut m) => { m.scale(scale, unit); }
         _ => {}
      }; self
   }
   pub fn justify(mut self) -> Component {
      match self {
         Component::Text(ref mut m) => { m.justify(); }
         _ => {}
      }; self
   }
}

pub enum Modifier {
   Color(Color),
   Scale(Scale),
   TranslateX(TranslateX),
   TranslateY(TranslateY),
   SizeWidthDynamic(SizeWidthDynamic),
   SizeHeightDynamic(SizeHeightDynamic),
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
