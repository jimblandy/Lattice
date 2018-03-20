#[macro_use(with_assets)]
extern crate Lattice;
use Lattice::window::{Window};
use Lattice::view::{View, Text};
use std::cell::Cell;
use std::sync::Mutex;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref TEXT_CLICKED: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));
    static ref LEFT_HOVERED: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));
}

fn main() {
    let mut w = Window::new("Premadeath").set_fullscreen(true);
    with_assets!(w);

    w.start(|_events| {
       let mut v = View::new();

       let lh = LEFT_HOVERED.lock().unwrap();
       let tc = TEXT_CLICKED.lock().unwrap();

       v.append(Text::new("assets/Macondo-Regular.ttf", "hover text")
               .shadow((if lh.get() {[-3,-3,3,3]} else {[0,0,0,0]}),
                       (if lh.get() {[0.8,0.8,0.8,0.8]} else {[0.0, 0.0, 0.0, 0.0]}))
               .hovered(|_e| {
                   let lh = LEFT_HOVERED.lock().unwrap();
                   lh.set(true);
               })
               .color([0.4, 0.4, 1.0, 1.0])
               .scale(2.0, "em")
               .width(25.0, "%")
               .translate_x(150.0, "px")
               .translate_y(150.0, "px"));
       lh.set(false);

       v.append(Text::new("assets/Macondo-Regular.ttf", "click text")
               .shadow((if tc.get() {[-3,-3,3,3]} else {[0,0,0,0]}),
                       (if tc.get() {[0.8,0.8,0.8,1.0]} else {[0.0,0.0,0.0,0.0]}))
               .clicked(move |_e| {
                  let tc = TEXT_CLICKED.lock().unwrap();
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
