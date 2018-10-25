extern crate bigdecimal;
use bigdecimal::BigDecimal;

use bigdecimal::{Zero, One, Signed, Num};

use std::ops;

mod polynomial;
use polynomial::Polynomial;

mod matrix2x2;
use matrix2x2::Matrix;

pub trait Decimal: Clone + PartialOrd + Num + One + Signed + ops::Neg<Output = Self> + From<i32> + From<f32> {}
impl Decimal for f64 {}
impl Decimal for BigDecimal {}

pub trait Appr: Decimal {
	fn trq(p: usize, q: usize, accu: i64) -> Polynomial<Self> {
		let mut qq = Matrix::<Polynomial<Self>>::one();
		for m in 1..=q {
			let qm = Matrix::new((
				(
					Polynomial::from(vec![-Self::cos_rational_x2((4*m*p-1) as i64, (2*q) as i64, accu), Self::one()]),
					-Polynomial::one()
				),
				(Polynomial::one(), Polynomial::zero())
			));
			qq = qm * qq;
		}
		let p = qq.trace().make_odd_or_even();
		p.into_vec().into_iter().map(|x| { if x.is_one() || x.is_zero() {x} else {x.accu(accu)} }).collect::<Vec<_>>().into()
	}

	fn intervals(p: usize, q: usize, accu: i64) -> Vec<(Self, Self)> {
		if q == 1 {
			return vec!((Self::from(-4), Self::from(4)))
		}
		let pol = Self::trq(p, q, accu);
		let eps = Self::eps(accu);
		let (mut r1, mut r2) = (
			(pol.clone() - Self::from(4)).find_roots(Self::from(-4), Self::from(4), eps.clone()).into_iter().map(|x| {x.accu(accu)}).collect::<Vec<_>>(),
			(pol.clone() + Self::from(4)).find_roots(Self::from(-4), Self::from(4), eps        ).into_iter().map(|x| {x.accu(accu)}).collect::<Vec<_>>()
		);
		r1.append(&mut r2);
		assert!(r1.len() % 2 == 0);
		r1.sort_by(|a, b| a.partial_cmp(b).unwrap());
		let mut r2 = Vec::new();
		let mut ri = r1.into_iter();
		while let Some(x1) = ri.next() {
			let x2 = ri.next().unwrap();
			r2.push((x1, x2));
		}
		r2
	}
	
	fn intervals_auto(p: usize, q: usize, accu: i64) -> (Vec<(Self, Self)>, Option<i64>) {
		let mut int = vec![];
		for accu2 in accu..accu+20 {
			int = Self::intervals(p, q, accu2);
			let vl = int.len();
			if vl == q || vl == q - 1 && q % 2 == 0 {
				return (int, Some(accu2))
			}
		}
		(int, None)
	}

	fn eps(_: i64) -> Self;
	fn accu(self, _: i64) -> Self;
	fn cos_rational_x2(p: i64, q: i64, accu: i64) -> Self;
	fn reduce_args(p: i64, q: i64) -> (u32, u32, i8) {
		assert!(q != 0);
		let mut p: u32 = p.abs() as u32;
		let q: u32 = q.abs() as u32;
		p %= 2 * q;
		let mut sign = if p >= q {
			p -= q;
			-1
		} else {1};
		if 2 * p > q {
			p = q - p;
			sign = -sign;
		}
		(p, q, sign)
	}
}

impl Appr for f64 {
	fn accu(self, _p: i64) -> Self {
		self
	}

	fn eps(_q: i64) -> Self {
		1e-15
	}

	fn cos_rational_x2(p: i64, q: i64, _accu: i64) -> f64 {
		let (p, q, sign) = Self::reduce_args(p, q);
		2.0 * (std::f64::consts::PI * p as f64 / q as f64).cos() * sign as f64
	}
}

fn with_accu(bd: BigDecimal, accu: i64) -> BigDecimal {
	with_accu_ref(&bd, accu)
}

fn with_accu_ref(bd: &BigDecimal, accu: i64) -> BigDecimal {
	let mut prec = accu + bd.digits() as i64 - bd.as_bigint_and_exponent().1;
	if prec < 0 {prec = 0};
	bd.with_prec(prec as u64)
}

mod pi;

impl Appr for BigDecimal {
	fn accu(self, p: i64) -> Self {
		with_accu(self, p)
	}
	
	fn eps(accu: i64) -> Self {
		BigDecimal::new(1.into(), accu)
	}

	fn cos_rational_x2(p: i64, q: i64, accu: i64) -> BigDecimal {
		let (p, q, sign) = Self::reduce_args(p, q);
// Sec. 4.1 of https://www.mpfr.org/algorithms.pdf
// roundings are currently done to the nearest digit, not up ^ or down V as required
// also, k is set to 0 for simplicity
		let waccu = if accu > 0 {accu + 2} else {2};
		let pi = BigDecimal::parse_bytes(&pi::DEC[..(waccu as usize + 1)], 10).unwrap();
		let p = BigDecimal::from(p);
		let q = BigDecimal::from(q);
		let x = pi * p / q;
		let r = with_accu(x.square(), waccu); // ^
		let mut s = BigDecimal::one();
		let mut t = BigDecimal::one();
		for l in 1.. {
			if t.digits() as i64 - t.as_bigint_and_exponent().1 < -waccu {
				break
			}
			t = with_accu(t * &r, waccu); // ^
			t = t / (2*l*(2*l-1)); // ^
			s = if l%2 == 0 { s + &t } else { s - &t }; // V
		}
		s = s.double();
		s = if sign == 1 { s } else { -s };
		with_accu(s + BigDecimal::one(), accu) - BigDecimal::one()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn bigdec() {
		assert_eq!(BigDecimal::parse_bytes(b"1.00009", 10).unwrap().with_prec(4), BigDecimal::one());
		assert_eq!(BigDecimal::cos_rational_x2(0, 1, 10), BigDecimal::from(2));
		assert_eq!(BigDecimal::cos_rational_x2(1, 1, 10), BigDecimal::from(-2));
		assert_eq!(BigDecimal::cos_rational_x2(1, 2, 10), BigDecimal::zero());
		assert_eq!(BigDecimal::cos_rational_x2(1, 3, 10), BigDecimal::one());
	}
	
	#[test]
	fn with_accu_test() {
		let x = BigDecimal::parse_bytes(b"123.44678", 10).unwrap();
		assert_eq!(with_accu_ref(&x, 6), x);
		assert_eq!(with_accu_ref(&x, 5), x);
		assert_eq!(with_accu_ref(&x, 4), BigDecimal::parse_bytes(b"123.4468", 10).unwrap());
		assert_eq!(with_accu_ref(&x, 3), BigDecimal::parse_bytes(b"123.447", 10).unwrap());
		assert_eq!(with_accu_ref(&x, 2), BigDecimal::parse_bytes(b"123.45", 10).unwrap());
		assert_eq!(with_accu_ref(&x, 1), BigDecimal::parse_bytes(b"123.4", 10).unwrap());
		assert_eq!(with_accu_ref(&x, 0), BigDecimal::parse_bytes(b"123", 10).unwrap());
		assert_eq!(with_accu(x, -1), BigDecimal::parse_bytes(b"120", 10).unwrap());
	}
}
