// Copyright 2018 Eduardo Sánchez Muñoz
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms. 

use core;
#[cfg(not(feature="no_std"))]
use std;

#[cfg(not(feature="no_std"))]
use ByteString;

use iterators;

use IntoMatcher;
use PrefixMatcher;
use SufixMatcher;
use ForwardSearcher;
use ReverseSearcher;

/// Borrowed reference to a byte string. It provides similar functionality as `str`
/// and `[u8]`.
#[derive(PartialEq, Eq)]
pub struct ByteStr {
    inner: [u8],
}

impl ByteStr {
    /// Creates an empty `ByteStr`.
    #[inline]
    pub fn empty<'a>() -> &'a Self {
        Self::from_slice(&[])
    }
    
    /// Creates an empty mutable `ByteStr`.
    #[inline]
    pub fn empty_mut<'a>() -> &'a mut Self {
        Self::from_slice_mut(&mut [])
    }
    
    /// Creates a `ByteStr` from a byte slice.
    #[inline]
    pub fn from_slice(bytes: &[u8]) -> &Self {
        unsafe { core::mem::transmute(bytes) }
    }
    
    /// Create a mutable `ByteStr` from a byte slice.
    #[inline]
    pub fn from_slice_mut(bytes: &mut [u8]) -> &mut Self {
        unsafe { core::mem::transmute(bytes) }
    }
    
    /// Forms a `ByteStr` from a pointer and a length.
    #[inline]
    pub unsafe fn from_raw_parts<'a>(ptr: *const u8, len: usize) -> &'a Self {
        ByteStr::from_slice(core::slice::from_raw_parts(ptr, len))
    }
    
    /// Forms a mutable `ByteStr` from a pointer and a length.
    #[inline]
    pub unsafe fn from_raw_parts_mut<'a>(ptr: *mut u8, len: usize) -> &'a mut Self {
        ByteStr::from_slice_mut(core::slice::from_raw_parts_mut(ptr, len))
    }
    
