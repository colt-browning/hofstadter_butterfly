extern crate bigdecimal;
use bigdecimal::{BigDecimal, ToPrimitive};

extern crate line_drawing;
use line_drawing::Supercover as Line;

extern crate repng;

use std::{
	io::{stdin, prelude::*},
	str::FromStr,
};

enum Format {
	Png,
	Svg,
}

use Format::*;

fn main() -> Result<(), Box<std::error::Error>> {
	let mut args = std::env::args().skip(1);
	let format = match args.next().unwrap_or(String::new()).as_ref() {
		"png" => Png,
		"svg" => Svg,
		_ => {
			println!("Usage:
cat out.txt | cargo run --release --bin txt2img [png|svg]");
			return Ok(())
		}
	};
	
	let s = stdin();
	let mut v = Vec::new();
	for line in s.lock().lines() {
		let line = line?;
		let t = line.split(": ").collect::<Vec<&str>>();
		assert!(t.len() == 2);
		let (frac, intervals) = (t[0], t[1]);
		let t = frac.split("/").collect::<Vec<&str>>();
		assert!(t.len() == 2);
		let (num, denom) = (t[0].parse::<u32>()?, t[1].parse::<u32>()?);
		let intervals = intervals.split(", ").map(|int| {
			let t = int.split("..").collect::<Vec<&str>>();
			assert!(t.len() == 2);
			let (l, r) = (BigDecimal::from_str(t[0]).unwrap(), BigDecimal::from_str(t[1]).unwrap());
			(l, r)
		}).collect::<Vec<_>>();
		v.push(((num, denom), intervals.clone()));
		if denom != num * 2 {
			v.push(((denom - num, denom), intervals));
		}
	}
	match format {
		Png => {
			let n: u32 = 4096;
			let color = [0, 0, 0, 255];
			let mut pixels = vec![255; (n * n * 4) as usize];
			for ((num, denom), intervals) in v {
				let x = ((num * n) as f64 / denom as f64) as i32;
				for (y1, y2) in intervals {
					let y1: BigDecimal = (y1 + BigDecimal::from(4)) / 8 * BigDecimal::from(n);
					let y1 = y1.to_i32().unwrap();
					let y2: BigDecimal = (y2 + BigDecimal::from(4)) / 8 * BigDecimal::from(n);
					let y2 = y2.to_i32().unwrap();
					let line = Line::new((x, y1), (x, y2));
					for (x, y) in line {
						let x = if x >= n as i32 { x-1 } else {x};
						let x = if x < 0 as i32 { 0 } else {x};
						let y = if y >= n as i32 { y-1 } else {y};
						let y = if y < 0 as i32 { 0 } else {y};
						let p = (n as usize * x as usize + y as usize) * 4;
						pixels[p..p+4].copy_from_slice(&color);
					}
				}
			}
			repng::encode(std::fs::File::create("out.png")?, n, n, &pixels)?;
		},
		Svg => {
			let (width, height) = (600, 450);
			let mut f = std::fs::File::create(&std::path::Path::new("out.svg"))?;
			writeln!(f, r#"<?xml version="1.0" encoding="UTF-8"?>"#);
			writeln!(f, r#"<svg version="1.1" xmlns="http://www.w3.org/2000/svg" width="{}" height="{}">"#, width, height);
			writeln!(f, r#"<g fill="transparent" stroke="black" stroke-width="0.5" stroke-linecap="square">"#);
			for ((num, denom), intervals) in v {
				write!(f, r#"<path id="{}_{}" d=""#, num, denom);
				let x = (num * height) as f32 / denom as f32;
				let x = if x == 0.0 {1.0} else {x};
				for (y1, y2) in intervals {
					let w: BigDecimal = (y2 - &y1) / 8 * BigDecimal::from(width);
					let w = w.with_prec(5);
					let y1: BigDecimal = (y1 + BigDecimal::from(4)) / 8 * BigDecimal::from(width);
					let prec = 38;
					let y1 = if y1.digits() > prec {y1.with_prec(prec)} else {y1};
					write!(f, "M {} {} h {} ", y1, x, w);
				}
				writeln!(f, "\"/>");
			}
			writeln!(f, "</g>");
			writeln!(f, "</svg>");
		},
	}
	Ok(())
}
