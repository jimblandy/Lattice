use std;
use std::io::Write;
extern crate glob;
use self::glob::glob;
use std::fs::File;
use std::io::prelude::*;

pub fn with_assets() {
   let mut file = File::create("src/assets.in").expect("file open");
   for entry in glob("src/assets/**/*.png").expect("Failed to read glob pattern") {
      match entry {
         Ok(path) => {
            let path = path.strip_prefix("src").expect("src prefix");
            let path = path.to_str().unwrap();
            let path = format!("\"{}\"\n", path);
            file.write_all( path.as_bytes() ).expect("file write");
         }
         Err(e) => println!("{:?}", e),
      }
   }
}
pub fn with_all() {
   with_assets()
}
