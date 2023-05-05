use std::env;
use std::fs::OpenOptions;
use std::io::{Write, BufWriter};
use std::process::Command;
use rand::Rng;

fn main() {
let args: Vec<String> = env::args().skip(1).collect(); // Command line arguments

// Reading and parsing from command line
let mut molecules: u32 = match args.get(0) {
	Some(val) => val.parse().unwrap(),
	None => {
		println!("Missing number of molecules");
		return;
	}
};

let rate: f32 = match args.get(1) {
	Some(val) => val.parse().unwrap(),
	None => {
		println!("Missing rate");
		return;
	}
};
 
let step: f32 = match args.get(2) {
	Some(val) => val.parse().unwrap(),
	None => {
		println!("Missing time step");
		return;
	}
};

let mut count: f32 = 0.0;
let mut rng = rand::thread_rng();
let mut random: f32;

let file = OpenOptions::new() // Open and flush if present else create the file
	.write(true)
	.create(true)
	.truncate(true)
	.open("data/data.txt")
	.unwrap();

let mut writer = BufWriter::new(file);
 
// Simulation
while molecules > 0 {
	random = rng.gen(); // Generates a float between 0 and 1

	write!(writer, "{}\t{}\n", molecules, count).unwrap(); // Write on file

	if (molecules as f32 * rate * step) > random {
		molecules -= 1;
	}

	count += step;
}

write!(writer, "{}\t{}\n", molecules, count).unwrap();

let _output = Command::new("gnuplot")
	.arg("plots/plot.plt")
	.output()
	.expect("Failed to execute gnuplot command");
}
