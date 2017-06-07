use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

pub fn main() {
   let example_assets = [
      "assets/background.png",
      "assets/handcloth.png",
      "assets/Macondo-Regular.ttf"];
   let mut f = File::create(Path::new("examples").join("assets.in")).unwrap();
   if cfg!(windows) {
      f.write_all(b"Hello Windows");
   } else {
      f.write_all(b"Hello Other");
   }
}
