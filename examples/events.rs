#[macro_use(with_assets)]
extern crate Lattice;
use Lattice::window::{Window};
use Lattice::view::{View, Text};
use std::rc::Rc;
use std::cell::{RefCell,Cell};
use std::sync::Mutex;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref text_clicked: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));
    static ref left_hovered: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));
}

fn main() {
    let mut w = Window::new("Premadeath").set_fullscreen(true);
    with_assets!(w);

    w.start(|events| {
       let mut v = View::new();

       v.append(Text::new("assets/Macondo-Regular.ttf", "hover text")
               .always(|e, i| {
                  let lh = left_hovered.lock().unwrap();
                  if lh.get() { i.shadow([-3, -3, 3, 3], [0.8, 0.8, 0.8, 1.0]); }
                  else { i.shadow([0, 0, 0, 0], [0.0, 0.0, 0.0, 0.0]); }
                  lh.set(false);
               })
               .hovered(move |e, i| {
                  let lh = left_hovered.lock().unwrap();
                  lh.set(true);
               })
               .color([0.4, 0.4, 1.0, 1.0])
               .scale(2.0, "em")
               .width(25.0, "%")
               .translate_x(150.0, "px")
               .translate_y(150.0, "px"));

       v.append(Text::new("assets/Macondo-Regular.ttf", "click text")
               .always(move |e, i| {
                  let tc = text_clicked.lock().unwrap();
                  if tc.get() { i.shadow([-3, -3, 3, 3], [0.8, 0.8, 0.8, 1.0]); }  
               })
               .clicked(move |e, i| {
                  let tc = text_clicked.lock().unwrap();
                  tc.set(true);
               })
               .color([1.0, 0.4, 0.4, 1.0])
               .scale(3.0, "em")
               .width(40.0, "%")
               .align("right")
               .translate_x(50.0, "%")
               .translate_y(30.0, "%"));

       v
    });
}
