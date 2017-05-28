#[macro_use(with_assets)]
extern crate Lattice;
use Lattice::window::{Window};
use Lattice::view::{View, Image};

fn main() {
    let mut w = Window::new("Premadeath")
              .set_fullscreen(true);
    with_assets!(w);
    w.start(|events| {
       let mut v = View::new();
       let mut i = Image::new("assets/background.png")
                  .height(100.0, "%")
                  .width(100.0, "%");
       v.append(i);
       v
    });
}
