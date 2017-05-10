pub struct Square {
   pub width: f64
}
impl Square {
   pub fn new(w: f64) -> Square {
      Square { width:w }
   }
}

pub struct Circle {
   pub radius: f64
}
impl Circle {
   pub fn new(r: f64) -> Circle {
      Circle { radius:r }
   }
}

pub enum Component {
   Square(Square),
   Circle(Circle),
}
