use bevy::prelude::*;

#[derive(Debug, Component)]
struct Pos {
    x: i32,
    y: i32,
}

pub fn hello_world() {
    println!("HELLO world");
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Resource)]
struct GreetTimer(Timer);

pub fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("ONETWO".to_string())));
    commands.spawn((Person, Name("TWOTWO".to_string())));
    commands.spawn((Person, Name("THREETWO".to_string())));
    commands.spawn((Person, Name("FOURTWO".to_string())));
}

fn query_persona(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    query: Query<&Name, With<Person>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("Hello {:?}", name.0);
        }
    }
}

pub struct HelloPlug;

impl Plugin for HelloPlug {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
        app.add_systems(Startup, add_people);
        app.add_systems(Update, query_persona);
    }
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(HelloPlug);

    app.run();
}