    /// Converts `self` into a byte slice.
    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        &self.inner
    }
    
    /// Converts `self` into a mutable byte slice.
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.inner
    }
    
    /// Copies the `self` into a `Vec`.
    #[cfg(not(feature="no_std"))]
    #[inline]
    pub fn to_vec(&self) -> Vec<u8> {
        self.as_slice().to_vec()
    }
    
    /// Copies the `self` into a `ByteString`.
    #[cfg(not(feature="no_std"))]
    #[inline]
    pub fn to_byte_string(&self) -> ByteString {
        ByteString::from_vec(self.to_vec())
    }
    
    /// Converts `self` into a boxed slice without clones or allocation.
    #[cfg(not(feature="no_std"))]
    pub fn into_boxed_slice(self: Box<Self>) -> Box<[u8]> {
        unsafe { Box::from_raw(Box::into_raw(self) as *mut [u8]) }
    }
    
    /// Converts `self` into a vector without clones or allocation.
    #[cfg(not(feature="no_std"))]
    pub fn into_vec(self: Box<Self>) -> Vec<u8> {
        self.into_boxed_slice().into_vec()
    }
    
    /// Converts `self` into a `ByteString` without clones or allocation.
    #[cfg(not(feature="no_std"))]
    pub fn into_byte_string(self: Box<Self>) -> ByteString {
        ByteString::from_vec(self.into_vec())
    }
    
    /// Returns the length of `self`.
    #[inline]
    pub fn len(&self) -> usize {
        self.as_slice().len()
    }
    
    /// Returns `true` if the length of `self` is zero.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    
    /// Converts `self` into a raw pointer that points to the first byte of the string.
    #[inline]
    pub fn as_ptr(&self) -> *const u8 {
        self.as_slice().as_ptr()
    }
    
    /// Converts `self` into a mutable raw pointer that points to the first byte of the string.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.as_mut_slice().as_mut_ptr()
    }
    
    /// Returns a reference to an element of the slice, or `None` if the
    /// index is out of bounds.
    #[inline]
    pub fn get(&self, index: usize) -> Option<&u8> {
        self.as_slice().get(index)
    }
    
    /// Returns a mutable reference to an element of the slice, or `None` if the
    /// index is out of bounds.
    #[inline]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut u8> {
        self.as_mut_slice().get_mut(index)
    }
    
    /// Returns a reference to the first byte of the string, or `None` if it is empty.
    #[inline]
    pub fn first(&self) -> Option<&u8> {
        self.as_slice().first()
    }
    
    /// Returns a mutable reference to the first byte of the string, or `None` if it is empty.
    #[inline]
    pub fn first_mut(&mut self) -> Option<&mut u8> {
        self.as_mut_slice().first_mut()
    }
    
    /// Returns a reference to the last byte of the string, or `None` if it is empty.
    #[inline]
    pub fn last(&self) -> Option<&u8> {
        self.as_slice().last()
    }
    
    /// Returns a mutable reference to the last byte of the string, or `None` if it is empty.
    #[inline]
    pub fn last_mut(&mut self) -> Option<&mut u8> {
        self.as_mut_slice().last_mut()
    }
    
    /// Returns the first and all the rest of the bytes of the slice, or `None` if it is empty.
    #[inline]
    pub fn split_first(&self) -> Option<(&u8, &ByteStr)> {
        self.as_slice().split_first().map(|(f, r)| (f, Self::from_slice(r)))
    }
    
    /// Returns the first and all the rest of the bytes of the slice, or `None` if it is empty.
    #[inline]
    pub fn split_first_mut(&mut self) -> Option<(&mut u8, &mut ByteStr)> {
        self.as_mut_slice().split_first_mut().map(|(f, r)| (f, Self::from_slice_mut(r)))
    }
    
    /// Returns the last and all the rest of the bytes of the slice, or `None` if it is empty.
    #[inline]
    pub fn split_last(&self) -> Option<(&u8, &ByteStr)> {
        self.as_slice().split_last().map(|(f, r)| (f, Self::from_slice(r)))
    }
    
    /// Returns the last and all the rest of the bytes of the slice, or `None` if it is empty.
    #[inline]
    pub fn split_last_mut(&mut self) -> Option<(&mut u8, &mut ByteStr)> {
        self.as_mut_slice().split_last_mut().map(|(f, r)| (f, Self::from_slice_mut(r)))
    }
    
    /// Returns an iterator over the string.
    #[inline]
    pub fn iter<'a>(&'a self) -> core::slice::Iter<'a, u8> {
        self.as_slice().iter()
    }
    
    /// Returns an iterator that allows modifying each value.
    #[inline]
    pub fn iter_mut<'a>(&'a mut self) -> core::slice::IterMut<'a, u8> {
        self.as_mut_slice().iter_mut()
    }
    
    /// Returns an iterator over all contiguous windows of length `size`.
    /// The windows overlap. If the string is shorter than `size`, the
    /// iterator returns no values.
    ///
    /// Similar to `slice::windows()`.
    #[inline]
    pub fn windows<'a>(&'a self, size: usize) -> iterators::Windows<'a> {
        iterators::Windows::new(self, size)
    }
    
    /// Returns an iterator over `size` bytes of the string at a time. The chunks do not
    /// overlap. If size does not divide the length of the slice, then the last chunk will
    /// not have length `size`.
    ///
    /// Similar to `slice::chunks()`.
    #[inline]
    pub fn chunks<'a>(&'a self, size: usize) -> iterators::Chunks<'a> {
        iterators::Chunks::new(self, size)
    }
    
    /// Returns an iterator over `size` elements of the slice at a time. The chunks are mutable
    /// strings and do not overlap. If `size` does not divide the length of the slice, then the
    /// last chunk will not have length `size`.
    ///
    /// Similar to `slice::chunks_mut()`.
    #[inline]
    pub fn chunks_mut<'a>(&'a mut self, size: usize) -> iterators::ChunksMut<'a> {
        iterators::ChunksMut::new(self, size)
    }
    
    /// Divides one string into two at an index.
    ///
    /// The first will contain all indices from `[0, mid)` (excluding the index `mid` itself) and
    /// the second will contain all indices from `[mid, len)` (excluding the index `len` itself).
    ///
    /// Similar to `slice::split_at()`.
    ///
    /// # Panics
    ///
    /// Panics if `mid > len`.
    #[inline]
    pub fn split_at(&self, mid: usize) -> (&ByteStr, &ByteStr) {
        let (first, second) = self.as_slice().split_at(mid);
        (ByteStr::from_slice(first), ByteStr::from_slice(second))
    }
    
    /// Divides one `&mut` string into two at an index.
    ///
    /// The first will contain all indices from `[0, mid)` (excluding the index `mid` itself) and
    /// the second will contain all indices from `[mid, len)` (excluding the index `len` itself).
    ///
    /// Similar to `slice::split_at_mut()`.
    ///
    /// # Panics
    ///
    /// Panics if `mid > len`.
    #[inline]
    pub fn split_at_mut(&mut self, mid: usize) -> (&mut ByteStr, &mut ByteStr) {
        let (first, second) = self.as_mut_slice().split_at_mut(mid);
        (ByteStr::from_slice_mut(first), ByteStr::from_slice_mut(second))
    }
    
    /// Returns an iterator over substrings of this string, separated by a matcher.
    #[inline]
    pub fn split<'a, M: IntoMatcher>(&'a self, m: M) -> iterators::Split<'a, M::Matcher>
        where <M as IntoMatcher>::Matcher: ForwardSearcher
    {
        iterators::Split::new(self, m.into_matcher())
    }
    
    /// Returns an iterator over mutable substrings of this string, separated by a matcher.
    #[inline]
    pub fn split_mut<'a, M: IntoMatcher>(&'a mut self, m: M) -> iterators::SplitMut<'a, M::Matcher>
        where <M as IntoMatcher>::Matcher: ForwardSearcher
    {
        iterators::SplitMut::new(self, m.into_matcher())
    }
    
    /// Returns an iterator over substrings of this string, separated by a matcher,
    /// starting at the end of the slice and working backwards.
    #[inline]
    pub fn rsplit<'a, M: IntoMatcher>(&'a self, m: M) -> iterators::RSplit<'a, M::Matcher>
        where <M as IntoMatcher>::Matcher: ReverseSearcher
    {
        iterators::RSplit::new(self, m.into_matcher())
    }
    
    /// Returns an iterator over mutable substrings of this string, separated by a matcher,
    /// starting at the end of the slice and working backwards.
    #[inline]
    pub fn rsplit_mut<'a, M: IntoMatcher>(&'a mut self, m: M) -> iterators::RSplitMut<'a, M::Matcher>
        where <M as IntoMatcher>::Matcher: ReverseSearcher
    {
        iterators::RSplitMut::new(self, m.into_matcher())
    }
    
    /// Returns an iterator over substrings of this string, separated by a matcher, returning
    /// at most `n` items.
    ///
    /// If `n` substrings are returned, the last substring will contain the remainder of the string.
    #[inline]
    pub fn splitn<'a, M: IntoMatcher>(&'a self, n: usize, m: M) -> iterators::SplitN<'a, M::Matcher>
        where <M as IntoMatcher>::Matcher: ForwardSearcher
    {
        iterators::SplitN::new(self, n, m.into_matcher())
    }
    
    /// Returns an iterator over mutable substrings of this string, separated by a matcher, returning
    /// at most `n` items.
    ///
    /// If `n` substrings are returned, the last substring will contain the remainder of the string.
    #[inline]
    pub fn splitn_mut<'a, M: IntoMatcher>(&'a mut self, n: usize, m: M) -> iterators::SplitNMut<'a, M::Matcher>
        where <M as IntoMatcher>::Matcher: ForwardSearcher
    {
        iterators::SplitNMut::new(self, n, m.into_matcher())
    }
    
    /// Returns an iterator over substrings of this string, separated by a matcher and stating from
    /// the end of the string, returning at most `n` items.
    ///
    /// If `n` substrings are returned, the last substring will contain the remainder of the string.
    #[inline]
    pub fn rsplitn<'a, M: IntoMatcher>(&'a self, n: usize, m: M) -> iterators::RSplitN<'a, M::Matcher>
        where <M as IntoMatcher>::Matcher: ReverseSearcher
    {
        iterators::RSplitN::new(self, n, m.into_matcher())
    }
    
    /// Returns an iterator over mutable substrings of this string, separated by a matcher and stating from
    /// the end of the string, returning at most `n` items.
    ///
    /// If `n` substrings are returned, the last substring will contain the remainder of the string.
    #[inline]
    pub fn rsplitn_mut<'a, M: IntoMatcher>(&'a mut self, n: usize, m: M) -> iterators::RSplitNMut<'a, M::Matcher>
        where <M as IntoMatcher>::Matcher: ReverseSearcher
    {
        iterators::RSplitNMut::new(self, n, m.into_matcher())
    }
    
    /// Returns an iterator over the disjoint matches within the given string.
    #[inline]
    pub fn matches<'a, M: IntoMatcher>(&'a self, m: M) -> iterators::Matches<'a, M::Matcher>
        where <M as IntoMatcher>::Matcher: ForwardSearcher
    {
        iterators::Matches::new(self, m.into_matcher())
    }
    
    /// Returns an iterator over the mutable disjoint matches within the given string.
    #[inline]
    pub fn matches_mut<'a, M: IntoMatcher>(&'a mut self, m: M) -> iterators::MatchesMut<'a, M::Matcher>
        where <M as IntoMatcher>::Matcher: ForwardSearcher
    {
        iterators::MatchesMut::new(self, m.into_matcher())
    }
    
    /// Returns an iterator over the disjoint matches within the given string, yielded in reverse order.
    #[inline]
    pub fn rmatches<'a, M: IntoMatcher>(&'a self, m: M) -> iterators::RMatches<'a, M::Matcher>
        where <M as IntoMatcher>::Matcher: ReverseSearcher
    {
        iterators::RMatches::new(self, m.into_matcher())
    }
    
    /// Returns an iterator over the mutable disjoint matches within the given string, yielded
    /// in reverse order.
    #[inline]
    pub fn rmatches_mut<'a, M: IntoMatcher>(&'a mut self, m: M) -> iterators::RMatchesMut<'a, M::Matcher>
        where <M as IntoMatcher>::Matcher: ReverseSearcher
    {
        iterators::RMatchesMut::new(self, m.into_matcher())
    }
    
    /// Returns an iterator over the disjoint matches within the given string, as well as the index
    /// that the match starts at.
    #[inline]
    pub fn match_indices<'a, M: IntoMatcher>(&'a self, m: M) -> iterators::MatchIndices<'a, M::Matcher>
        where <M as IntoMatcher>::Matcher: ForwardSearcher
    {
        iterators::MatchIndices::new(self, m.into_matcher())
    }
    
    /// Returns an iterator over the mutable disjoint matches within the given string, as well as
    /// the index that the match starts at.
    #[inline]
    pub fn match_indices_mut<'a, M: IntoMatcher>(&'a mut self, m: M) -> iterators::MatchIndicesMut<'a, M::Matcher>
        where <M as IntoMatcher>::Matcher: ForwardSearcher
    {
        iterators::MatchIndicesMut::new(self, m.into_matcher())
    }
    
    /// Returns an iterator over the disjoint matches within the given string, yielded in reverse order,
    /// as well as the index that the match starts at.
    #[inline]
    pub fn rmatch_indices<'a, M: IntoMatcher>(&'a self, m: M) -> iterators::RMatchIndices<'a, M::Matcher>
        where <M as IntoMatcher>::Matcher: ReverseSearcher
    {
        iterators::RMatchIndices::new(self, m.into_matcher())
    }
    
    /// Returns an iterator over the mutable disjoint matches within the given string, yielded in reverse
    /// order, as well as the index that the match starts at.
    #[inline]
    pub fn rmatch_indices_mut<'a, M: IntoMatcher>(&'a mut self, m: M) -> iterators::RMatchIndicesMut<'a, M::Matcher>
        where <M as IntoMatcher>::Matcher: ReverseSearcher
    {
        iterators::RMatchIndicesMut::new(self, m.into_matcher())
    }
    
    /// Returns `true` if the string contains a substring that matches the given matcher.
    #[inline]
    pub fn contains<M: IntoMatcher>(&self, m: M) -> bool
        where <M as IntoMatcher>::Matcher: ForwardSearcher
    {
        m.into_matcher().find(self).is_some()
    }
    
    /// Returns `true` if the string beginning a matches the given matcher.
    #[inline]
    pub fn starts_with<M: IntoMatcher>(&self, m: M) -> bool
        where <M as IntoMatcher>::Matcher: PrefixMatcher
    {
        m.into_matcher().is_prefix_of(self)
    }
    
    /// Returns `true` if the string ending a matches the given matcher.
    #[inline]
    pub fn ends_with<M: IntoMatcher>(&self, m: M) -> bool
        where <M as IntoMatcher>::Matcher: SufixMatcher
    {
        m.into_matcher().is_sufix_of(self)
    }
    
    /// Returns the byte index of the first character of `self` that matches the
    /// matcher or `None` it it doesn't match.
    #[inline]
    pub fn find<M: IntoMatcher>(&self, m: M) -> Option<usize>
        where <M as IntoMatcher>::Matcher: ForwardSearcher
    {
        m.into_matcher().find(self).map(|(a, _)| a)
    }
    
    /// Returns the byte index of the last character of `self` that matches the
    /// matcher or `None` it it doesn't match.
    #[inline]
    pub fn rfind<M: IntoMatcher>(&self, m: M) -> Option<usize>
        where <M as IntoMatcher>::Matcher: ReverseSearcher
    {
        m.into_matcher().rfind(self).map(|(_, b)| b)
    }
    
    /// Swaps two bytes in the string, indexed by `a` and `b`.
    ///
    /// # Panics
    ///
    /// Panics if `a` or `b` are out of bounds.
    #[inline]
    pub fn swap(&mut self, a: usize, b: usize) {
        self.as_mut_slice().swap(a, b);
    }
    
    /// Reverses the order of bytes in the slice.
    #[inline]
    pub fn reverse(&mut self) {
        self.as_mut_slice().reverse();
    }
    
    /// Copies all elements from `src` into `self`, using a memcpy.
    ///
    /// The length of `src` must be the same as `self`.
    #[inline]
    pub fn copy_from_slice(&mut self, src: &[u8]) {
        self.as_mut_slice().copy_from_slice(src);
    }
    
    /// Copies all elements from `src` into `self`, using a memcpy.
    ///
    /// The length of `src` must be the same as `self`.
    #[inline]
    pub fn copy_from_byte_str(&mut self, src: &ByteStr) {
        self.as_mut_slice().copy_from_slice(src.as_slice());
    }
}

