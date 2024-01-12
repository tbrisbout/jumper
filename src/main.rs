use bevy::prelude::App;
use bevy::prelude::Update;

fn main() {
    App::new()
        .add_systems(Update, greet)
        .run();
}

fn greet() {
    println!("hello!");
}
