use std::fs::OpenOptions;
use std::io::{Write, BufWriter};
use std::process::Command;
use rand::Rng;
use std::str::FromStr;
use std::fmt::Debug;
use std::time::Instant;

fn get_arg<T: FromStr + Debug>(index: usize, value: T) -> T 
where <T as FromStr>::Err: Debug, {
	// Returns the parsed argument passed to the command line at position `index`	
  //
  // # Arguments
  // * `index` - An integer repreting the position of the argument to get (starting from 0)
  // * `value` - A value to return if the argument is not found
  use std::env::args;
  
	let arg: T = match  args().nth(index + 1) {
		Some(val) => val.parse::<T>().unwrap(),
		None => value
	};
	
	return arg;
}

fn main() {									
	let molecules = get_arg::<u32>(0, 20); // Number of molecules
	let rate: f32 = get_arg(1, 0.1); // Rate of degradation
	let step: f32 = get_arg(2, 0.005); // Time step
	let sims: u32 = get_arg(3, 1); // Number of simulations to run
	let mut count = molecules;
	let mut time: f32 = 0.0;
	let mut rng = rand::thread_rng();
	let mut random: f32;

	let file = OpenOptions::new() // Open and flush if present else create the file
		.write(true)
		.create(true)
		.truncate(true)
		.open("data/data.txt")
		.unwrap();

	let mut writer = BufWriter::new(&file);
	 
	// Simulations
	let start = Instant::now();

	// #[cfg(feature = "naive")] 
	for _ in 0..sims {
		while count > 0 {
			random = rng.gen(); // Generates a float between 0 and 1

			write!(writer, "{}\t{}\n", time, count).unwrap(); // Write on file

			if (count as f32 * rate * step) > random {
				count -= 1;
			}

			time += step;
		} // End of a single simulation

		write!(writer, "{}\t{}\n\n\n", time, count).unwrap();
		count = molecules;
		time = 0.0;
	}
	
	writer.flush().unwrap();
	file.sync_all().unwrap(); // Wait that all the data are written on the file
	
  let duration = start.elapsed();

  println!("\n\tTime: {:?}\n", duration);

	let _output = Command::new("gnuplot")
		.arg("plots/plot.plt")
		.output()
		.expect("Failed to execute gnuplot command");
}
