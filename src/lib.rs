// Copyright 2018 Eduardo Sánchez Muñoz
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#![cfg_attr(feature="no_std", no_std)]

#[cfg(not(feature="no_std"))]
use std as core;

mod byte_str;
pub use byte_str::ByteStr;

#[cfg(not(feature="no_std"))]
mod byte_string;
#[cfg(not(feature="no_std"))]
pub use byte_string::ByteString;

pub mod iterators;

mod search;

pub use search::Matcher;
pub use search::IntoMatcher;

pub use search::PrefixMatcher;
pub use search::SufixMatcher;
pub use search::FullMatcher;
pub use search::ForwardSearcher;
pub use search::ReverseSearcher;
pub use search::DoubleEndedSearcher;

pub use search::StrMatcher;
pub use search::ByteMatcher;

#[cfg(test)]
mod tests;
