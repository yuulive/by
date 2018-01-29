// Copyright 2018 Eduardo Sánchez Muñoz
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std;
use ByteStr;

// An owned byte string. It provides similar functionality as `String`
/// and `Vec<u8>`.
#[derive(Clone, Default, PartialEq, Eq)]
pub struct ByteString {
    inner: Vec<u8>,
}

impl ByteString {
    /// Creates a new empty `ByteString` without allocating any memory.
    #[inline]
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }
    
    /// Converts a vector of bytes into a `ByteString` without clones or allocation.
    #[inline]
    pub fn from_vec(vec: Vec<u8>) -> Self {
        Self { inner: vec }
    }
    
    /// Creates a `ByteString` from a slice.
    #[inline]
    pub fn from_slice(slice: &[u8]) -> Self {
        Self::from_vec(slice.to_vec())
    }
    
    /// Returns a reference to the underlying byte vector.
    #[inline]
    pub fn as_vec(&self) -> &Vec<u8> {
        &self.inner
    }
    
    /// Returns a mutable reference to the underlying byte vector.
    #[inline]
    pub fn as_mut_vec(&mut self) -> &mut Vec<u8> {
        &mut self.inner
    }
    
    /// Converts `self` into a vector without clones or allocation.
    #[inline]
    pub fn into_vec(self) -> Vec<u8> {
        self.inner
    }
    
    /// Converts `self` into a boxed slice, dropping any excess capacity.
    #[inline]
    pub fn into_boxed_slice(self) -> Box<[u8]> {
        self.into_vec().into_boxed_slice()
    }
    
    /// Converts `self` into a boxed `ByteStr`, dropping any excess capacity.
    #[inline]
    pub fn into_boxed_byte_str(self) -> Box<ByteStr> {
        unsafe { Box::from_raw(Box::into_raw(self.into_boxed_slice()) as *mut ByteStr) }
    }
    
    /// Returns a `ByteStr` containing the entiere string.
    #[inline]
    pub fn as_byte_str(&self) -> &ByteStr {
        ByteStr::from_slice(self.as_vec().as_slice())
    }
    
    /// Returns a mutable `ByteStr` containing the entiere string.
    #[inline]
    pub fn as_mut_byte_str(&mut self) -> &mut ByteStr {
        ByteStr::from_slice_mut(self.as_mut_vec().as_mut_slice())
    }
    
    /// Returns the number of byte the string can hold without reallocating.
    #[inline]
    pub fn capacity(&self) -> usize {
        self.as_vec().capacity()
    }
    
    /// Reserves capacity for at least `additional` more bytes to be inserted into `self`.
    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        self.as_mut_vec().reserve(additional);
    }
    
    /// Reserves the minimum capacity for exactly `additional` more elements to be inserted
    /// into `self`.
    #[inline]
    pub fn reserve_exact(&mut self, additional: usize) {
        self.as_mut_vec().reserve_exact(additional);
    }
    
    /// Shrinks the capacity of the string as much as possible.
    #[inline]
    pub fn shrink_to_fit(&mut self) {
        self.as_mut_vec().shrink_to_fit();
    }
    
    /// Shortens the string, keeping the first `len` bytes.
    #[inline]
    pub fn truncate(&mut self, len: usize) {
        self.as_mut_vec().truncate(len);
    }
    
    /// Clears the vector, removing all values and keeping the capacity of the string.
    #[inline]
    pub fn clear(&mut self) {
        self.as_mut_vec().clear();
    }
    
    /// Sets the length of a vector.
    ///
    /// This will explicitly set the size of the vector, without actually modifying
    /// its buffers, so it is up to the caller to ensure that the vector is actually
    /// the specified size.
    #[inline]
    pub unsafe fn set_len(&mut self, len: usize) {
        self.as_mut_vec().set_len(len);
    }
    
    /// Removes an element from the vector and returns it.
    ///
    /// The removed element is replaced by the last element of the vector.
    ///
    /// This does not preserve ordering, but is O(1).
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of bounds.
    #[inline]
    pub fn swap_remove(&mut self, index: usize) -> u8 {
        self.as_mut_vec().swap_remove(index)
    }
    
    /// Inserts an element at position `index` within the string, shifting all elements
    /// after it to the right.
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of bounds.
    #[inline]
    pub fn insert(&mut self, index: usize, element: u8) {
        self.as_mut_vec().insert(index, element);
    }
    
    /// Removes and returns the element at position `index` within the string, shifting
    /// all elements after it to the left.
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of bounds.
    #[inline]
    pub fn remove(&mut self, index: usize) -> u8 {
        self.as_mut_vec().remove(index)
    }
    
    /// Retains only the elements specified by the predicate.
    #[inline]
    pub fn retain<F>(&mut self, mut f: F)
        where F: FnMut(u8) -> bool
    {
        self.as_mut_vec().retain(|&byte| f(byte));
    }
    
    /// Removes all but the first of consecutive elements in the string that
    /// resolve to the same key.
    ///
    /// Similar to `Vec::dedup_by_key()`.
    #[inline]
    pub fn dedup_by_key<F, K>(&mut self, key: F)
        where F: FnMut(&mut u8) -> K,
              K: PartialEq<K>
    {
        self.as_mut_vec().dedup_by_key(key);
    }
    
    /// Removes all but the first of consecutive elements in the string satisfying a given equality relation.
    ///
    /// Similar to `Vec::dedup_by()`.
    #[inline]
    pub fn dedup_by<F>(&mut self, same_bucket: F)
        where F: FnMut(&mut u8, &mut u8) -> bool
    {
        self.as_mut_vec().dedup_by(same_bucket);
    }
    
    /// Removes consecutive repeated elements in the vector.
    ///
    /// If the vector is sorted, this removes all duplicates.
    #[inline]
    pub fn dedup(&mut self) {
        self.as_mut_vec().dedup();
    }
    
    /// Appends an element to the back of the string.
    #[inline]
    pub fn push(&mut self, element: u8) {
        self.as_mut_vec().push(element);
    }
    
    /// Removes the last element from a string and returns it, or `None` if it is empty.
    #[inline]
    pub fn pop(&mut self) -> Option<u8> {
        self.as_mut_vec().pop()
    }
    
    /// Appends a slice to the back of the string.
    #[inline]
    pub fn push_slice(&mut self, other: &[u8]) {
        self.as_mut_vec().extend_from_slice(other);
    }
    
    /// Appends a `ByteStr` to the back of the string.
    #[inline]
    pub fn push_byte_str(&mut self, other: &ByteStr) {
        self.push_slice(other.as_slice());
    }
    
    /// Inserts a slice at position `index` within the string, shifting all elements
    /// after it to the right.
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of bounds.
    pub fn insert_slice(&mut self, index: usize, other: &[u8]) {
        let old_len = self.len();
        assert!(index <= old_len);
        
        self.reserve(other.len());
        
        unsafe {
            std::ptr::copy(self.as_ptr().offset(index as isize),
                           self.as_mut_ptr().offset((index + other.len()) as isize),
                           old_len - index);
            std::ptr::copy_nonoverlapping(other.as_ptr(), self.as_mut_ptr().offset(index as isize), other.len());
        }
    }
    
    /// Inserts a `ByteStr` at position `index` within the string, shifting all elements
    /// after it to the right.
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of bounds.
    #[inline]
    pub fn insert_byte_str(&mut self, index: usize, other: &ByteStr) {
        self.insert_slice(index, other.as_slice());
    }
    
    /// Creates a draining iterator that removes the specified range in the
    /// vector and yields the removed items.
    #[inline]
    pub fn drain(&mut self, start: Option<usize>, end: Option<usize>) -> std::vec::Drain<u8> {
        match (start, end) {
            (None, None) => self.as_mut_vec().drain(..),
            (Some(start), None) => self.as_mut_vec().drain(start ..),
            (None, Some(end)) => self.as_mut_vec().drain(.. end),
            (Some(start), Some(end)) => self.as_mut_vec().drain(start .. end),
        }
    }
    
    /// Splits the string into two at the given index.
    ///
    /// Returns a newly allocated `ByteString`. `self` contains elements `[0, at)`, and
    /// the returned `ByteString` contains elements `[at, len)`.
    ///
    /// # Panics
    ///
    /// Panics if `at > len`.
    #[inline]
    pub fn split_off(&mut self, at: usize) -> Self {
        Self::from_vec(self.as_mut_vec().split_off(at))
    }
}

