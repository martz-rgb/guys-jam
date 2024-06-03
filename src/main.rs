use bevy::app;

fn hello_world() {
    println!("hello, world!")
}

fn main() {
    let mut app = app::App::new();

    app.add_systems(app::Update, hello_world);

    app.run();
}