// Default
impl<'a> Default for &'a ByteStr {
    #[inline]
    fn default() -> Self {
        ByteStr::empty()
    }
}

impl<'a> Default for &'a mut ByteStr {
    #[inline]
    fn default() -> Self {
        ByteStr::empty_mut()
    }
}

// Debug
impl core::fmt::Debug for ByteStr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        use core::fmt::Write;
        
        fn to_hex(nibble: u8) -> u8 {
            if nibble < 10 {
                b'0' + nibble
            } else {
                b'a' + nibble - 10
            }
        }
        
        f.write_str("b\"")?;
        for &byte in self.iter() {
            match byte {
                b'\t' => f.write_str("\\t")?,
                b'\r' => f.write_str("\\r")?,
                b'\n' => f.write_str("\\n")?,
                b'\\' => f.write_str("\\\\")?,
                b'"' => f.write_str("\\\"")?,
                0x20 ... 0x7E => f.write_char(byte as char)?,
                _ => {
                    f.write_str("\\x")?;
                    f.write_char(to_hex(byte >> 4) as char)?;
                    f.write_char(to_hex(byte & 0xF) as char)?;
                }
            }
        }
        f.write_char('"')?;
        Ok(())
    }
}

// ToOwned
#[cfg(not(feature="no_std"))]
impl std::borrow::ToOwned for ByteStr {
    type Owned = ByteString;
    
