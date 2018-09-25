use ::{Zero, One, Signed};
use std::{ops, fmt};

#[derive(Debug, Clone, PartialEq)]
pub struct Polynomial<T> {
	factors: Vec<T>,
}

impl<T> Polynomial<T> {
	pub fn into_vec(self) -> Vec<T> {
		self.factors
	}
	
	pub fn as_ref(&self) -> &Vec<T> {
		&self.factors
	}
	
	pub fn degree(&self) -> usize {
		self.factors.len() - 1
	}
}

impl<T> Zero for Polynomial<T> where T: Zero + Clone {
	fn zero() -> Self {
		Polynomial {
			factors: vec!(T::zero()),
		}
	}
	
	fn is_zero(&self) -> bool {
		self.degree() == 0 && self.factors[0].is_zero()
	}
}

impl<T> One for Polynomial<T> where T: Zero + One + Clone + PartialEq {
	fn one() -> Self {
		Polynomial::monomial(0, T::one())
	}
	
	fn is_one(&self) -> bool {
		self.degree() == 0 && self.factors[0].is_one()
	}
}

impl<T> fmt::Display for Polynomial<T> where T: fmt::Display + Zero + One + Signed {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if self.degree() == 0 && self.factors[0].is_zero() {
			return write!(f, "{}", self.factors[0])
		}
		let mut empty = true;
		for (n, a) in self.factors.iter().enumerate().filter(|(_, a)| {
			!a.is_zero()
		}) {
			if !empty && a.is_positive() {
				write!(f, "+")?;
			}
			empty = false;
			if n == 0 || !a.is_one() {
				write!(f, "{}", a)?;
			}
			match n {
				0 => (),
				1 => { write!(f, "x")?; }
				_ => { write!(f, "x{}", n)?; }
			}
		}
		Ok(())
	}
}

impl<T> Polynomial<T> where T: Zero + Clone {
	fn cleanup(mut self) -> Self {
		while self.factors.len() > 1 && self.factors.last().unwrap().is_zero() {
			self.factors.pop();
		}
		self
	}

	pub fn monomial(n: usize, factor: T) -> Self {
		if factor.is_zero() {
			return Self::zero()
		}
		let mut factors = vec![T::zero(); n+1];
		factors[n] = factor;
		Polynomial {
			factors,
		}
	}
		
	fn extend(&mut self, l: usize) {
		let t = self.factors.len();
		for _ in t..l {
			self.factors.push(Zero::zero());
		}
	}
	
	pub fn make_odd_or_even(mut self) -> Self {
		let t = self.degree() % 2;
		for (n, mut a) in self.factors.iter_mut().enumerate() {
			if n % 2 != t {
				*a = T::zero();
			}
		}
		self
	}
}
	
impl<T> Polynomial<T> where T: Zero + Clone {
	pub fn eval_mul<'a, X>(&'a self, x: &X) -> X
// currently used in tests only
		where X:
			Clone +
			ops::Add +
			ops::Mul +
			Zero +
			One +
			ops::Mul<&'a T, Output = X>
	{
		let mut sum = X::zero();
		let mut xp = X::one();
		for factor in &self.factors {
			let m = xp.clone() * factor;
			sum = sum + m;
			xp = xp * x.clone();
		}
		sum
	}
	
	pub fn eval_ref<X>(&self, x: &X) -> X
		where X:
			Clone +
			ops::Add +
			ops::Mul +
			Zero +
			One +
			From<T>
	{
		let mut sum = X::zero();
		let mut xp = X::one();
		for factor in &self.factors {
			let m = xp.clone() * X::from(factor.clone());
			sum = sum + m;
			xp = xp * x.clone();
		}
		sum
	}

	pub fn eval<X>(&self, x: X) -> X
		where X:
			Copy +
			ops::Add +
			ops::Mul +
			Zero +
			One +
			From<T>
	{
		self.eval_ref(&x)
	}
}

impl<T> From<Vec<T>> for Polynomial<T> where T: Zero + Clone {
	fn from(factors: Vec<T>) -> Self {
		Polynomial {
			factors,
		}.cleanup()
	}
}

impl<T> ops::Add for Polynomial<T> where T: Zero + Clone {
	type Output = Polynomial<T>;
	fn add(mut self, mut rhs: Self) -> Self {
		self.extend(rhs.factors.len());
		rhs.extend(self.factors.len());
		let v: Vec<T> = self.factors.into_iter().zip(rhs.factors.into_iter()).map(|(l, r)| { l + r }).collect();
		Self::from(v).cleanup()
	}
}

impl<T> ops::Sub for Polynomial<T> where T: Zero + ops::Sub<Output=T> + Clone {
	type Output = Polynomial<T>;
	fn sub(mut self, mut rhs: Self) -> Self {
		self.extend(rhs.factors.len());
		rhs.extend(self.factors.len());
		let v: Vec<T> = self.factors.into_iter().zip(rhs.factors.into_iter()).map(|(l, r)| { l - r }).collect();
		Self::from(v).cleanup()
	}
}

