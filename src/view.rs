pub struct Square {
   width: f64,
   dirty: bool
}
impl Square {
   pub fn new(w: f64) -> Square {
      Square { width:w, dirty:true }
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
   dirty: bool
}
impl Circle {
   pub fn new(r: f64) -> Circle {
      Circle { radius:r, dirty:true }
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
   Square(Square),
   Circle(Circle),
}
