use std;
use std::io::Write;
extern crate glob;
use self::glob::glob;
use std::fs::File;
use std::io::prelude::*;

pub fn with_assets() {
   let mut out_file = File::create("src/assets.in").expect("file open");
   out_file.write_all( b"[" ).expect("file write");
   for entry in glob("src/assets/**/*.png").expect("Failed to read glob pattern") {
      match entry {
         Ok(path) => {
            let mut in_file = File::open(path.clone()).expect("asset file");
            let mut contents = Vec::new();
            in_file.read_to_end(&mut contents).expect("read file");
            let path = path.strip_prefix("src").expect("src prefix");
            let path = path.to_str().unwrap();
            let path = format!("\"{}\"", path);
            out_file.write_all( format!("({},include_bytes!({}).to_vec()),", path, path).as_bytes() ).expect("file write");
         }
         Err(e) => println!("{:?}", e),
      }
   }
   out_file.write_all( b"]" ).expect("file write");
}
pub fn with_all() {
   with_assets()
}
