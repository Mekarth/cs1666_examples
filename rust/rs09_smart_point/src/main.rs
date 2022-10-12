//TODO: Use smart pointers to create a generic linked-list

// This is a generic stack implementation using a linked list (thanks https://rust-unofficial.github.io/too-many-lists/
// for helping me understand how this works)
type Link<T> = Option<Box<Node<T>>>;
// generic linked list node that may or may not have a next node
struct Node<T> {
	data: T,
	next: Link<T>,
}

struct LinkedList<T> {
	head: Link<T>,
}

impl<T> LinkedList<T> {
	fn new() -> Self {
		LinkedList { head: None, }
	}

	fn push(&mut self, data: T) {
		let new_node = Box::new(
			Node {
				data: data,
				next: self.head.take(),
			}
		);

		self.head = Some(new_node);
	}

	fn pop(&mut self) -> Option<T> {
		self.head.take().map(|node| {
			self.head = node.next;
			node.data
		})
	}
}

fn main() {
	let s10 = 10;
	let h10 = Box::new(10);

	println!("stack 10: {}", s10);
	println!("heap 10: {}", h10);
	// Will error, need to dereference smart pointer
	//println!("Equal? {}", s10 == h10);
	// OK
	println!("Equal? {}", s10 == *h10);

	println!();

	// Needed to do this with references as well!
	let s5 = 5;
	let r5 = &s5;
	println!("stack 5: {}", s5);
	println!("ref 5: {}", r5);
	// Will error, need to dereference the reference
	//println!("Equal? {}", s5 == r5);
	// OK
	println!("Equal? {}", s5 == *r5);

	// Edit and extend this example to learn more about how Box<T> and refs
	// Do you need to deref an &mut to assign a value to it?
	// Can you mutate a Box<T>?
	
	// test to make sure linked list stack works
	println!();
	println!("Creating linked list...");
	let mut our_list = LinkedList::new();
	println!("Pushing '1'...");
	our_list.push(1);
	println!("Pushing '2'...");
	our_list.push(2);
	println!("Pushing '3'...");
	our_list.push(3);

	// popped value should be 3
	let mut popped_node = our_list.pop();
	match popped_node {
		Some(d) => println!("Popped value: {}", d),
		None => println!("No popped value :("),
	}
	// popped value should be 2
	popped_node = our_list.pop();
	match popped_node {
		Some(d) => println!("Popped value: {}", d),
		None => println!("No popped value :("),
	}
	// popped value should be 1
	popped_node = our_list.pop();
	match popped_node {
		Some(d) => println!("Popped value: {}", d),
		None => println!("No popped value :("),
	}
	// should have returned None -> no value
	popped_node = our_list.pop();
	match popped_node {
		Some(d) => println!("Popped value: {}", d),
		None => println!("No popped value :("),
	}
}