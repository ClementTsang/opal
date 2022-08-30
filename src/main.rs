mod app;
use app::*;

mod components;

fn main() {
    #[cfg(feature = "console_log")]
    {
        use log::Level;
        console_log::init_with_level(Level::Trace).expect("error initializing log");
    }
    yew::start_app::<App>();
}
