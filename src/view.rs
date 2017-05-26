pub struct SizeWidthDynamic {
   scalar: f64,
   unit: String,
}
impl SizeWidthDynamic {
   pub fn new(scalar: f64, unit: String) -> Modifier {
      Modifier::SizeWidthDynamic(SizeWidthDynamic { scalar:scalar, unit:unit })
   }
   pub fn is(m: &Modifier) -> bool {
      match *m {
         Modifier::SizeWidthDynamic(_) => { true }
         _ => { false }
      }
   }
   pub fn is_not(m: &Modifier) -> bool {
      !Translate::is(m)
   }
   pub fn scalar(&self) -> f64 {
      self.scalar
   }
   pub fn set_scalar(&mut self, scalar: f64) {
      self.scalar = scalar
   }
   pub fn unit(&self) -> &String {
      &self.unit
   }
   pub fn set_unit(&mut self, unit: String) {
      self.unit = unit;
   }
}

pub struct SizeHeightDynamic {
   scalar: f64,
   unit: String,
}
impl SizeHeightDynamic {
   pub fn new(scalar: f64, unit: String) -> Modifier {
      Modifier::SizeHeightDynamic(SizeHeightDynamic { scalar:scalar, unit:unit })
   }
   pub fn is(m: &Modifier) -> bool {
      match *m {
         Modifier::SizeHeightDynamic(_) => { true }
         _ => { false }
      }
   }
   pub fn is_not(m: &Modifier) -> bool {
      !Translate::is(m)
   }
   pub fn scalar(&self) -> f64 {
      self.scalar
   }
   pub fn set_scalar(&mut self, scalar: f64) {
      self.scalar = scalar
   }
   pub fn unit(&self) -> &String {
      &self.unit
   }
   pub fn set_unit(&mut self, unit: String) {
      self.unit = unit;
   }
}

pub struct Translate {
   dx: f64,
   dy: f64,
}
impl Translate {
   pub fn new(dx: f64, dy: f64) -> Modifier {
      Modifier::Translate(Translate { dx:dx, dy:dy })
   }
   pub fn is(m: &Modifier) -> bool {
      match *m {
         Modifier::Translate(_) => { true }
         _ => { false }
      }
   }
   pub fn is_not(m: &Modifier) -> bool {
      !Translate::is(m)
   }
   pub fn dx(&self) -> f64 {
      self.dx
   }
   pub fn set_dx(&mut self, dx: f64) {
      self.dx = dx;
   }
   pub fn dy(&self) -> f64 {
      self.dy
   }
   pub fn set_dy(&mut self, dy: f64) {
      self.dy = dy;
   }
}

pub struct Font {
   name: String,
   modifiers: Vec<Modifier>,
}
impl Font {
   pub fn new(name: String) -> Component {
      Component::Font(Font { name: name, modifiers:Vec::new() })
   }
   pub fn name(&self) -> &String {
      &self.name
   }
   pub fn modifiers(&self) -> &Vec<Modifier> {
      &self.modifiers
   }
   pub fn translate(&mut self, dx: f64, dy: f64) -> &mut Font {
      self.modifiers.push(Translate::new(dx, dy));
      self
   }
   pub fn width(&mut self, scalar: f64, unit: &str) -> &mut Font {
      self.modifiers.push(SizeWidthDynamic::new(scalar, unit.to_owned()));
      self
   }
   pub fn height(&mut self, scalar: f64, unit: &str) -> &mut Font {
      self.modifiers.push(SizeHeightDynamic::new(scalar, unit.to_owned()));
      self
   }
   pub fn set_name(&mut self, name: String) {
      self.name = name;
   }
}

pub struct Image {
   name: String,
   modifiers: Vec<Modifier>,
}
impl Image {
   pub fn new(name: &str) -> Component {
      Component::Image(Image { name: name.to_owned(), modifiers:Vec::new() })
   }
   pub fn modifiers(&self) -> &Vec<Modifier> {
      &self.modifiers
   }
   pub fn translate(&mut self, dx: f64, dy: f64) -> &mut Image {
      self.modifiers.push(Translate::new(dx, dy));
      self
   }
   pub fn width(&mut self, scalar: f64, unit: &str) -> &mut Image {
      self.modifiers.push(SizeWidthDynamic::new(scalar, unit.to_owned()));
      self
   }
   pub fn height(&mut self, scalar: f64, unit: &str) -> &mut Image {
      self.modifiers.push(SizeHeightDynamic::new(scalar, unit.to_owned()));
      self
   }
   pub fn name(&self) -> &String {
      &self.name
   }
   pub fn set_name(&mut self, name: String) {
      self.name = name;
   }
}

