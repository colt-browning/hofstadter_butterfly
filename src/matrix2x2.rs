use ::{Zero, One};
use std::ops;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix<T> {
	m: ((T, T), (T, T)),
}

impl<T> Matrix<T> {
	pub fn new(m: ((T, T), (T, T))) -> Self {
		Matrix {
			m,
		}
	}
	
	pub fn _into_tuple(self) -> ((T, T), (T, T)) {
		self.m
	}
}

impl<T> Matrix<T> where T: ops::Add<Output = T> {
	pub fn trace(self) -> T {
		(self.m.0).0 + (self.m.1).1
	}
}

impl<T> Matrix<T> where T: ops::Sub<Output = T> + ops::Mul<Output = T> {
	pub fn _det(self) -> T {
		(self.m.0).0 * (self.m.1).1 - (self.m.0).1 * (self.m.1).0
	}
}

impl<T> ops::Add for Matrix<T> where T: ops::Add<Output = T> {
	type Output = Self;
	fn add(self, rhs: Self) -> Self {
		Matrix {
			m: (
				(
					(self.m.0).0 + (rhs.m.0).0,
					(self.m.0).1 + (rhs.m.0).1,
				),
				(
					(self.m.1).0 + (rhs.m.1).0,
					(self.m.1).1 + (rhs.m.1).1,
				)
			),
		}
	}
}

impl<T> ops::Mul for Matrix<T> where T: Clone + ops::Add<Output = T> + ops::Mul<Output = T> {
	type Output = Self;
	fn mul(self, rhs: Self) -> Self {
		Matrix {
			m: (
				(
					((self.m.0).0).clone() * ((rhs.m.0).0).clone() + ((self.m.0).1).clone() * ((rhs.m.1).0).clone(),
					(self.m.0).0 * ((rhs.m.0).1).clone() + (self.m.0).1 * ((rhs.m.1).1).clone(),
				),
				(
					((self.m.1).0).clone() * (rhs.m.0).0 + ((self.m.1).1).clone() * (rhs.m.1).0,
					(self.m.1).0 * (rhs.m.0).1 + (self.m.1).1 * (rhs.m.1).1,
				)
			),
		}
	}
}

impl<'a, T> ops::Mul<&'a T> for Matrix<T> where T: ops::Mul<&'a T, Output = T> {
// currently used in tests only
	type Output = Self;
	fn mul(self, rhs: &'a T) -> Matrix<T> {
		Matrix {
			m: (
				(
					(self.m.0).0 * rhs,
					(self.m.0).1 * rhs,
				),
				(
					(self.m.1).0 * rhs,
					(self.m.1).1 * rhs,
				)
			),
		}
	}
}

impl<T> One for Matrix<T> where T: Clone + One + Zero + PartialEq {
	fn one() -> Self {
		Self::new((
			(T::one(), T::zero()),
			(T::zero(), T::one())
		))
	}
	
	fn is_one(&self) -> bool {
		(self.m.0).0.is_one() && (self.m.0).1.is_zero() && (self.m.1).0.is_zero() && (self.m.1).1.is_one()
	}
}

impl<T> Zero for Matrix<T> where T: Zero {
	fn zero() -> Self {
		Self::new((
			(T::zero(), T::zero()),
			(T::zero(), T::zero())
		))
	}
	
	fn is_zero(&self) -> bool {
		(self.m.0).0.is_zero() && (self.m.0).1.is_zero() && (self.m.1).0.is_zero() && (self.m.1).1.is_zero()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn add() {
		let m1 = Matrix::new((
		(2, 5),
		(-2, 4)));
		let m2 = Matrix::new((
		(0, 7),
		(11, 8)));
		assert_eq!(m1 + m2, Matrix::new((
		(2, 12),
		(9, 12)
		)));
	}
	
	#[test]
	fn mul() {
		let m1 = Matrix::new((
		(2, 5),
		(-2, 4)));
		let m2 = Matrix::new((
		(0, 7),
		(11, 8)));
		assert_eq!(m1 * m2, Matrix::new((
		(55, 54),
		(44, 18)
		)));
		assert_eq!(m2 * m1, Matrix::new((
		(-14, 28),
		(6, 87)
		)));
		assert_eq!(m1 * Matrix::one(), m1);
		assert_eq!(Matrix::one() * m1, m1);
	}
	
	#[test]
	fn det() {
		assert_eq!(Matrix::new((
			(2, 3),
			(4, 5)))._det(),
			-2);
	}
}
