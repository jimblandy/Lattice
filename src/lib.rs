pub mod view;
pub mod events;
pub mod window;
pub mod build;
pub mod text;

#[macro_export]
macro_rules! with_assets {
   ($w: ident) => (
      $w.load_assets(include!("assets.in").to_vec());
   );
}
