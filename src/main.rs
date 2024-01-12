use bevy::prelude::{App, Startup, Update, Query, DefaultPlugins, Plugin};
use bevy::ecs::component::Component;
use bevy::ecs::system::{Commands, Resource, Res, ResMut};
use bevy::ecs::query::With;
use bevy::time::{Time, Timer, TimerMode};

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
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(Startup, add_people)
            .add_systems(Update, greet_people);
    }
}

#[derive(Resource)]
struct GreetTimer(Timer);

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    // update the timer with the time elapsed since the last update
    // if that caused the timer to finish, say hello
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}
