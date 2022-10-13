use bevy::{
	window::PresentMode,
	prelude::*,	
};

const TITLE: &str = "bv03 Basic Motion";
const WIN_W: f32 = 1280.;
const WIN_H: f32 = 720.;
const PLAYER_SZ: f32 = 32.;

const PLAYER_SPEED: f32 = 500.;
const ACCEL_RATE: f32 = 2000.;

// use a velocity component to track the player's velocity
#[derive(Component)]
struct Velocity {
	velocity: Vec2,
}

impl Velocity {
	fn new() -> Self {
		Self { velocity: Vec2::splat(0.) }
	}
}

#[derive(Component)]
struct Player;

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

fn setup(mut commands: Commands) {
	commands.spawn_bundle(Camera2dBundle::default());

	commands
		.spawn_bundle(SpriteBundle {
			sprite: Sprite {
				color: Color::SEA_GREEN,
				custom_size: Some(Vec2::splat(PLAYER_SZ)),
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
	mut player: Query<(&mut Transform, &mut Velocity), With<Player>>,
){
	let (mut player_transform, mut player_velocity) = player.single_mut();

	let mut deltav = Vec2::splat(0.);

	//let mut x_vel = 0.;
	//let mut y_vel = 0.;

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

	// calculating by deltat instead of just relying on frames *should* normalize for different framerates
	let deltat = time.delta_seconds();
	let acc = ACCEL_RATE * deltat;

	// calculate the velocity vector by multiplying it with the acceleration constant
	// normalizing and clamping prevents the diagonal speed boost
	player_velocity.velocity = if deltav.length() > 0. {
		(player_velocity.velocity + (deltav.normalize_or_zero() * acc)).clamp_length_max(PLAYER_SPEED)
	}
	else if player_velocity.velocity.length() > acc {
		player_velocity.velocity + (player_velocity.velocity.normalize_or_zero() * -acc)
	}
	else {
		Vec2::splat(0.)
	};
	let change = player_velocity.velocity * deltat;

	let new_pos = player_transform.translation + Vec3::new(
		change.x,
		0.,
		0.,
	);
	// check for player staying within the window with new x position
	if new_pos.x >= -(WIN_W/2.) + PLAYER_SZ/2. && new_pos.x <= WIN_W/2. - PLAYER_SZ/2. {
		player_transform.translation = new_pos;
	}

	let new_pos = player_transform.translation + Vec3::new(
		0.,
		change.y,
		0.,
	);
	// check for player staying within the window with new y position
	if new_pos.y >= -(WIN_H/2.) + PLAYER_SZ/2. && new_pos.y <= WIN_H/2. - PLAYER_SZ/2. {
		player_transform.translation = new_pos;
	}



	/*if player_transform.translation.x + x_vel < (-WIN_W / 2.) + (PLAYER_SZ / 2.) {
		player_transform.translation.x = (-WIN_W / 2.) + (PLAYER_SZ / 2.);
	}
	else if player_transform.translation.x + x_vel > (WIN_W / 2.) - (PLAYER_SZ / 2.) {
		player_transform.translation.x = (WIN_W / 2.) - (PLAYER_SZ / 2.);
	}
	else {
		player_transform.translation.x += x_vel;
	}

	if player_transform.translation.y + y_vel < (-WIN_H / 2.) + (PLAYER_SZ / 2.) {
		player_transform.translation.y = (-WIN_H / 2.) + (PLAYER_SZ / 2.);
	}
	else if player_transform.translation.y + y_vel > (WIN_H / 2.) - (PLAYER_SZ / 2.) {
		player_transform.translation.y = (WIN_H / 2.) - (PLAYER_SZ / 2.);
	}
	else {
		player_transform.translation.y += y_vel;
	}*/
}

/* TODO: 
 * Can we slowly ramp up to speed limit instead of instantly hitting it? - Use vector math
 * Can we normalize behavior on different refresh rates? - use a timer
 * How do we stay inside the window? - if a transform goes out of bounds, normalize to window bounds
 * How do we avoid breaking the speed limit on the diagonal?
 */
