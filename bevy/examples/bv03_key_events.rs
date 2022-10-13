use bevy::{
	prelude::*,
	window::PresentMode,
};

const TITLE: &str = "Key Events";
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
		.insert_resource(ClearColor(Color::CYAN))
		.add_plugins(DefaultPlugins)
		.add_startup_system(setup)
		.add_system(change_color)
		.run();
}

fn setup(mut commands: Commands) {
	commands.spawn_bundle(Camera2dBundle::default());
}

//TODO: Write a system to change the clear color in response to a keypress
//      * w -> red
//      * a -> green
//      * s -> blue
//      * d -> fuchsia
// Don't forget to add your system to the app in `main()`!

fn change_color(
	input: Res<Input<KeyCode>>,
	mut clear_color: ResMut<ClearColor>
) {
	if input.pressed(KeyCode::A) {
		clear_color.0 = Color::GREEN;
	}

	if input.pressed(KeyCode::D) {
		clear_color.0 = Color::FUCHSIA;
	}

	if input.pressed(KeyCode::W) {
		clear_color.0 = Color::RED;
	}

	if input.pressed(KeyCode::S) {
		clear_color.0 = Color::BLUE;
	}
}
