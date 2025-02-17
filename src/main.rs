use application::Application;

fn main() {
    let mut app  = Application::new();
    app.run();
}

mod data_collection;
mod models;
mod application;
mod enums;
mod traits;