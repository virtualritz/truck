//! Reads/writes STEP files from/to truck.
//!
//! # Current Status
//!
//! It is possible to output data modeled by truck-modeling.
//! Shapes created by set operations cannot be output yet.
//! Input will come further down the road.

#![cfg_attr(not(debug_assertions), deny(warnings))]
#![deny(clippy::all, rust_2018_idioms)]
#![warn(
    missing_docs,
    missing_debug_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]

/// STEP input module
#[doc(hidden)]
pub mod r#in;
/// STEP output module
pub mod out;

#[doc(hidden)]
#[macro_export]
macro_rules! impl_from {
	($(impl From<&$refed: ty> for $converted: ty {
		$from_func: item
	})*) => {
		$(impl From<&$refed> for $converted {
			$from_func
		}
		impl From<$refed> for $converted {
			fn from(x: $refed) -> Self { Self::from(&x) }
		})*
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_try_from {
	($(impl TryFrom<&$refed: ty> for $converted: ty {
		$try_from_func: item
	})*) => {
		$(impl TryFrom<&$refed> for $converted {
            type Error = ExpressParseError;
			$try_from_func
		}
		impl TryFrom<$refed> for $converted {
            type Error = ExpressParseError;
            fn try_from(x: $refed) -> Result<Self, ExpressParseError> { Self::try_from(&x) }
		})*
	};
}
