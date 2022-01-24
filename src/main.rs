mod app;
use app::*;

mod components;

#[cfg(feature = "console_log")]
fn init_log() {
    use log::Level;
    console_log::init_with_level(Level::Trace).expect("error initializing log");
}

#[cfg(not(feature = "console_log"))]
fn init_log() {}

fn main() {
    init_log();
    yew::start_app::<App>();
}
