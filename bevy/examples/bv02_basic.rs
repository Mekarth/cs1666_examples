use bevy::{
	prelude::*,
	window::PresentMode,
};

const TITLE: &str = "bv01 Basic";
const WIN_W: f32 = 1280.;
const WIN_H: f32 = 720.;

fn main() {
	App::new()
		.insert_resource(WindowDescriptor {
			title: String::from(TITLE),
			width: WIN_W,
			height: WIN_H,
			present_mode: PresentMode::Fifo,
			..default()
		})
		.insert_resource(ClearColor(Color::rgb(0., 1., 1.)))
		.add_plugins(DefaultPlugins)
		.add_startup_system(setup)
		.run();
}

fn setup(mut commands: Commands) {
	commands.spawn_bundle(Camera2dBundle::default());
}
