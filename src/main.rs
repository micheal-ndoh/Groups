mod application;
mod data_collection;
mod enums;
mod models;

use application::Application;

fn main() {
    let app = Application::new();
    Application::run();
}