pub struct Text {
   content: String,
   modifiers: Vec<Modifier>,
}
impl Text {
   pub fn new(cs: String) -> Component {
      Component::Text(Text { content: cs, modifiers:Vec::new() })
   }
   pub fn translate(&mut self, dx: f64, dy: f64) -> &mut Text {
      self.modifiers.push(Translate::new(dx, dy));
      self
   }
   pub fn width(&mut self, scalar: f64, unit: &str) -> &mut Text {
      self.modifiers.push(SizeWidthDynamic::new(scalar, unit.to_owned()));
      self
   }
   pub fn height(&mut self, scalar: f64, unit: &str) -> &mut Text {
      self.modifiers.push(SizeHeightDynamic::new(scalar, unit.to_owned()));
      self
   }
   pub fn content(&self) -> &String {
      &self.content
   }
   pub fn set_content(&mut self, content: String) {
      self.content = content;
   }
}

pub struct Rectangle {
   height: f64,
   width: f64,
   modifiers: Vec<Modifier>,
}
impl Rectangle {
   pub fn new(w: f64, h: f64) -> Component {
      Component::Rectangle(Rectangle { width:w, height:h, modifiers:Vec::new() })
   }
   pub fn translate(&mut self, dx: f64, dy: f64) -> &mut Rectangle {
      self.modifiers.push(Translate::new(dx, dy));
      self
   }
   pub fn width(&mut self, scalar: f64, unit: &str) -> &mut Rectangle {
      self.modifiers.push(SizeWidthDynamic::new(scalar, unit.to_owned()));
      self
   }
   pub fn height(&mut self, scalar: f64, unit: &str) -> &mut Rectangle {
      self.modifiers.push(SizeHeightDynamic::new(scalar, unit.to_owned()));
      self
   }
   pub fn get_width(&self) -> f64 {
      self.width
   }
   pub fn get_height(&self) -> f64 {
      self.height
   }
   pub fn set_width(&mut self, w: f64) {
      self.width = w;
   }
   pub fn set_height(&mut self, h: f64) {
      self.height = h;
   }
}

pub struct Square {
   width: f64,
   modifiers: Vec<Modifier>,
}
impl Square {
   pub fn new(w: f64) -> Component {
      Component::Square(Square { width:w, modifiers:Vec::new() })
   }
   pub fn translate(&mut self, dx: f64, dy: f64) {
      self.modifiers.push(Translate::new(dx, dy));
   }
   pub fn width(&mut self, scalar: f64, unit: &str) -> &mut Square {
      self.modifiers.push(SizeWidthDynamic::new(scalar, unit.to_owned()));
      self
   }
   pub fn height(&mut self, scalar: f64, unit: &str) -> &mut Square {
      self.modifiers.push(SizeHeightDynamic::new(scalar, unit.to_owned()));
      self
   }
   pub fn get_width(&self) -> f64 {
      self.width
   }
   pub fn set_width(&mut self, w: f64) {
      self.width = w;
   }
}

pub struct Circle {
   radius: f64,
   modifiers: Vec<Modifier>,
}
impl Circle {
   pub fn new(r: f64) -> Component {
      Component::Circle(Circle { radius:r, modifiers:Vec::new() })
   }
   pub fn translate(&mut self, dx: f64, dy: f64) {
      self.modifiers.push(Translate::new(dx, dy));
   }
   pub fn width(&mut self, scalar: f64, unit: &str) -> &mut Circle {
      self.modifiers.push(SizeWidthDynamic::new(scalar, unit.to_owned()));
      self
   }
   pub fn height(&mut self, scalar: f64, unit: &str) -> &mut Circle {
      self.modifiers.push(SizeHeightDynamic::new(scalar, unit.to_owned()));
      self
   }
   pub fn radius(&self) -> f64 {
      self.radius
   }
   pub fn set_radius(&mut self, r: f64) {
      self.radius = r;
   }
}

pub enum Component {
   Modifier(Modifier),
   Image(Image),
   Font(Font),
   Text(Text),
   Rectangle(Rectangle),
   Square(Square),
   Circle(Circle),
}
impl Component {
   pub fn width(&mut self, scalar: f64, unit: &str) -> &mut Component {
      match *self {
         Component::Image(ref mut m) => { m.width(scalar,unit); }
         Component::Font(ref mut m) => { m.width(scalar,unit); }
         Component::Text(ref mut m) => { m.width(scalar,unit); }
         Component::Rectangle(ref mut m) => { m.width(scalar,unit); }
         Component::Square(ref mut m) => { m.width(scalar,unit); }
         Component::Circle(ref mut m) => { m.width(scalar,unit); }
         _ => {}
      }; self
   }
   pub fn height(&mut self, scalar: f64, unit: &str) -> &mut Component {
      match *self {
         Component::Image(ref mut m) => { m.height(scalar,unit); }
         Component::Font(ref mut m) => { m.height(scalar,unit); }
         Component::Text(ref mut m) => { m.height(scalar,unit); }
         Component::Rectangle(ref mut m) => { m.height(scalar,unit); }
         Component::Square(ref mut m) => { m.height(scalar,unit); }
         Component::Circle(ref mut m) => { m.height(scalar,unit); }
         _ => {}
      }; self
   }
}

pub enum Modifier {
   Translate(Translate),
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
