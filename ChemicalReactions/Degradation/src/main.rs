use std::fs::OpenOptions;
use std::io::{Write, BufWriter};
use std::ops::{Mul, Div, Sub, AddAssign};
use rand::Rng;
use std::str::FromStr;
use std::fmt::Debug;
use std::time::Instant;
use std::iter::Sum;
use num::{NumCast, Zero};

#[cfg(feature = "plot")]
use std::process::Command;

fn get_arg<T>(index: usize, value: T) -> T 
where <T as FromStr>::Err: Debug, T: FromStr + Debug {
	// Returns the parsed argument passed to the command line at position `index`	
	//
	// # Arguments
	// * `index` - An integer repreting the position of the argument to get (starting from 0)
	// * `value` - A value to return if the argument is not found
	let arg: T = match std::env::args().nth(index + 1) {
		Some(val) => val.parse::<T>().unwrap(),
		None => value
	};
	
	arg
}

// Implementing the mean for the `std::iter::Iterator`, won't do it for the variance tough (or 
// maybe I will)
trait MeanExt: Iterator {
	fn mean<M>(self) -> M
	where
		M: Mean<Self::Item>,
		Self: Sized,
	{
		M::mean(self)
	}
}

impl<I: Iterator> MeanExt for I {}

trait Mean<A = Self> {
	fn mean<I>(iter: I) -> Self
	where
		I: Iterator<Item = A>;
}

impl<T> Mean for T 
where 
		T: AddAssign<T> + Div<T, Output = T> + NumCast + Zero + Copy 
	{
		fn mean<I>(iter: I) -> Self
		where 
			I: Iterator<Item = T>,
		{
			let mut sum = T::zero();
			let mut count: usize = 0;

			for v in iter {
					sum += v;
					count += 1;
			}

			sum / num::cast(count).unwrap()
		}
}

impl<'a, T> Mean<&'a T> for T
where T: AddAssign<T> + Div<T, Output = T> + NumCast + Zero + Copy + 'a {
	fn mean<I>(iter: I) -> Self
	where I: Iterator<Item = &'a T>,
	{
		iter.copied().mean()
	}
}

// Implementing generic functions for the mean and the variance
/* fn mean<'a, T>(data: &'a[T]) -> T
where T: Div<T, Output = T> + Sum<&'a T> + NumCast, {
	// Returns the estimated mean of the values in data	
	//
	// # Arguments
	// * `data` - An array of values
	data.iter().sum::<T>() / num::cast(data.len()).unwrap()
} */

fn variance<'a, T>(data: &'a[T]) -> T
where T: Mul<T, Output = T> + Div<T, Output = T> + Sub<T, Output = T> + AddAssign<T> +
				 Zero + Copy + NumCast + Sum<&'a T> {
	// Returns the estimated variance of the values in data	
	//
	// # Arguments
	// * `data` - An array of values
	let mut variance = T::zero(); 
	let mean: T = data.iter().mean();
	
	for &value in data {
		variance += (mean - value) * (mean - value);
	}
	
	variance / num::cast(data.len()).unwrap()
}

fn main() {									
	let molecules = get_arg::<u32>(0, 20); // Number of molecules
	let rate: f32 = get_arg(1, 0.1); // Rate of degradation
	let step: f32 = get_arg(2, 0.005); // Time step
	let sims: usize = get_arg(3, 1); // Number of simulations to run
	let mut count = molecules;
	let mut time: f32 = 0.0;
	let mut rng = rand::thread_rng();
	let mut random: f32;
	let mut times: Vec<f32> = vec![0.0; sims];

	let file = OpenOptions::new() // Open and flush if present else create the file
		.write(true)
		.create(true)
		.truncate(true)
		.open("data/data.txt")
		.unwrap();

	let mut writer = BufWriter::new(&file);
	 
	// Simulations
	let mut start: Instant;

	// #[cfg(feature = "naive")] 
	for run in 0..sims {
		start = Instant::now();
	
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
		
		times[run] = start.elapsed().as_secs_f32();
	}
	
	writer.flush().unwrap();
	file.sync_all().unwrap(); // Wait that all the data are written on the file
	
	let mu = times.iter().mean::<f32>(); // Compute mean
	let sigma = variance(&times).sqrt(); // Compute standard deviation

  println!("\n\tTime (\u{00B5} \u{00B1} \u{03C3} on {} runs): \
  				 {:?} \u{00B1} {:?} [s]\n", sims, mu, sigma);

	#[cfg(feature = "plot")] 
	let _output = Command::new("gnuplot")
		.arg("plots/plot.plt")
		.output()
		.expect("Failed to execute gnuplot command");
}
