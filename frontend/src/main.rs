use zoon::start_app;

mod app;

fn main() {
    start_app("app", app::view::root);
}
