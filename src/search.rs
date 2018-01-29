// Copyright 2018 Eduardo Sánchez Muñoz
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use ByteStr;

// Matcher
pub trait Matcher {}

// IntoMatcher
pub trait IntoMatcher {
    type Matcher: Matcher;
    
    fn into_matcher(self) -> Self::Matcher;
}

// PrefixMatcher
pub trait PrefixMatcher: Matcher {
    fn is_prefix_of(&self, haystack: &ByteStr) -> bool;
}

// SufixMatcher
pub trait SufixMatcher: Matcher {
    fn is_sufix_of(&self, haystack: &ByteStr) -> bool;
}

// FullMatcher
pub trait FullMatcher: Matcher {
    fn matches(&self, haystack: &ByteStr) -> bool;
}

// ForwardSearcher
pub trait ForwardSearcher: Matcher {
    fn find(&self, haystack: &ByteStr) -> Option<(usize, usize)>;
}

// ReverseSearcher
pub trait ReverseSearcher: Matcher {
    fn rfind(&self, haystack: &ByteStr) -> Option<(usize, usize)>;
}

// DoubleEndedSearcher
pub trait DoubleEndedSearcher: ForwardSearcher + ReverseSearcher {}

// StrMatcher
pub struct StrMatcher<'a> {
    needle: &'a ByteStr,
}

impl<'a> StrMatcher<'a> {
    #[inline]
    pub fn new(needle: &'a ByteStr) -> Self {
        Self { needle: needle }
    }
}

impl<'a> Matcher for StrMatcher<'a> {}

impl<'a, T: AsRef<[u8]>> IntoMatcher for &'a T {
    type Matcher = StrMatcher<'a>;
    
    #[inline]
    fn into_matcher(self) -> StrMatcher<'a> {
        StrMatcher::new(ByteStr::from_slice(self.as_ref()))
    }
}

impl<'a> PrefixMatcher for StrMatcher<'a> {
    fn is_prefix_of(&self, haystack: &ByteStr) -> bool {
        haystack.len() >= self.needle.len() && haystack[0 .. self.needle.len()] == self.needle[..]
    }
}

impl<'a> SufixMatcher for StrMatcher<'a> {
    fn is_sufix_of(&self, haystack: &ByteStr) -> bool {
        haystack.len() >= self.needle.len() &&
            haystack[haystack.len() - self.needle.len() .. haystack.len()] == self.needle[..]
    }
}

impl<'a> FullMatcher for StrMatcher<'a> {
    #[inline]
    fn matches(&self, haystack: &ByteStr) -> bool {
        haystack == self.needle
    }
}

impl<'a> ForwardSearcher for StrMatcher<'a> {
    fn find(&self, haystack: &ByteStr) -> Option<(usize, usize)> {
        for (i, window) in haystack.windows(self.needle.len()).enumerate() {
            if window == self.needle {
                return Some((i, i + self.needle.len()));
            }
        }
        None
    }
}

impl<'a> ReverseSearcher for StrMatcher<'a> {
    fn rfind(&self, haystack: &ByteStr) -> Option<(usize, usize)> {
        for (i, window) in haystack.windows(self.needle.len()).enumerate().rev() {
            if window == self.needle {
                return Some((i, i + self.needle.len()));
            }
        }
        None
    }
}

// ByteMatcher
pub struct ByteMatcher {
    needle: u8,
}

impl ByteMatcher {
    #[inline]
    pub fn new(needle: u8) -> Self {
        ByteMatcher { needle: needle }
    }
}

impl Matcher for ByteMatcher {}

impl IntoMatcher for u8 {
    type Matcher = ByteMatcher;
    
    fn into_matcher(self) -> ByteMatcher {
        ByteMatcher::new(self)
    }
}

impl PrefixMatcher for ByteMatcher {
    #[inline]
    fn is_prefix_of(&self, haystack: &ByteStr) -> bool {
        haystack.first().map_or(false, |&byte| byte == self.needle)
    }
}

impl SufixMatcher for ByteMatcher {
    #[inline]
    fn is_sufix_of(&self, haystack: &ByteStr) -> bool {
        haystack.last().map_or(false, |&byte| byte == self.needle)
    }
}

impl FullMatcher for ByteMatcher {
    #[inline]
    fn matches(&self, haystack: &ByteStr) -> bool {
        haystack == [self.needle]
    }
}

impl ForwardSearcher for ByteMatcher {
    fn find(&self, haystack: &ByteStr) -> Option<(usize, usize)> {
        for (i, &byte) in haystack.iter().enumerate() {
            if byte == self.needle {
                return Some((i, i + 1));
            }
        }
        None
    }
}

impl ReverseSearcher for ByteMatcher {
    fn rfind(&self, haystack: &ByteStr) -> Option<(usize, usize)> {
        for (i, &byte) in haystack.iter().enumerate().rev() {
            if byte == self.needle {
                return Some((i, i + 1));
            }
        }
        None
    }
}

impl DoubleEndedSearcher for ByteMatcher {}