impl<T> ops::Neg for Polynomial<T> where T: ops::Neg<Output=T> {
	type Output = Self;
	fn neg(self) -> Self {
		Polynomial {
			factors: self.factors.into_iter().map(ops::Neg::neg).collect()
		}
	}
}

impl<T> ops::Add<T> for Polynomial<T> where T: Zero + Clone {
	type Output = Polynomial<T>;
	fn add(self, rhs: T) -> Self {
		self + Self::monomial(0, rhs)
	}
}

impl<T> ops::Sub<T> for Polynomial<T> where T: Zero + ops::Sub<Output=T> + Clone {
	type Output = Polynomial<T>;
	fn sub(self, rhs: T) -> Self {
		self + Self::monomial(0, T::zero()-rhs)
	}
}

impl<T> ops::Mul<T> for Polynomial<T> where T: Zero + ops::Mul<Output=T> + Clone {
	type Output = Polynomial<T>;
	fn mul(self, rhs: T) -> Self {
		Self::from(self.factors.into_iter().map(|x| x*rhs.clone()).collect::<Vec<_>>()).cleanup()
	}
}

impl<T> ops::Mul for Polynomial<T> where T: Zero + ops::Mul<Output=T> + Clone {
	type Output = Polynomial<T>;
	fn mul(self, rhs: Self) -> Self {
		let v: Vec<T> = (0..=(self.degree() + rhs.degree())).map(|n| {
			let b = if n > rhs.degree() { n - rhs.degree() } else {0};
			let e = if n > self.degree() { self.degree() } else {n};
			(b..=e).map(|i| {
				self.factors[i].clone() * rhs.factors[n-i].clone()
			}).fold(T::zero(), |a, b| a + b)
		}).collect();
		Self::from(v).cleanup()
	}
}

impl<T> ops::Rem for Polynomial<T> where T: Zero + ops::Sub<Output=T> + ops::Mul<Output=T> + ops::Div<Output=T> + Clone {
	type Output = Polynomial<T>;
	fn rem(mut self, rhs: Self) -> Self {
		while self.degree() >= rhs.degree() {
			let mut rhs2 = rhs.clone();
			while self.degree() > rhs2.degree() {
				rhs2.factors.insert(0, T::zero());
			}
			let f = self.factors.last().unwrap().clone() / rhs.factors.last().unwrap().clone();
			let rhs2d = rhs2.degree();
			self = self - rhs2 * f;
			if self.degree() == rhs2d { // in case the leading factors do not cancel away exactly
				self.factors.pop();
				self = self.cleanup();
			}
		}
		self
	}
}

impl<T> Polynomial<T> where T: Zero + One + ops::Sub<Output=T> + ops::Mul<Output=T> + ops::Div<Output=T> + Clone {
	fn _gcd(&self, rhs: &Self) -> Self {
		let mut a = self.clone();
		let mut b = rhs.clone();
		while !b.is_zero() {
			let t = a % b.clone();
			a = b;
			b = t;
		}
		let f = T::one() / a.factors.last().unwrap().clone();
		a * f
	}
}

impl<T> Polynomial<T> where T: Zero + Clone + ops::Mul<Output=T> + From<i32> {
	pub fn derivative(mut self) -> Self {
		if self.degree() == 0 {
			Polynomial::zero()
		} else {
			Polynomial {
				factors: self.factors.drain(1..).enumerate().map(|(n, a)| { T::from((n+1) as i32) * a }).collect(),
			}
		}
	}
}

impl<T> Polynomial<T> where T:
	Zero
	+ One
	+ ops::Sub<Output=T>
	+ ops::Mul<Output=T>
	+ ops::Div<Output=T>
	+ ops::Neg<Output=T>
	+ From<i32>
	+ Clone
{
	pub fn sturm_sequence(&self) -> Vec<Self> {
		if self.degree() == 0 {
			return vec!(self.clone())
		}
		let mut seq = vec![self.clone(), self.clone().derivative()]; // assuming square-free case
		while seq.last().unwrap().degree() > 0 {
			let p = seq[seq.len()-2].clone() % seq.last().unwrap().clone();
			seq.push(p * (-T::one()));
		}
		seq
	}
}

