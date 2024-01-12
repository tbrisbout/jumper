use bevy::prelude::{App, Startup, Update, Query};
use bevy::ecs::component::Component;
use bevy::ecs::system::Commands;
use bevy::ecs::query::With;

fn main() {
    App::new()
        .add_systems(Startup, add_people)
        .add_systems(Update, (hello, greet_people))
        .run();
}

fn hello() {
    println!("hello!");
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("hello {}!", name.0);
    }
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Foo Bar".to_string())));
    commands.spawn((Person, Name("Bar Baz".to_string())));
    commands.spawn((Person, Name("Baz Qux".to_string())));
}
