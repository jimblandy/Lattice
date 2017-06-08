use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

pub fn main() {
   let example_assets = [
      "assets/background.png",
      "assets/handcloth.png",
      "assets/Macondo-Regular.ttf"];
   let mut f = File::create(Path::new("examples").join("assets.in")).unwrap();
   f.write_all(b"[");
   if cfg!(windows) {
      for ast in example_assets.iter() {
         f.write_all( format!("(\"{}\",include_bytes!(\"{}\").to_vec()),",ast,ast.to_string().replace("/","\\\\")).as_bytes() );
      }
   } else {
      for ast in example_assets.iter() {
         f.write_all( format!("(\"{}\",include_bytes!(\"{}\").to_vec()),",ast,ast.to_string()).as_bytes() );
      }
   }
   f.write_all(b"]");
}
