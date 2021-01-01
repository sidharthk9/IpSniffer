use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();

	for phrase in &args {
		println!("{}", phrase);
	}

	println!("{:?}", args);
}
