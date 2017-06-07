use std::io::Write;
extern crate glob;
use self::glob::{glob};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

macro_rules! read_entry {
   ($f: ident, $p: ident) => (
      match $p {
         Ok(path) => {
            let mut in_file = File::open(path.clone()).expect("asset file");
            let mut contents = Vec::new();
            in_file.read_to_end(&mut contents).expect("read file");
            let path = path.strip_prefix("src").expect("src prefix");
            let path = path.to_str().unwrap();
            let path_ref = format!("\"{}\"", path).replace("\\","/");
            let path_bytes = format!("\"{}\"", path).replace("\\","\\\\");
            $f.write_all( format!("({},include_bytes!({}).to_vec()),", path_ref, path_bytes).as_bytes() ).expect("file write");
         }
         Err(e) => println!("{:?}", e),
      }
   );
}

/// Looks for an ./assets directory and copies all resource files into an assets.in file.
pub fn with_assets() {
   let path = Path::new("src").join("assets.in");
   let mut out_file = File::create(path).expect("file open");
   out_file.write_all( b"[" ).expect("file write");
   for entry in glob("src/assets/**/*.png").expect("Failed to read glob pattern") {
      read_entry!(out_file, entry)
   }
   for entry in glob("src/assets/**/*.ttf").expect("Failed to read glob pattern") {
      read_entry!(out_file, entry)
   }
   out_file.write_all( b"]" ).expect("file write");
}

/// Enables all build script features: with_assets.
pub fn with_all() {
   with_assets()
}
