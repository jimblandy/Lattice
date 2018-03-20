#[macro_use(with_assets)]
extern crate Lattice;
use Lattice::window::{Window};
use Lattice::view::{View, Image};

fn main() {
    let mut w = Window::new("Premadeath")
              .set_fullscreen(true);
    with_assets!(w);
    w.start(|_events| {
       let mut v = View::new();
       let i = Image::new("assets/background.png")
              .height(100.0, "%")
              .width(100.0, "%");
       v.append(i);
       let i = Image::new("assets/handcloth.png")
              .height(20.0, "%")
              .width(20.0, "%");
       v.append(i);
       let i = Image::new("assets/handcloth.png")
              .height(40.0, "%")
              .width(40.0, "%")
              .rotate(40.0, "degree");
       v.append(i);
       v
    });
}
