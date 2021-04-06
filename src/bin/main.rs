use hofstadter_butterfly::Appr;

use bigdecimal::BigDecimal;

const HELP: &'static str = "\
Usage:
<run> <t> trq <p> <q>
<run> <t> trq_upto <qmax>
<run> <t> intervals <p> <q>
<run> <t> check
<run> <t> check_full
<run> d intervals_upto <qmax>
<run> d intervals_farey <number of Farey iterations>

Underlying type <t> is 'f' for f64 or 'd' for BigDecimal.";

fn help() {
	eprintln!("{}", HELP)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum UnderlyingType {
	F64,
	BigDecimal,
}

impl UnderlyingType {
	fn trq(self, p: usize, q: usize) -> String {
		match self {
			UnderlyingType::F64 => f64::trq(p, q, 0).as_ref().iter().map(|x| {format!("{}", x)}).collect::<Vec<_>>().join(", "),
			UnderlyingType::BigDecimal => BigDecimal::trq(p, q, q as i64 * 3 / 4 + 2).as_ref().iter().map(|x| {format!("{}", x)}).collect::<Vec<_>>().join(", "),
		}
	}
	
	fn intervals(self, p: usize, q: usize) -> (usize, String) {
		match self {
			UnderlyingType::F64 => {
				let pol = f64::intervals(p, q, 0);
				(pol.len(), pol.iter().map(|x| {format!("{}..{}", x.0, x.1)}).collect::<Vec<_>>().join(", "))
			},
			UnderlyingType::BigDecimal => {
				let pol = BigDecimal::intervals(p, q, q as i64 * 3 / 4 + 2);
				(pol.len(), pol.iter().map(|x| {format!("{}..{}", x.0, x.1)}).collect::<Vec<_>>().join(", "))
			},
		}
	}
	
	fn intervals_auto(self, p: usize, q: usize, accu: i64) -> (String, i64) {
		match self {
			UnderlyingType::F64 => panic!("intervals_auto make no sense for f64"),
			UnderlyingType::BigDecimal => {
				let (pol, na) = BigDecimal::intervals_auto(p, q, accu);
				(pol.iter().map(|x| {format!("{}..{}", x.0, x.1)}).collect::<Vec<_>>().join(", "), na.expect("Failed to find required accuracy"))
			},
		}
	}
}

fn main() {
	let mut args = std::env::args().skip(1);
	let ut = match args.next().unwrap_or(String::new()).as_ref() {
		"f" => UnderlyingType::F64,
		"d" => UnderlyingType::BigDecimal,
		_ => {
			help();
			return
		}
	};
	
	match args.next().unwrap_or(String::new()).as_ref() {
		"trq" => {
			let p: usize = args.next().expect(HELP).parse().expect(HELP);
			let q: usize = args.next().expect(HELP).parse().expect(HELP);
			println!("{}", ut.trq(p, q));
		},
		"trq_upto" => {
			let qmax: usize = args.next().expect(HELP).parse().expect(HELP);
			for q in 1..=qmax {
				for p in (1..=q/2).filter(|x| coprime(*x, q)) {
					println!("[{}, {}] -> {},", p, q, ut.trq(p, q));
				}
			}
		},
		"intervals" => {
			let p: usize = args.next().expect(HELP).parse().expect(HELP);
			let q: usize = args.next().expect(HELP).parse().expect(HELP);
			let (vl, vs) = ut.intervals(p, q);
			println!("{} [{}]", vl, vs);
		},
		"intervals_upto" => {
			let qmax: usize = args.next().expect(HELP).parse().expect(HELP);
			let mut accu = 4;
			for q in 1..=qmax {
				for p in (0..=q/2).filter(|x| coprime(*x, q)) {
					let (int, naccu) = ut.intervals_auto(p, q, accu);
					accu = naccu;
					println!("{}/{}: {}", p, q, int);
				}
			}
		},
		"intervals_farey" => {
			let n: usize = args.next().expect(HELP).parse().expect(HELP);
			let mut accu = 4;
			for (p, q) in farey(n).into_iter() {
				let (int, naccu) = ut.intervals_auto(p, q, accu);
				accu = naccu;
				println!("{}/{}: {}", p, q, int);
			}
		},
		"check" => {
			for q in 2.. {
				let vl = ut.intervals(1, q).0;
				if vl == q || vl == q - 1 && q % 2 == 0 {
					if q > 100 || q % 10 == 0 {
						println!("{} ok", q);
					}
				} else {
					println!("{} fail", q);
					break
				}
			}
		},
		"check_full" => {
			'qloop: for q in 2.. {
				for p in (2..=q/2).filter(|x| coprime(*x, q)) {
					let vl = ut.intervals(p, q).0;
					if !(vl == q || vl == q - 1 && q % 2 == 0) {
						println!("{}/{} fail", p, q);
						break 'qloop
					}
				}
				if q > 100 || q % 10 == 0 {
					println!("{} ok", q);
				}
			}
		},
		_ => {
			help();
		}
	};
}

pub fn coprime(p: usize, q: usize) -> bool {
	let (mut l, mut m) = (p, q);
	while l > 0 {
		let t = m % l;
		m = l;
		l = t;
	}
	m == 1
}

pub fn farey(n: usize) -> Vec<(usize, usize)> {
	let right = (1, 2);
	let mut f = vec![(0, 1), right];
//	let mut fseq = f.clone();
	for _ in 0..n {
		let mut nf = Vec::new();
		for i in 0..f.len()-1 {
			nf.push(f[i]);
			let ((p1, q1), (p2, q2)) = (f[i], f[i+1]);
			let pq = (p1+p2, q1+q2);
			nf.push(pq);
//			fseq.push(pq);
		}
		nf.push(right);
		f = nf;
	}
	f.sort_by_key(|(_, q)| *q);
	f
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn coprimality() {
		assert!(coprime(2, 3));
		assert!(coprime(1, 4));
		assert!(coprime(3, 2));
		assert!(coprime(4, 1));
		assert!(coprime(0, 1));
		assert!(coprime(1, 0));
		assert!(!coprime(6, 4));
		assert!(!coprime(2, 4));
		assert!(!coprime(0, 3));
		assert!(coprime(61, 1024));
		assert!(!coprime(4, 6));
		assert!(!coprime(4, 2));
		assert!(!coprime(3, 0));
		assert!(coprime(1024, 61));
	}
	
	#[test]
	fn farey_test() {
		assert_eq!(farey(4), vec![(0, 1), (1, 2), (1, 3), (1, 4), (1, 5), (2, 5), (1, 6), (2, 7), (3, 7), (3, 8), (2, 9), (4, 9), (3, 10), (3, 11), (4, 11), (5, 12), (5, 13)]);
	}
}
