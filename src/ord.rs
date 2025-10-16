use core::ops::{RangeFrom, RangeToInclusive, RangeInclusive, RangeFull};

mod private {
	pub trait Sealed {}
}
use private::Sealed;

/// Adds the `limit` method.
///
/// ## Panics
/// Panics if the start bound is greater than the end bound.
///
/// ## Example
///```
/// # use limit::Limit;
/// assert_eq!(5i32.limit(3..), 5);
/// assert_eq!(2i32.limit(3..), 3);
///
/// assert_eq!(5i32.limit(..=7), 5);
/// assert_eq!(9i32.limit(..=7), 7);
///
/// assert_eq!(5i32.limit(3..=7), 5);
/// assert_eq!(2i32.limit(3..=7), 3);
/// assert_eq!(9i32.limit(3..=7), 7);
///
/// assert_eq!(5i32.limit(..), 5);
/// assert_eq!(i32::MIN.limit(..), i32::MIN);
/// ```
pub trait Limit: Ord + Sized {
	fn limit<B: LimitBounds<Self>>(self, bounds: B) -> Self {
		bounds.limit_bounds(self)
	}
}

impl<T: Ord> Limit for T {}

/// Types that can be used as bounds for `Limit`.
pub trait LimitBounds<T>: Sealed + Sized {
	#[doc(hidden)]
	fn limit_bounds(self, value: T) -> T;
}

impl<T: Ord> Sealed for RangeFrom<T> {}
impl<T: Ord> LimitBounds<T> for RangeFrom<T> {
	#[doc(hidden)]
	fn limit_bounds(self, value: T) -> T {
		value.max(self.start)
	}
}

impl<T: Ord> Sealed for RangeToInclusive<T> {}
impl<T: Ord> LimitBounds<T> for RangeToInclusive<T> {
	#[doc(hidden)]
	fn limit_bounds(self, value: T) -> T {
		value.min(self.end)
	}
}

impl<T: Ord> Sealed for RangeInclusive<T> {}
impl<T: Ord> LimitBounds<T> for RangeInclusive<T> {
	#[doc(hidden)]
	fn limit_bounds(self, value: T) -> T {
		let (start, end) = self.into_inner();
		value.clamp(start, end)
	}
}

impl Sealed for RangeFull {}
impl<T: Ord> LimitBounds<T> for RangeFull {
	#[doc(hidden)]
	fn limit_bounds(self, value: T) -> T {
		value
	}
}