impl<T> Polynomial<T> where T:
	Zero
	+ One
	+ ops::Sub<Output=T>
	+ ops::Mul<Output=T>
	+ ops::Div<Output=T>
	+ ops::Neg<Output=T>
	+ From<i32>
	+ From<f32>
	+ Signed
	+ PartialOrd
	+ Clone
{
	pub fn localize_roots(&self, left: T, right: T) -> Vec<(T, T)> {
		if right <= left {
			return vec![]
		}
		self.try_localize_roots_internal(left, right, None).unwrap()
	}
	
	pub fn try_localize_roots(&self, left: T, right: T, expected_roots: usize) -> Result<Vec<(T, T)>, usize> {
		self.try_localize_roots_internal(left, right, Some(expected_roots))
	}
	
	fn try_localize_roots_internal(&self, left: T, right: T, expected_roots: Option<usize>) -> Result<Vec<(T, T)>, usize> {
		let ss = self.sturm_sequence();
		let (ssl, ssr) = (
			ss.iter().map(|p| p.eval_ref(&left).is_positive()).collect::<Vec<_>>(),
			ss.iter().map(|p| p.eval_ref(&right).is_positive()).collect::<Vec<_>>(),
		);
		let (csl, csr) = (
			ssl.iter().zip(&ssl[1..]).filter(|(x, y)| { **x^*y }).count(),
			ssr.iter().zip(&ssr[1..]).filter(|(x, y)| { **x^*y }).count(),
		);
//		assert!(csr <= csl);
		if let Some(n) = expected_roots {
			let r = csl - csr;
			if r != n {
				return Err(r)
			}
		}
		Ok(self.localize_roots_internal(left, right, csl, csr, &ss))
	}

	fn localize_roots_internal(&self, left: T, right: T, csl: usize, csr: usize, ss: &Vec<Self>) -> Vec<(T, T)> {
		if csr == csl {
			return vec![]
		} else if csl - csr == 1 {
			return vec!((left, right))
		}
		let middle = (right.clone() + left.clone()) / (2.0f32).into();
		let ssm = ss.iter().map(|p| p.eval_ref(&middle).is_positive()).collect::<Vec<_>>();
		let csm = ssm.iter().zip(&ssm[1..]).filter(|(x, y)| { **x^*y }).count();
//		assert!(csm <= csl && csm >= csr);
		let (mut lrl, mut lrr) = (
			self.localize_roots_internal(left, middle.clone(), csl, csm, ss),
			self.localize_roots_internal(middle, right, csm, csr, ss)
		);
		lrl.append(&mut lrr);
		lrl
	}
	
	pub fn find_roots(&mut self, left: T, right: T, eps: T) -> Vec<T> {
		let mut v = Vec::new();
		while !self.is_zero() && self.factors[0].is_zero() {
			v.push(T::zero());
			self.factors.remove(0);
		}
		v.append(&mut self.localize_roots(left, right).into_iter().map(|(mut l, mut r)| {
			let vrp = self.eval_ref(&r).is_positive();
			while r.clone() - l.clone() > eps {
				let m = (r.clone() + l.clone()) / (2.0f32).into();
				let vmp = self.eval_ref(&m).is_positive();
				if vmp == vrp {
					r = m;
				} else {
					l = m;
				}
			}
			r
		}).collect());
		v
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[derive(Debug, Clone, PartialEq)]
	struct Matrix {
		m: Vec<Vec<f32>>,
	}
	
	impl<'a> ops::Mul<&'a f32> for Matrix {
		type Output = Matrix;
		fn mul(mut self, rhs: &f32) -> Matrix {
			for mut row in &mut self.m {
				for mut v in row {
					*v *= rhs;
				}
			}
			self
		}
	}
	
	impl ops::Add for Matrix {
		type Output = Matrix;
		fn add(mut self, rhs: Self) -> Matrix {
			for i in 0..3 {
				for j in 0..3 {
					self.m[i][j] += rhs.m[i][j];
				}
			}
			self
		}
	}

	impl ops::Mul for Matrix {
		type Output = Matrix;
		fn mul(self, rhs: Self) -> Matrix {
			let mut m: Self = Zero::zero();
			for i in 0..3 {
				for j in 0..3 {
					for k in 0..3 {
						m.m[i][j] += self.m[i][k] * rhs.m[k][j];
					}
				}
			}
			m
		}
	}

	impl Zero for Matrix {
		fn zero() -> Matrix {
			Matrix {
				m: vec![
					vec![0.0, 0.0, 0.0],
					vec![0.0, 0.0, 0.0],
					vec![0.0, 0.0, 0.0],
				],
			}
		}
		
		fn is_zero(&self) -> bool {
			for row in &self.m {
				for v in row {
					if *v != 0.0 {
						return false;
					}
				}
			}
			true
		}
	}

	impl One for Matrix {
		fn one() -> Matrix {
			Matrix {
				m: vec![
					vec![1.0, 0.0, 0.0],
					vec![0.0, 1.0, 0.0],
					vec![0.0, 0.0, 1.0],
				],
			}
		}
		
		fn is_one(&self) -> bool {
			*self == Self::one()
		}
	}
	
    #[test]
    fn eval() {
		let p = Polynomial::from(vec![4, 5, 6]);
		let x = 2;
        assert_eq!(p.eval(x), 38);
		let x = 2.0;
        assert_eq!(p.eval(x), 38.0);
    }
	
	#[test]
	fn eval_mul() {
		let p = Polynomial::from(vec![0.0, 2.0, 0.0]);
		let x = Matrix { m: vec![
					vec![3.0, 5.0, 2.0],
					vec![1.0, 0.0, 1.0],
					vec![3.0, 0.0, -1.0],
				]};
//   x^2
// 20.0, 15.0, 9.0
//  6.0,  5.0, 1.0
//  6.0, 15.0, 7.0
		assert_eq!(p.eval_mul(&x), Matrix { m: vec![
				vec![6.0, 10.0, 4.0],
				vec![2.0, 0.0, 2.0],
				vec![6.0, 0.0, -2.0],
			]});
		let p = Polynomial::from(vec![4.0, 5.0, 6.0]);
		assert_eq!(p.eval_mul(&x), Matrix { m: vec![
				vec![139.0, 115.0, 64.0],
				vec![41.0, 34.0, 11.0],
				vec![51.0, 90.0, 41.0],
			]});
		assert_eq!(
		Polynomial::from(vec![3, -2, 1]).eval_mul(&::matrix2x2::Matrix::new((
			(3, -2),
			(1, 9)
		))), ::matrix2x2::Matrix::new((
			(4, -20),
			(10, 64)
		)));
	}
	
	#[test]
	fn add() {
		let p1 = Polynomial::from(vec![3, -5, 2]);
		let p2 = Polynomial::from(vec![7, 3, -1, 3]);
		let s = Polynomial::from(vec![10, -2, 1, 3]);
		assert_eq!(p1.clone() + p2.clone(), s);
		assert_eq!(p2 + p1, s);
	}
	
	#[test]
	fn mul() {
		let p1 = Polynomial::from(vec![3, -5, 2]);
		let p2 = Polynomial::from(vec![7, 3, -1, 3]);
		let p = Polynomial::from(vec![21, -26, -4, 20, -17, 6]);
		assert_eq!(p1.clone() * p2.clone(), p);
		assert_eq!(p2 * p1, p);
	}
	
	#[test]
	fn display() {
		assert_eq!(
			Polynomial::from(vec![0, 2, 0, -5, 1]).to_string(),
			String::from("2x-5x3+x4")
		);
		assert_eq!(
			Polynomial::from(vec![1, 2, 3]).to_string(),
			String::from("1+2x+3x2")
		);
	}
	
	#[test]
	fn pm_monom() {
		let p = Polynomial::from(vec![2, 3, -1]);
		assert_eq!(p.clone()+6, Polynomial::from(vec![8, 3, -1]));
		assert_eq!(p.clone()-3, Polynomial::from(vec![-1, 3, -1]));
	}
	
	#[test]
	fn deriv() {
		assert_eq!(
			Polynomial::from(vec![0.3, 0.7, 0.2]).derivative(),
			Polynomial::from(vec![0.7, 0.4])
		);
		assert_eq!(
			Polynomial::from(vec![3, 7, 2]).derivative(),
			Polynomial::from(vec![7, 4])
		);
	}
	
	#[test]
	fn rem() {
		assert!(
			(Polynomial::from(vec![3.0, 5.1, 7.2, 2.0])
			% Polynomial::from(vec![1.5, 1.0])
			- Polynomial::from(vec![4.8_f64])).into_vec()[0].abs()
			< 1e-14
		);
	}
	
	#[test]
	fn gcd() {
		assert_eq!(
			Polynomial::from(vec![-9.0, 0.0, 1.0])
			._gcd(&Polynomial::from(vec![-6.0, 1.0, 1.0])),
			Polynomial::from(vec![3.0, 1.0])
		);
	}
	
	#[test]
	fn sturm() {
		assert_eq!(
			Polynomial::from(vec![-1.0, -1.0, 0.0, 1.0, 1.0]).sturm_sequence(),
			vec![
				Polynomial::from(vec![-1.0, -1.0, 0.0, 1.0, 1.0]),
				Polynomial::from(vec![-1.0, 0.0, 3.0, 4.0]),
				Polynomial::from(vec![15.0/16.0, 3.0/4.0, 3.0/16.0]),
				Polynomial::from(vec![-64.0, -32.0]),
				Polynomial::from(vec!(-3.0/16.0)),
			]
		);
	}
	
	#[test]
	fn roots() {
		let eps = 1e-14_f64;
		let r = Polynomial::from(vec![-4.0, 0.0, 1.0])
			.find_roots(-5.0, 5.0, eps);
		let r2 = vec![-2.0, 2.0];
		assert!(r.len() == r2.len() && r.into_iter().zip(r2.into_iter()).all(|(x, x2)| (x-x2).abs() <= eps));
	}
}
