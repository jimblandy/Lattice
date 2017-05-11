extern crate Lattice;
extern crate LatticeSDL2Driver;

struct GlobalState {
   pub text: String
}

enum Event {
   ClickUp,
   ClickDown,
   PressLeft,
   PressRight
}

fn react(model: GlobalState, events: Vec<Lattice::Event>) -> Vec<Event> {
   
}
fn control(ref mut model: GlobalState, events: Vec<Event>) {
   
}
fn render(model: GlobalState) -> Lattice::Component {
}

fn main() {
   let mut state = GlobalState::new();
   let rview = render(state, view);
   LatticeSDL2Driver::start(move |window, es| {
     let es = react(state, es);
     control(state, es);
     let tree = render(state);
     LatticeSDL2Driver::render(tree);
   })
}
