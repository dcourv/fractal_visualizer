extern crate minifb;

use minifb::{Key, Window, WindowOptions};
// use rayon::prelude::*;
use std::thread::sleep;
use std::time::Duration;
use std::time::Instant;

const WIDTH: usize = 600;
const HEIGHT: usize = 600;

const MAX_ITERS: usize = 256;

const COLORS: [u32; 8] = [
	0x800000, 0x008000, 0x808000, 0x000080, 0x800080, 0x008080, 0xc0c0c0, 0x808080,
];

const DEFAULT_FRACT_FN: fn(f64, f64) -> (bool, usize) = mandelbrot;

// @TODO Make sure #[cfg(Debug)] is doing what you think it is

// @TODO IMPLEMENT
// fn map<T>(x: T, old_lo: T, old_hi: T, new_lo: T, new_hi: T) -> T
// 	where
// {

// }

fn _map(x: usize, old_lo: usize, old_hi: usize, new_lo: u32, new_hi: u32) -> u32 {
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

		// |z| = 2 is a definite escape threshold, i.e. if |z|^2 >= 4, z will eventually escape
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

		// |z| = 2 is an escape value
		if (a * a) + (b * b) > 4. {
			return (false, n);
		}
	}

	(true, MAX_ITERS)
}

fn _test1(c_re: f64, c_im: f64) -> (bool, usize) {
	let mut a = 0f64;
	let mut b = 0f64;

	for n in 0..MAX_ITERS {
		// z -> z^2 + c
		// z = a+bi
		// a -> a^2 - b^2 + c_re
		// b -> 2ab + c_im

		// very strange behavior--just add one more zero and it looks just like mandel!
		let a_new = a.powf(2.000000000000001) - b.powf(2.000000000000001) + c_re;
		let b_new = 2. * a * b + c_im;

		a = a_new;
		b = b_new;

		if (a * a) + (b * b) > 4. {
			return (false, n);
		}
	}

	(true, MAX_ITERS)
}

fn test(c_re: f64, c_im: f64) -> (bool, usize) {
	let mut a = 0f64;
	let mut b = 0f64;

	for n in 0..MAX_ITERS {
		// z -> z^1.5 + c

		let pow = 3.;

		let r_new = (a * a + b * b).powf(pow / 2.);
		let theta_new = pow * f64::atan2(b, a);

		let a_new = r_new * theta_new.cos() + c_re;
		let b_new = r_new * theta_new.sin() + c_im;

		a = a_new;
		b = b_new;

		if (a * a) + (b * b) > 100. {
			return (false, n);
		}
	}

	(true, MAX_ITERS)
}

