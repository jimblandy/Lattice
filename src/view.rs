pub struct Translate {
   dx: f64,
   dy: f64,
   dirty: bool
}
impl Translate {
   pub fn new(dx: f64, dy: f64) -> Translate {
      Translate { dx:dx, dy:dy,  dirty: true }
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
      self.dirty = true;
   }
   pub fn dy(&self) -> f64 {
      self.dy
   }
   pub fn set_dy(&mut self, dy: f64) {
      self.dy = dy;
      self.dirty = true;
   }
   pub fn clean(&mut self) {
      self.dirty = false
   }
}

pub struct Font {
   name: String,
   modifiers: Vec<Modifier>,
   dirty: bool
}
impl Font {
   pub fn new(name: String) -> Font {
      Font { name: name, modifiers:Vec::new(), dirty: true }
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
      self.dirty = true;
   }
   pub fn clean(&mut self) {
      self.dirty = false
   }
}

pub struct Image {
   name: String,
   modifiers: Vec<Modifier>,
   dirty: bool
}
impl Image {
   pub fn new(name: String) -> Image {
      Image { name: name, modifiers:Vec::new(), dirty: true }
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
      self.dirty = true;
   }
   pub fn clean(&mut self) {
      self.dirty = false
   }
}

pub struct Text {
   content: String,
   modifiers: Vec<Modifier>,
   dirty: bool
}
impl Text {
   pub fn new(cs: String) -> Text {
      Text { content: cs, modifiers:Vec::new(), dirty: true }
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
      self.dirty = true;
   }
   pub fn clean(&mut self) {
      self.dirty = false
   }
}

pub struct Rectangle {
   height: f64,
   width: f64,
   modifiers: Vec<Modifier>,
   dirty: bool
}
impl Rectangle {
   pub fn new(w: f64, h: f64) -> Rectangle {
      Rectangle { width:w, height:h, modifiers:Vec::new(), dirty:true }
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
      self.dirty = true;
   }
   pub fn set_height(&mut self, h: f64) {
      self.height = h;
      self.dirty = true;
   }
   pub fn clean(&mut self) {
      self.dirty = false;
   }
}

pub struct Square {
   width: f64,
   modifiers: Vec<Modifier>,
   dirty: bool
}
impl Square {
   pub fn new(w: f64) -> Square {
      Square { width:w, modifiers:Vec::new(), dirty:true }
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
      self.dirty = true;
   }
   pub fn clean(&mut self) {
      self.dirty = false;
   }
}

pub struct Circle {
   radius: f64,
   modifiers: Vec<Modifier>,
   dirty: bool
}
impl Circle {
   pub fn new(r: f64) -> Circle {
      Circle { radius:r, modifiers:Vec::new(), dirty:true }
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
      self.dirty = true;
   }
   pub fn clean(&mut self) {
      self.dirty = false;
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
   Nul
}
