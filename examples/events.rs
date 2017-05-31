#[macro_use(with_assets)]
extern crate Lattice;
use Lattice::window::{Window};
use Lattice::view::{View, Text};
use std::boxed::*;

fn main() {
    let mut w = Window::new("Premadeath")
              .set_fullscreen(true);
    with_assets!(w);
    w.start(|events| {
       let mut v = View::new();
       v.append(Text::new("assets/Macondo-Regular.ttf", "hover text")
               .always(|e, i| { i.shadow([0,0,0,0], [0.0, 0.0, 0.0, 0.0]); })
               .hovered(|e, i| { i.shadow([-3, -3, 3, 3], [0.4, 0.4, 0.4, 1.0]); })
               .color([0.4, 0.4, 1.0, 1.0])
               .scale(2.0, "em")
               .width(25.0, "%")
               .translate_x(150.0, "px")
               .translate_y(150.0, "px"));
       let mut text_clicked = false;
       v.append(Text::new("assets/Macondo-Regular.ttf", "click text")
               .clicked(move |e, i| { text_clicked = true; })
               .always(move |e, i| { if text_clicked { i.shadow([-3, -3, 3, 3], [0.4, 0.4, 0.4, 1.0]); }  })
               .color([1.0, 0.4, 0.4, 1.0])
               .scale(3.0, "em")
               .width(40.0, "%")
               .align("right")
               .translate_x(50.0, "%")
               .translate_y(30.0, "%"));
       v
    });
}
