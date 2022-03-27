extern crate wee_alloc;

mod app;
use app::*;

mod components;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn main() {
    #[cfg(feature = "console_log")]
    {
        use log::Level;
        console_log::init_with_level(Level::Trace).expect("error initializing log");
    }
    yew::start_app::<App>();
}