// From
impl<'a> From<&'a [u8]> for ByteString {
    #[inline]
    fn from(src: &'a [u8]) -> Self {
        ByteStr::from_slice(src).to_byte_string()
    }
}

impl<'a> From<&'a str> for ByteString {
    #[inline]
    fn from(src: &'a str) -> Self {
        ByteString::from(src.as_bytes())
    }
}

impl From<Vec<u8>> for ByteString {
    #[inline]
    fn from(src: Vec<u8>) -> Self {
        ByteString::from_vec(src)
    }
}

impl From<Box<[u8]>> for ByteString {
    #[inline]
    fn from(src: Box<[u8]>) -> Self {
        ByteString::from_vec(Vec::from(src))
    }
}

impl From<Box<ByteStr>> for ByteString {
    #[inline]
    fn from(src: Box<ByteStr>) -> Self {
        src.into_byte_string()
    }
}

// FromIterator
impl std::iter::FromIterator<u8> for ByteString {
    #[inline]
    fn from_iter<T>(iter: T) -> Self
        where T: IntoIterator<Item=u8>
    {
        ByteString::from_vec(std::iter::FromIterator::from_iter(iter))
    }
}

// Debug
impl std::fmt::Debug for ByteString {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        std::fmt::Debug::fmt(self.as_byte_str(), f)
    }
}

// Deref
impl std::ops::Deref for ByteString {
    type Target = ByteStr;
    
    #[inline]
    fn deref(&self) -> &ByteStr {
        self.as_byte_str()
    }
}

impl std::ops::DerefMut for ByteString {
    #[inline]
    fn deref_mut(&mut self) -> &mut ByteStr {
        self.as_mut_byte_str()
    }
}

// Borrow
impl std::borrow::Borrow<ByteStr> for ByteString {
    #[inline]
    fn borrow(&self) -> &ByteStr {
        self.as_byte_str()
    }
}

impl std::borrow::BorrowMut<ByteStr> for ByteString {
    #[inline]
    fn borrow_mut(&mut self) -> &mut ByteStr {
        self.as_mut_byte_str()
    }
}

impl std::borrow::Borrow<[u8]> for ByteString {
    #[inline]
    fn borrow(&self) -> &[u8] {
        self.as_slice()
    }
}

impl std::borrow::BorrowMut<[u8]> for ByteString {
    #[inline]
    fn borrow_mut(&mut self) -> &mut [u8] {
        self.as_mut_slice()
    }
}

// AsRef
impl std::convert::AsRef<ByteStr> for ByteString {
    #[inline]
    fn as_ref(&self) -> &ByteStr {
        self.as_byte_str()
    }
}

impl std::convert::AsRef<[u8]> for ByteString {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

// AsMut
impl std::convert::AsMut<ByteStr> for ByteString {
    #[inline]
    fn as_mut(&mut self) -> &mut ByteStr {
        self.as_mut_byte_str()
    }
}

impl std::convert::AsMut<[u8]> for ByteString {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8] {
        self.as_mut_slice()
    }
}