fn _mzoom(nc_re: f64, nc_im: f64, n: f64) -> (bool, usize) {
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

fn update_ab_minmax(
	a_min: &mut f64,
	a_max: &mut f64,
	b_min: &mut f64,
	b_max: &mut f64,
	a_mid: f64,
	b_mid: f64,
	range: f64,
) {
	*a_min = a_mid - range;
	*a_max = a_mid + range;

	*b_min = b_mid - range;
	*b_max = b_mid + range;
}

fn main() {
	let mut window = Window::new("Mandelbrot", WIDTH, HEIGHT, WindowOptions::default())
		.expect("Unable to open window");

	let mut buffer = vec![0u32; WIDTH * HEIGHT];

	// @TODO read args and set bship alternately
	let fractal_fn = match std::env::args().nth(1) {
		Some(arg) => match arg.as_str() {
			"mandelbrot" | "mand" | "m" => mandelbrot,
			"burning_ship" | "bship" | "b" => bship,
			// @TODO dejankify
			"test" => test,
			_ => {
				eprintln!("Unknown fractal function");
				return;
			}
		},
		None => DEFAULT_FRACT_FN,
	};

	// Initialize
	// @TODO fix bug with a/b min range/zoom location
	// NB: zooms to bottom left corner
	let mut range = 3.0;
	let mut a_mid = 0.0;
	let mut b_mid = 0.0;
	// let mut b_mid = range / 2.;

	// This is unnecessary, but rust compiler is stupid and doesn't know that update_ab_minmax initializes them
	let mut a_min = 0.;
	let mut a_max = 0.;
	let mut b_min = 0.;
	let mut b_max = 0.;

	update_ab_minmax(
		&mut a_min, &mut a_max, &mut b_min, &mut b_max, a_mid, b_mid, range,
	);

	while window.is_open() && !window.is_key_down(Key::Q) {
		// DEBUG
		println!(
			"a min, a max, b min, b max, a mid, b mid, range: {:?}",
			(&mut a_min, &mut a_max, &mut b_min, &mut b_max, a_mid, b_mid, range)
		);

		let frame_start = Instant::now();

		sleep(Duration::from_micros(16600));

		/* Keyboard shortcuts */
		{
			// @TODO move?
			window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
			if window.is_key_down(Key::Equal) {
				range /= 1.25;
				update_ab_minmax(
					&mut a_min, &mut a_max, &mut b_min, &mut b_max, a_mid, b_mid, range,
				);
				// @DEBUG
				println!("a min, max, mid: {} {} {}", a_min, a_max, a_mid)
			} else if window.is_key_down(Key::Minus) {
				range *= 1.25;
				update_ab_minmax(
					&mut a_min, &mut a_max, &mut b_min, &mut b_max, a_mid, b_mid, range,
				);
			} else if window.is_key_down(Key::Left) {
				a_mid -= range / 10.;
				update_ab_minmax(
					&mut a_min, &mut a_max, &mut b_min, &mut b_max, a_mid, b_mid, range,
				);
			} else if window.is_key_down(Key::Right) {
				a_mid += range / 10.;
				update_ab_minmax(
					&mut a_min, &mut a_max, &mut b_min, &mut b_max, a_mid, b_mid, range,
				);
			} else if window.is_key_down(Key::Up) {
				b_mid -= range / 10.;
				update_ab_minmax(
					&mut a_min, &mut a_max, &mut b_min, &mut b_max, a_mid, b_mid, range,
				);
			} else if window.is_key_down(Key::Down) {
				b_mid += range / 10.;
				update_ab_minmax(
					&mut a_min, &mut a_max, &mut b_min, &mut b_max, a_mid, b_mid, range,
				);
				// } else {
				// 	sleep(Duration::from_micros(16600));
				// 	continue;
			}
		}

		/* Update buffer with colorized fractal result */
		{
			// @TODO can do this outside loop, update on keypress
			// let mut c_re = map_fl(0, 0, WIDTH, a_min, a_max);
			// let c_im = map_fl(0, 0, HEIGHT, b_min, b_max);

			let c_re_inc = map_fl(1, 0, WIDTH, 0.0, 2. * range);
			let c_im_inc = map_fl(1, 0, HEIGHT, 0.0, 2. * range);

			let now = Instant::now();

			let hot_colors = COLORS.clone();
			let colors_len = COLORS.len();

			let mut c_im = b_min;
			let mut idx = 0;
			for y in 0..HEIGHT {
				let mut c_re = a_min;
				for x in 0..WIDTH {
					// @DEBUG
					// let c_re_alt = map_fl(x, 0, WIDTH, a_min, a_max);
					// let c_im_alt = map_fl(y, 0, HEIGHT, b_min, b_max);

					// println!("{:?}", (x, y));
					// println!("{:?}", (c_re, c_im));
					// println!("{:?}", (c_re_alt, c_im_alt));
					// println!("{:?}", (c_re_inc, c_im_inc));

					// assert_eq!(c_re, c_re_alt);
					// assert_eq!(c_im, c_im_alt);

					let (fract_res, n_iters) = fractal_fn(c_re, c_im);

					// @TODO change to idx
					buffer[idx] = if fract_res {
						0u32
					} else {
						// @TODO change to enum
						hot_colors[n_iters % colors_len]
					};

					c_re += c_re_inc;
					idx += 1;
				}
				c_im += c_im_inc;
				// idx += 1;
			}

			let update_buf_time = Instant::now() - now;
			println!("{:?}", update_buf_time);
		}

		let frame_dur = Instant::now() - frame_start;
		let target_frame_dur = Duration::from_micros(33333);
		// @TODO use better api Duration::zero() when stable
		if frame_dur < target_frame_dur {
			sleep(target_frame_dur - frame_dur);
		}

		// Was trying to parallelize with rayon, but this is slow as shit!
		// for (buf_idx, pix) in buffer.par_iter_mut().enumerate() {
		// 	// buf_idx = x + (y*WIDTH)
		// 	let x = buf_idx % WIDTH;
		// 	let y = (buf_idx - x) / HEIGHT;

		// 	let c_re = map_fl(x, 0, WIDTH, a_min, a_max);
		// 	let c_im = map_fl(y, 0, HEIGHT, b_min, b_max);

		// 	let (fract_res, n_iters) = fractal_fn(c_re, c_im);

		// 	println!("{:?}", (a_min, a_max, b_min, b_max));

		// 	*pix = if fract_res {
		// 		0u32
		// 	} else {
		// 		// @TODO change to enum
		// 		COLORS[n_iters % COLORS.len()]
		// 	};
		// }

		// Figure out if we're ahead of our frame time, if so: sleep rest of frame time
		// sleep(Duration::from_micros(16600));
	}
}
