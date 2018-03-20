#[macro_use(with_assets)]
extern crate Lattice;
use Lattice::window::{Window};
use Lattice::view::{View, Text, ViewUnit, AlignUnit};

fn main() {
    let mut w = Window::new("Premadeath")
              .set_fullscreen(true);
    with_assets!(w);
    w.start(|_events| {
       let mut v = View::new();
       v.append(Text::new("assets/Macondo-Regular.ttf", "paragraph justified paragraph justified paragraph justified paragraph justified paragraph justified
paragraph justified paragraph justified paragraph justified paragraph justified paragraph justified paragraph justified paragraph justified paragraph justified")
               .color([0.4, 0.4, 1.0, 1.0])
               .shadow([-3,-3,3,3],[1.0,1.0,1.0,1.0])
               .scale(2.0, ViewUnit::Em)
               .width(25.0, ViewUnit::Percent)
               .align(AlignUnit::Justify)
               .translate_x(150.0, ViewUnit::Pixel)
               .translate_y(150.0, ViewUnit::Pixel));
       v.append(Text::new("assets/Macondo-Regular.ttf", "paragraph right aligned paragraph right aligned paragraph right aligned 
paragraph right aligned paragraph right aligned paragraph right aligned paragraph right aligned paragraph right aligned")
               .color([1.0, 0.4, 0.4, 1.0])
               .scale(3.0, ViewUnit::Em)
               .width(40.0, ViewUnit::Percent)
               .align(AlignUnit::Right)
               .translate_x(50.0, ViewUnit::Percent)
               .translate_y(30.0, ViewUnit::Percent));
       v
    });
}
