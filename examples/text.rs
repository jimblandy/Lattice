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
               .width(100.0, "px")
               .justify()
               .translate_x(50.0, "px")
               .translate_y(50.0, "px"));
       v.append(Text::new("assets/Macondo-Regular.ttf", "text banner centered")
               .color([1.0, 0.4, 0.4, 1.0])
               .translate_x(1.0, "~")
               .translate_y(-10.0, "%"));
       v
    });
}
