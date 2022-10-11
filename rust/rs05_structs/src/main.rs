// Tuple struct
struct Point(i32, i32);

// Regular struct
struct Rectangle {
	top_left: Point,
	width: i32,
	height: i32,
	//         ^ last trailing comma optional
}

// Methods
impl Rectangle {
	// Note that there is no special constructor name
	// Do not have use "new"
	fn new(top_left: Point, width: i32, height: i32) -> Rectangle {
		Rectangle {
			top_left,
			width,
			height,
		}
	}

	fn identify(&self) {
		println!("I am a Rectangle");
	}
}

// Trait def
trait Shape {
	fn area(&self) -> i32;
	fn contains(&self, p: Point) -> bool;
}

//TODO: Provide an implementation of the Shape trait for Rectangle

impl Shape for Rectangle {
	fn area(&self) -> i32 {
		self.width * self.height
	}

	fn contains(&self, p: Point) -> bool {
		let lowest_x = self.top_left.0;
		let lowest_y = self.top_left.1;
		let highest_x = lowest_x + self.width;
		let highest_y = lowest_y + self.height;

		(lowest_x < p.0 && p.0 < highest_x) && (lowest_y < p.1 && p.1 < highest_y)
	}
}

fn print_area<T: Shape> (some_shape: T) {
	println!("Shape's area: {}", some_shape.area());
}

fn main() {
	// Using typical computer graphics coordinates, (0, 0) in the top left
	// y increases as it goes down, x increases as it goes right
	let r = Rectangle::new(Point(5, 5), 10, 20);
	r.identify();

	print_area(r);

	let s = Rectangle::new(Point(0, 0), 10, 10);

	println!("s contains (5, 5): {}", s.contains(Point(5, 5)));
	println!("s contains (-1, -1): {}", s.contains(Point(-1, -1)));
	println!("s contains (7, 11): {}", s.contains(Point(7, 11)));
}