    #[inline]
    fn to_owned(&self) -> ByteString {
        self.to_byte_string()
    }
}

// AsRef
impl core::convert::AsRef<ByteStr> for ByteStr {
    #[inline]
    fn as_ref(&self) -> &ByteStr {
        self
    }
}

impl core::convert::AsRef<[u8]> for ByteStr {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

// AsMut
impl core::convert::AsMut<ByteStr> for ByteStr {
    #[inline]
    fn as_mut(&mut self) -> &mut ByteStr {
        self
    }
}

impl core::convert::AsMut<[u8]> for ByteStr {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8] {
        self.as_mut_slice()
    }
}

// PartialEq
impl<'a> core::cmp::PartialEq<ByteStr> for &'a ByteStr {
    #[inline]
    fn eq(&self, other: &ByteStr) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl core::cmp::PartialEq<[u8]> for ByteStr {
    #[inline]
    fn eq(&self, other: &[u8]) -> bool {
        self.as_slice() == other
    }
}

impl<'a> core::cmp::PartialEq<[u8]> for &'a ByteStr {
    #[inline]
    fn eq(&self, other: &[u8]) -> bool {
        self.as_slice() == other
    }
}

macro_rules! impl_partial_eq_array {
    ($size:expr) => {
        impl core::cmp::PartialEq<[u8; $size]> for ByteStr {
            #[inline]
            fn eq(&self, other: &[u8; $size]) -> bool {
                self.as_slice() == other
            }
        }
        
        impl<'a> core::cmp::PartialEq<[u8; $size]> for &'a ByteStr {
            #[inline]
            fn eq(&self, other: &[u8; $size]) -> bool {
                self.as_slice() == other
            }
        }
    }
}

impl_partial_eq_array!(0);
impl_partial_eq_array!(1);
impl_partial_eq_array!(2);
impl_partial_eq_array!(3);
impl_partial_eq_array!(4);
impl_partial_eq_array!(5);
impl_partial_eq_array!(6);
impl_partial_eq_array!(7);
impl_partial_eq_array!(8);
impl_partial_eq_array!(9);
impl_partial_eq_array!(10);
impl_partial_eq_array!(11);
impl_partial_eq_array!(12);
impl_partial_eq_array!(13);
impl_partial_eq_array!(14);
impl_partial_eq_array!(15);
impl_partial_eq_array!(16);
impl_partial_eq_array!(17);
impl_partial_eq_array!(18);
impl_partial_eq_array!(19);
impl_partial_eq_array!(20);
impl_partial_eq_array!(21);
impl_partial_eq_array!(22);
impl_partial_eq_array!(23);
impl_partial_eq_array!(24);
impl_partial_eq_array!(25);
impl_partial_eq_array!(26);
impl_partial_eq_array!(27);
impl_partial_eq_array!(28);
impl_partial_eq_array!(29);
impl_partial_eq_array!(30);
impl_partial_eq_array!(31);
impl_partial_eq_array!(32);

#[cfg(not(feature="no_std"))]
impl core::cmp::PartialEq<ByteString> for ByteStr {
    #[inline]
    fn eq(&self, other: &ByteString) -> bool {
        self.as_slice() == other.as_slice()
    }
}

#[cfg(not(feature="no_std"))]
impl<'a> core::cmp::PartialEq<ByteString> for &'a ByteStr {
    #[inline]
    fn eq(&self, other: &ByteString) -> bool {
        self.as_slice() == other.as_slice()
    }
}

#[cfg(not(feature="no_std"))]
impl<'b> core::cmp::PartialEq<std::borrow::Cow<'b, ByteStr>> for ByteStr {
    #[inline]
    fn eq(&self, other: &std::borrow::Cow<'b, ByteStr>) -> bool {
        self.as_slice() == other.as_slice()
    }
}

#[cfg(not(feature="no_std"))]
impl<'a, 'b> core::cmp::PartialEq<std::borrow::Cow<'b, ByteStr>> for &'a ByteStr {
    #[inline]
    fn eq(&self, other: &std::borrow::Cow<'b, ByteStr>) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<'a, T: ?Sized> core::cmp::PartialEq<&'a T> for ByteStr
    where ByteStr: core::cmp::PartialEq<T>
{
    #[inline]
    fn eq(&self, other: &&'a T) -> bool {
        self == *other
    }
}

// Index
impl core::ops::Index<usize> for ByteStr {
    type Output = u8;
    
    #[inline]
    fn index(&self, index: usize) -> &u8 {
        &self.as_slice()[index]
    }
}

impl core::ops::Index<core::ops::Range<usize>> for ByteStr {
    type Output = ByteStr;
    
    #[inline]
    fn index(&self, index: core::ops::Range<usize>) -> &ByteStr {
        ByteStr::from_slice(&self.as_slice()[index])
    }
}

impl core::ops::Index<core::ops::RangeFrom<usize>> for ByteStr {
    type Output = ByteStr;
    
    #[inline]
    fn index(&self, index: core::ops::RangeFrom<usize>) -> &ByteStr {
        ByteStr::from_slice(&self.as_slice()[index])
    }
}

impl core::ops::Index<core::ops::RangeTo<usize>> for ByteStr {
    type Output = ByteStr;
    
    #[inline]
    fn index(&self, index: core::ops::RangeTo<usize>) -> &ByteStr {
        ByteStr::from_slice(&self.as_slice()[index])
    }
}

impl core::ops::Index<core::ops::RangeFull> for ByteStr {
    type Output = ByteStr;
    
    #[inline]
    fn index(&self, index: core::ops::RangeFull) -> &ByteStr {
        ByteStr::from_slice(&self.as_slice()[index])
    }
}

// IndexMux
impl core::ops::IndexMut<usize> for ByteStr {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut u8 {
        &mut self.as_mut_slice()[index]
    }
}

impl core::ops::IndexMut<core::ops::Range<usize>> for ByteStr {
    #[inline]
    fn index_mut(&mut self, index: core::ops::Range<usize>) -> &mut ByteStr {
        ByteStr::from_slice_mut(&mut self.as_mut_slice()[index])
    }
}

impl core::ops::IndexMut<core::ops::RangeFrom<usize>> for ByteStr {
    #[inline]
    fn index_mut(&mut self, index: core::ops::RangeFrom<usize>) -> &mut ByteStr {
        ByteStr::from_slice_mut(&mut self.as_mut_slice()[index])
    }
}

impl core::ops::IndexMut<core::ops::RangeTo<usize>> for ByteStr {
    #[inline]
    fn index_mut(&mut self, index: core::ops::RangeTo<usize>) -> &mut ByteStr {
        ByteStr::from_slice_mut(&mut self.as_mut_slice()[index])
    }
}

impl core::ops::IndexMut<core::ops::RangeFull> for ByteStr {
    #[inline]
    fn index_mut(&mut self, index: core::ops::RangeFull) -> &mut ByteStr {
        ByteStr::from_slice_mut(&mut self.as_mut_slice()[index])
    }
}

// IntoIterator
impl<'a> core::iter::IntoIterator for &'a ByteStr {
    type Item = &'a u8;
    type IntoIter = core::slice::Iter<'a, u8>;
    
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> core::iter::IntoIterator for &'a mut ByteStr {
    type Item = &'a mut u8;
    type IntoIter = core::slice::IterMut<'a, u8>;
    
    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
