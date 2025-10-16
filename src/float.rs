use core::ops::{RangeFrom, RangeToInclusive, RangeInclusive, RangeFull};

mod private {
	pub trait Sealed {}
}
use private::Sealed;

/// Adds the `limit` method for floating point types.
///
/// This is implemented on `f32`, `f64`, as well as `f16` and `f128` if the `nightly` feature is enabled.
///
/// Note that this behaves like `clamp`, not `min`/`max`. Specifically, if `self` is NaN, the result is NaN. NaN bounds are not supported and will panic.
///
/// ## Panics
/// Panics if any bound is NaN, or if the start bound is greater than the end bound.
///
/// ## Example
/// ```
/// # use limit::LimitFloat;
/// assert_eq!(5.0f32.limit(3.0..), 5.0);
/// assert_eq!(2.0f32.limit(3.0..), 3.0);
/// assert!(f32::NAN.limit(3.0..).is_nan());
///
/// assert_eq!(5.0f32.limit(..=7.0), 5.0);
/// assert_eq!(9.0f32.limit(..=7.0), 7.0);
/// assert!(f32::NAN.limit(..=7.0).is_nan());
///
/// assert_eq!(5.0f32.limit(3.0..=7.0), 5.0);
/// assert_eq!(2.0f32.limit(3.0..=7.0), 3.0);
/// assert_eq!(9.0f32.limit(3.0..=7.0), 7.0);
/// assert!(f32::NAN.limit(3.0..=7.0).is_nan());
///
/// assert_eq!(5.0f32.limit(..), 5.0);
/// assert_eq!(f32::INFINITY.limit(..), f32::INFINITY);
/// ```
pub trait LimitFloat: Sized {
	fn limit<B: LimitFloatBounds<Self>>(self, bounds: B) -> Self {
		bounds.limit_bounds(self)
	}
}

/// Types that can be used as bounds for `LimitFloat`.
pub trait LimitFloatBounds<T>: Sealed + Sized {
	#[doc(hidden)]
	fn limit_bounds(self, value: T) -> T;
}

impl Sealed for RangeFull {}
macro_rules! limit_float {
	($(#[$m:meta])* $t:ty) => {
		$(#[$m])* impl LimitFloat for $t {}

		$(#[$m])* impl Sealed for RangeFrom<$t> {}
		$(#[$m])* impl LimitFloatBounds<$t> for RangeFrom<$t> {
			#[doc(hidden)]
			fn limit_bounds(self, value: $t) -> $t {
				value.clamp(self.start, <$t>::INFINITY)
			}
		}

		$(#[$m])* impl Sealed for RangeToInclusive<$t> {}
		$(#[$m])* impl LimitFloatBounds<$t> for RangeToInclusive<$t> {
			#[doc(hidden)]
			fn limit_bounds(self, value: $t) -> $t {
				value.clamp(<$t>::NEG_INFINITY, self.end)
			}
		}

		$(#[$m])* impl Sealed for RangeInclusive<$t> {}
		$(#[$m])* impl LimitFloatBounds<$t> for RangeInclusive<$t> {
			#[doc(hidden)]
			fn limit_bounds(self, value: $t) -> $t {
				let (start, end) = self.into_inner();
				value.clamp(start, end)
			}
		}

		$(#[$m])* impl LimitFloatBounds<$t> for RangeFull {
			#[doc(hidden)]
			fn limit_bounds(self, value: $t) -> $t {
				value
			}
		}
	}
}

limit_float!(f32);
limit_float!(f64);
limit_float!(#[cfg(feature = "nightly")] #[doc(cfg(feature = "nightly"))] f16);
limit_float!(#[cfg(feature = "nightly")] #[doc(cfg(feature = "nightly"))] f128);
