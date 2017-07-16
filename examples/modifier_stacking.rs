#[macro_use(with_assets)]
extern crate Lattice;
use Lattice::window::{Window};
use Lattice::view::{View, Image, Rectangle};

fn main() {
    let mut w = Window::new("Premadeath")
              .set_fullscreen(true);
    with_assets!(w);
    w.start(|events| {
       let mut v = View::new();

       //A bounding box 75% of the smaller of height/width, split into 10 pieces horizontally and vertically
       let fraction = 75.0 / 10.0;

       //Different colors for different tiles
       let colors = [
          [1.0, 0.4, 0.4, 1.0],
          [0.4, 1.0, 0.4, 1.0],
          [0.4, 0.4, 1.0, 1.0],
          [1.0, 1.0, 1.0, 1.0]
       ];

       //center tiles horizontally and vertically
       for x in 0..10 {
       for y in 0..10 {
          v.append(Rectangle::new(fraction, "<%", fraction, "<%")
                 .color( colors[((x+y) % 4) as usize] )

                 //Horizontal or Vertical could be larger, so move to 50% horizontal - half of effective width
                 .translate_x(50.0, "h%")
                 .translate_x(-37.5, "<%")

                 .translate_y(50.0, "v%")
                 .translate_y(-37.5, "<%")

                 //Arrange tiles in "container"
                 .translate_x(fraction*(x as f64), "<%")
                 .translate_y(fraction*(y as f64), "<%")

          );
       }}

       v
    });
}
