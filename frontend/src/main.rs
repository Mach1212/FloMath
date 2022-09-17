use zoon::start_app;

mod app;
mod lib;

fn main() {
    start_app("app", app::view::root);
}
