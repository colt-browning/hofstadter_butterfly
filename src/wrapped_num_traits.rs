extern crate num;

pub trait Zero: ::std::ops::Add<Output=Self> where Self: Sized {
	fn zero() -> Self;
	fn is_zero(&self) -> bool;
}

impl<T> Zero for T where T: self::num::Zero {
	fn zero() -> Self {
		self::num::Zero::zero()
	}
	fn is_zero(&self) -> bool {
		self::num::Zero::is_zero(self)
	}
}

pub trait One: ::std::ops::Mul<Output=Self> where Self: Sized {
	fn one() -> Self;
	fn is_one(&self) -> bool;
}

impl<T> One for T where T: self::num::One + PartialEq {
	fn one() -> Self {
		self::num::One::one()
	}
	fn is_one(&self) -> bool {
		self::num::One::is_one(self)
	}
}

pub trait Signed {
// just one function (the corresponding num trait has more)
	fn is_positive(&self) -> bool;
}

impl<T> Signed for T where T: Zero + PartialOrd {
	fn is_positive(&self) -> bool {
		*self > T::zero()
	}
}
