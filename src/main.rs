use bevy::prelude::{App, Startup, Update, Query, DefaultPlugins, Plugin};
use bevy::ecs::component::Component;
use bevy::ecs::system::Commands;
use bevy::ecs::query::With;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, HelloPlugin))
        .run();
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

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Foo Bar" {
            name.0 = "Foo Barbar".to_string();
            break;
        }
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_people)
            .add_systems(Update, greet_people);
    }
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("hello {}!", name.0);
    }
}

