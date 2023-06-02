use bevy::prelude::*;
fn hello_world() {
    println!("Hello, world!");
}
#[derive(Component)]
struct Person;
#[derive(Component)]
struct Name(String);
fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Alice".to_string())));
    commands.spawn((Person, Name("Bob".to_string())));
    commands.spawn((Person, Name("Catt".to_string())));
}
fn greet(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("hello {}", name.0);
    }
}
pub struct HelloPlugin;
impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(add_people)
            .add_system(hello_world)
            .add_system(greet);
    }
}
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .run();
}
