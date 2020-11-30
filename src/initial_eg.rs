use bevy::prelude::*;

fn main() {
    App::build()
    .add_plugins(DefaultPlugins)
    .add_plugin(HelloPlugin)
    .run()
}

struct Person;

struct Name(String);

fn add_people(mut commands: Commands) {
commands
    .spawn((Person, Name("Fred".into())))
    .spawn((Person, Name("Barney".into())))
    .spawn((Person, Name("Wilma".into())));
}

struct GreetTimer(Timer);

#[derive(Debug, Clone)]
struct Counter(u64);

use std::fmt;
impl fmt::Display for Counter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, mut counter: ResMut<Counter>, query: Query<(&Person,&Name)>) {
    // update our timer with the time elapsed since the last update
    timer.0.tick(time.delta_seconds);
    
    if timer.0.finished {
        counter.0 +=1;
        for(_person, name) in query.iter() {
            println!("Round {}", *counter);
            println!("Hello {}!", name.0);
        }
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .add_resource(GreetTimer(Timer::from_seconds(2.0, true)))
        .add_resource(Counter(0))
        .add_startup_system(add_people.system())
        .add_system(greet_people.system());
    }
}
