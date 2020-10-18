extern crate minifb;

use minifb::{Key, Window, WindowOptions};
use std::thread::sleep;
use std::time::Duration;
use std::time::Instant;

const WIDTH: usize = 600;
const HEIGHT: usize = 600;

const MAX_ITERS: usize = 256;

const COLORS: [u32; 8] = [
	0x800000, 0x008000, 0x808000, 0x000080, 0x800080, 0x008080, 0xc0c0c0, 0x808080,
];

// @TODO IMPLEMENT
// fn map<T>(x: T, old_lo: T, old_hi: T, new_lo: T, new_hi: T) -> T
// 	where
// {

// }

fn map(x: usize, old_lo: usize, old_hi: usize, new_lo: u32, new_hi: u32) -> u32 {
	return new_lo
		+ (((x - old_lo) as f64 / (old_hi - old_lo) as f64) * (new_hi - new_lo) as f64) as u32;
}

// @TODO FIX this is super janky
fn map_fl(x: usize, old_lo: usize, old_hi: usize, new_lo: f64, new_hi: f64) -> f64 {
	return new_lo
		+ (((x - old_lo) as f64 / (old_hi - old_lo) as f64) * (new_hi - new_lo) as f64) as f64;
}

fn bship(c_re: f64, c_im: f64) -> (bool, usize) {
	let mut a = 0.;
	let mut b = 0.;

	for n in 0..MAX_ITERS {
		// z -> z^2 + c
		// z = a+bi
		// a -> a^2 - b^2 + c_re
		// b -> 2ab + c_im
		let a_new = (a * a) - (b * b) + c_re;
		let b_new = 2. * a.abs() * b.abs() + c_im;

		a = a_new;
		b = b_new;

		if (a * a) + (b * b) > 4. {
			return (false, n);
		}
	}

	(true, MAX_ITERS)
}

fn mandelbrot(c_re: f64, c_im: f64) -> (bool, usize) {
	let mut a = 0.;
	let mut b = 0.;

	for n in 0..MAX_ITERS {
		// z -> z^2 + c
		// z = a+bi
		// a -> a^2 - b^2 + c_re
		// b -> 2ab + c_im

		let a_new = (a * a) - (b * b) + c_re;
		let b_new = 2. * a * b + c_im;

		a = a_new;
		b = b_new;

		if (a * a) + (b * b) > 4. {
			return (false, n);
		}
	}

	(true, MAX_ITERS)
}

fn mzoom(nc_re: f64, nc_im: f64, n: f64) -> (bool, usize) {
	let mut a = 0.;
	let mut b = 0.;

	for n_iters in 0..MAX_ITERS {
		// z -> z^2 + c
		// z = a+bi
		// a -> a^2 - b^2 + c_re
		// b -> 2ab + c_im
		let a_new = ((a * a) - (b * b)) / n + nc_re;
		let b_new = 2. * a * b / n + nc_im;

		a = a_new;
		b = b_new;

		if (a * a) + (b * b) > 4. / (n * n) {
			return (false, n_iters);
		}
	}

	(true, MAX_ITERS)
}

fn main() {
	let mut window = Window::new("Mandelbrot", WIDTH, HEIGHT, WindowOptions::default())
		.expect("Unable to open window");

	let mut buffer = vec![0u32; WIDTH * HEIGHT];

	// Order: ARGB
	for pixel in buffer.iter_mut() {
		*pixel = 0x00000000;
	}

	// Limit to max ~60 fps update rate
	window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

	// window.set_background_color(0, 0, 20);

	let mut range = 2.;

	let mut base_a = 0.;
	let mut base_b = 0.;

	// let mut n = 1.;

	while window.is_open() && !window.is_key_down(Key::Q) {
		// @TODO move?
		window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();

		// @TODO how can we use iter to remove bounds checks?
		if window.is_key_down(Key::Equal) {
			range /= 1.25;
		} else if window.is_key_down(Key::Minus) {
			range *= 1.25;
		} else if window.is_key_down(Key::Left) {
			// base_a -= /* range / 10. */ 0.1;
			base_a -= range / 10.;
		} else if window.is_key_down(Key::Right) {
			// base_a += /* range / 10. */ 0.1;
			base_a += range / 10.;
		} else if window.is_key_down(Key::Up) {
			// base_b -= /* range / 10. */ 0.1;
			base_b -= range / 10.;
		} else if window.is_key_down(Key::Down) {
			// base_b += /* range / 10. */ 0.1;
			base_b += range / 10.;
		} else {
			sleep(Duration::from_micros(16600));
			continue;
		}

		let a_min = base_a - range;
		let a_max = base_a + range;

		let b_min = base_b - range;
		let b_max = base_b + range;

		let mut colors_index = 0usize;

		for x in 0..WIDTH {
			for y in 0..HEIGHT {
				// let red = map(x + y, 0, WIDTH + HEIGHT, 0, 256);
				// let green = red;
				// let blue = red;

				#[cfg(debug)]
				let now = Instant::now();
				let c_re = map_fl(x, 0, WIDTH, a_min, a_max);
				let c_im = map_fl(y, 0, HEIGHT, b_min, b_max);
				#[cfg(debug)]
				// let t_map = now.elapsed();
				#[cfg(debug)]
				let now = Instant::now();

				let (in_mandelbrot, n_iters) = mandelbrot(c_re, c_im);
				#[cfg(debug)]
				// let t_mand = now.elapsed();
				#[cfg(debug)]
				let now = Instant::now();
				let (_, _) = mandelbrot(c_re, c_im);
				#[cfg(debug)]
				// let t_mand_op = now.elapsed();
				#[cfg(debug)]
				let now = Instant::now();

				let brightness = map(n_iters, 0, MAX_ITERS, 0, 0x00FFFFFF);
				// let brightness = 255 - brightness;
				#[cfg(debug)]
				// let t_map2 = now.elapsed();
				#[cfg(debug)]
				let now = Instant::now();

				buffer[x + y * WIDTH] = if in_mandelbrot {
					0u32
				} else {
					COLORS[n_iters % COLORS.len()]
				};
				// let t_rest = now.elapsed();
				// println!(
				// 	"map: {:?}, mand: {:?}, op: {:?}, diff: {:?}, map2: {:?}, rest: {:?}",
				// 	t_map,
				// 	t_mand,
				// 	t_mand_op,
				// 	t_mand.as_nanos() as i32 - t_mand_op.as_nanos() as i32,
				// 	t_map2,
				// 	t_rest
				// );
			}
		}
	}
}
