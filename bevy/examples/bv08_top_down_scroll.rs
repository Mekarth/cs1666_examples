use bevy::{
	window::PresentMode,
	prelude::*,	
};
use std::convert::From;

const TITLE: &str = "Top-down Scroll";
const WIN_W: f32 = 1280.;
const WIN_H: f32 = 720.;

const PLAYER_SPEED: f32 = 500.;
const ACCEL_RATE: f32 = 5000.;

const TILE_SIZE: f32 = 100.;

const LEVEL_W: f32 = 1920.;
const LEVEL_H: f32 = 1080.;

enum PlayerType {
	Bird,
	Plane,
	UFO,
	Helicopter,
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Brick;

#[derive(Component)]
struct Background;

#[derive(Component)]
struct Velocity {
	velocity: Vec2,
}

impl Velocity {
	fn new() -> Self {
		Self { velocity: Vec2::splat(0.) }
	}
}

impl From<Vec2> for Velocity {
	fn from(velocity: Vec2) -> Self {
		Self { velocity }
	}
}

fn main() {
	App::new()
		.insert_resource(WindowDescriptor {
			title: String::from(TITLE),
			width: WIN_W,
			height: WIN_H,
			present_mode: PresentMode::Fifo,
			..default()
		})
		.insert_resource(ClearColor(Color::DARK_GRAY))
		.add_plugins(DefaultPlugins)
		.add_startup_system(setup)
		.add_system(move_player)
		.run();
}

fn setup(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,	
){
	commands.spawn_bundle(Camera2dBundle::default());

	let bg_texture_handle = asset_server.load("bg.png");

	commands
		.spawn_bundle(SpriteBundle {
			texture: bg_texture_handle.clone(),
			transform: Transform::from_translation(Vec3::splat(0.)),
			..default()
		})
		.insert(Background);

	let bird_handle = asset_server.load("birds.png");
	let bird_atlas = TextureAtlas::from_grid(bird_handle, Vec2::splat(TILE_SIZE), 2, 2);
	let bird_atlas_handle = texture_atlases.add(bird_atlas);
	commands
		.spawn_bundle(SpriteSheetBundle {
			texture_atlas: bird_atlas_handle.clone(),
			sprite: TextureAtlasSprite {
				index: PlayerType::UFO as usize,
				..default()
			},
			transform: Transform {
				translation: Vec3::new(0., 0., 900.),
				..default()
			},
			..default()
		})
		.insert(Velocity::new())
		.insert(Player);
}

fn move_player(
	time: Res<Time>,
	input: Res<Input<KeyCode>>,
	mut player: Query<(&mut Transform, &mut Velocity), (With<Player>, Without<Background>)>,
){
	let (mut pt, mut pv) = player.single_mut();

	let mut deltav = Vec2::splat(0.);

	if input.pressed(KeyCode::A) {
		deltav.x -= 1.;
	}

	if input.pressed(KeyCode::D) {
		deltav.x += 1.;
	}

	if input.pressed(KeyCode::W) {
		deltav.y += 1.;
	}

	if input.pressed(KeyCode::S) {
		deltav.y -= 1.;
	}

	let deltat = time.delta_seconds();
	let acc = ACCEL_RATE * deltat;

	pv.velocity = if deltav.length() > 0. {
		(pv.velocity + (deltav.normalize_or_zero() * acc)).clamp_length_max(PLAYER_SPEED)
	}
	else if pv.velocity.length() > acc {
		pv.velocity + (pv.velocity.normalize_or_zero() * -acc)
	}
	else {
		Vec2::splat(0.)
	};
	let change = pv.velocity * deltat;

	let new_pos = pt.translation + Vec3::new(
		change.x,
		0.,
		0.,
	);
	if new_pos.x >= -(LEVEL_W/2.) + TILE_SIZE/2.
		&& new_pos.x <= LEVEL_W/2. - TILE_SIZE/2.
	{
		pt.translation = new_pos;
	}

	let new_pos = pt.translation + Vec3::new(
		0.,
		change.y,
		0.,
	);
	if new_pos.y >= -(LEVEL_H/2.) + TILE_SIZE/2.
		&& new_pos.y <= LEVEL_H/2. - TILE_SIZE/2.
	{
		pt.translation = new_pos;
	}
}

//TODO: Write a system to keep the camera centered on the player (within level bounds)
// Don't forget to add your system to the app in `main()`!

//<Your code here>
