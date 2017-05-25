pub mod view;
pub mod events;
pub mod window;

#[macro_export]
macro_rules! with_assets {
    ($w: ident) => (
        $w.load_asset(include!("assets.in"), include_bytes!(include!("assets.in")));
    )
}
