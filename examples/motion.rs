#[macro_use(with_assets)]
extern crate Lattice;
use Lattice::window::{Window};
use Lattice::view::{View, Text, Image};

fn main() {
    let mut w = Window::new("Premadeath")
              .set_fullscreen(true);
    with_assets!(w);
    w.start(|events| {
       let mut v = View::new();
       v.append(Text::new("assets/Macondo-Regular.ttf",
                format!("{:.2} seconds since start", events.time_elapsed).as_str())
               .scale(3.0, "em"));

       let cycle = events.time_elapsed % 4.0;
       let cycle_x = if cycle<1.0 { cycle } else if cycle<2.0 { 1.0 } else if cycle<3.0 { 1.0-(cycle-2.0) } else { 0.0 };
       let cycle_y = if cycle<1.0 { 0.0 } else if cycle<2.0 { cycle-1.0 } else if cycle<3.0 { 1.0 } else { 1.0-(cycle-3.0) };
       v.append(Image::new("assets/handcloth.png")
                  .height(20.0, "%")
                  .width(20.0, "%")
                  .translate_x(20.0 + 30.0*cycle_x, "%")
                  .translate_y(20.0 + 30.0*cycle_y, "%"));

       v
    });
}
