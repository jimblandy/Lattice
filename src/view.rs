pub struct SizeWidthDynamic {
   ratio: f64,
   unit: String,
}
impl SizeWidthDynamic {
   pub fn new(ratio: f64, unit: String) -> SizeWidthDynamic {
      SizeWidthDynamic { ratio:ratio, unit:unit }
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
   pub fn ratio(&self) -> f64 {
      self.ratio
   }
   pub fn set_ratio(&mut self, ratio: f64) {
      self.ratio = ratio
   }
   pub fn unit(&self) -> &String {
      &self.unit
   }
   pub fn set_unit(&mut self, unit: String) {
      self.unit = unit;
   }
}

pub struct SizeHeightDynamic {
   ratio: f64,
   unit: String,
}
impl SizeHeightDynamic {
   pub fn new(ratio: f64, unit: String) -> SizeHeightDynamic {
      SizeHeightDynamic { ratio:ratio, unit:unit }
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
   pub fn ratio(&self) -> f64 {
      self.ratio
   }
   pub fn set_ratio(&mut self, ratio: f64) {
      self.ratio = ratio
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
   pub fn new(dx: f64, dy: f64) -> Translate {
      Translate { dx:dx, dy:dy }
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
   pub fn new(name: String) -> Font {
      Font { name: name, modifiers:Vec::new() }
   }
   pub fn name(&self) -> &String {
      &self.name
   }
   pub fn translate(&mut self, dx: f64, dy: f64) {
      self.modifiers.retain(Translate::is_not);
      self.modifiers.push(Modifier::Translate(Translate::new(dx, dy)));
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
   pub fn new(name: String) -> Image {
      Image { name: name, modifiers:Vec::new() }
   }
   pub fn translate(&mut self, dx: f64, dy: f64) {
      self.modifiers.retain(Translate::is_not);
      self.modifiers.push(Modifier::Translate(Translate::new(dx, dy)));
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
   pub fn new(cs: String) -> Text {
      Text { content: cs, modifiers:Vec::new() }
   }
   pub fn translate(&mut self, dx: f64, dy: f64) {
      self.modifiers.retain(Translate::is_not);
      self.modifiers.push(Modifier::Translate(Translate::new(dx, dy)));
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
   pub fn new(w: f64, h: f64) -> Rectangle {
      Rectangle { width:w, height:h, modifiers:Vec::new() }
   }
   pub fn translate(&mut self, dx: f64, dy: f64) {
      self.modifiers.retain(Translate::is_not);
      self.modifiers.push(Modifier::Translate(Translate::new(dx, dy)));
   }
   pub fn width(&self) -> f64 {
      self.width
   }
   pub fn height(&self) -> f64 {
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
   pub fn new(w: f64) -> Square {
      Square { width:w, modifiers:Vec::new() }
   }
   pub fn translate(&mut self, dx: f64, dy: f64) {
      self.modifiers.retain(Translate::is_not);
      self.modifiers.push(Modifier::Translate(Translate::new(dx, dy)));
   }
   pub fn width(&self) -> f64 {
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
   pub fn new(r: f64) -> Circle {
      Circle { radius:r, modifiers:Vec::new() }
   }
   pub fn translate(&mut self, dx: f64, dy: f64) {
      self.modifiers.retain(Translate::is_not);
      self.modifiers.push(Modifier::Translate(Translate::new(dx, dy)));
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
pub enum Modifier {
   Translate(Translate),
   SizeWidthDynamic(SizeWidthDynamic),
   SizeHeightDynamic(SizeHeightDynamic),
}

pub struct View {
}
