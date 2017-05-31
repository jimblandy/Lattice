#[macro_use(with_assets)]
extern crate Lattice;
use Lattice::window::{Window};
use Lattice::view::{View, Text};

fn main() {
    let mut w = Window::new("Premadeath")
              .set_fullscreen(true);
    with_assets!(w);
    w.start(|events| {
       let mut v = View::new();
       v.append(Text::new("assets/Macondo-Regular.ttf", "paragraph justified paragraph justified paragraph justified paragraph justified paragraph justified
paragraph justified paragraph justified paragraph justified paragraph justified paragraph justified paragraph justified paragraph justified paragraph justified")
               .color([0.4, 0.4, 1.0, 1.0])
               .scale(2.0, "em")
               .width(25.0, "%")
               .align("justify")
               .translate_x(150.0, "px")
               .translate_y(150.0, "px"));
       v.append(Text::new("assets/Macondo-Regular.ttf", "paragraph right aligned paragraph right aligned paragraph right aligned 
paragraph right aligned paragraph right aligned paragraph right aligned paragraph right aligned paragraph right aligned")
               .color([1.0, 0.4, 0.4, 1.0])
               .scale(3.0, "em")
               .width(25.0, "%")
               .align("right")
               .translate_x(60.0, "%")
               .translate_y(60.0, "%"));
       v
    });
}
