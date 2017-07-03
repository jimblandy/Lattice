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
       v.append(Image::new("assets/background.png")
                  .always(|e|{ e.set("a","b") })
                  .height(100.0, "%")
                  .width(100.0, "%"));
       
       v.ifappend("a","b",Image::new("assets/handcloth.png")
                  .translate_x(25.0, "h%")
                  .translate_x(25.0, "h%")
                  .height(20.0, "%")
                  .width(20.0, "%"));
       v
    });
}
