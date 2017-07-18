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

       v.append(Text::new("assets/Macondo-Regular.ttf", "10 em")
               .scale(2.0, "em")
               .color([1.0, 1.0, 1.0, 1.0])
               .translate_x(10.0, "em")
               .translate_y(1.0, "em"));

       v.append(Text::new("assets/Macondo-Regular.ttf", "10 %")
               .scale(2.0, "em")
               .color([1.0, 1.0, 1.0, 1.0])
               .translate_x(10.0, "%")
               .translate_y(4.0, "em"));

       v.append(Text::new("assets/Macondo-Regular.ttf", "10 h% (horizontal %)")
               .scale(2.0, "em")
               .color([1.0, 1.0, 1.0, 1.0])
               .translate_x(10.0, "h%")
               .translate_y(7.0, "em"));

       v.append(Text::new("assets/Macondo-Regular.ttf", "10 v% (vertical %)")
               .scale(2.0, "em")
               .color([1.0, 1.0, 1.0, 1.0])
               .translate_x(10.0, "v%")
               .translate_y(10.0, "em"));

       v.append(Text::new("assets/Macondo-Regular.ttf", "10 <% (lesser percent)")
               .scale(2.0, "em")
               .color([1.0, 1.0, 1.0, 1.0])
               .translate_x(10.0, "<%")
               .translate_y(13.0, "em"));

       v.append(Text::new("assets/Macondo-Regular.ttf", "10 >% (greater percent)")
               .scale(2.0, "em")
               .color([1.0, 1.0, 1.0, 1.0])
               .translate_x(10.0, ">%")
               .translate_y(16.0, "em"));

       v.append(Text::new("assets/Macondo-Regular.ttf", "0 px")
               .scale(2.0, "em")
               .color([1.0, 1.0, 1.0, 1.0])
               .translate_x(0.0, "px")
               .translate_y(19.0, "em"));

       v.append(Text::new("assets/Macondo-Regular.ttf", "10 px")
               .scale(2.0, "em")
               .color([1.0, 1.0, 1.0, 1.0])
               .translate_x(10.0, "px")
               .translate_y(22.0, "em"));

       v
    });
}
