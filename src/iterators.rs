// Copyright 2018 Eduardo Sánchez Muñoz
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms. 

use core;
use ByteStr;
use Matcher;
use ForwardSearcher;
use ReverseSearcher;
use DoubleEndedSearcher;

// Windows
#[derive(Clone)]
pub struct Windows<'a> {
    inner: core::slice::Windows<'a, u8>
}

impl<'a> Windows<'a> {
    #[inline]
    pub fn new(string: &'a ByteStr, size: usize) -> Self {
        Self { inner: string.as_slice().windows(size) }
    }
}

impl<'a> core::fmt::Debug for Windows<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        core::fmt::Debug::fmt(&self.inner, f)
    }
}

impl<'a> Iterator for Windows<'a> {
    type Item = &'a ByteStr;
    
    #[inline]
    fn next(&mut self) -> Option<&'a ByteStr> {
        self.inner.next().map(|s| ByteStr::from_slice(s))
    }
    
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
    
    #[inline]
    fn count(self) -> usize {
        self.inner.count()
    }
    
    #[inline]
    fn nth(&mut self, n: usize) -> Option<&'a ByteStr> {
        self.inner.nth(n).map(|s| ByteStr::from_slice(s))
    }
    
    #[inline]
    fn last(self) -> Option<&'a ByteStr> {
        self.inner.last().map(|s| ByteStr::from_slice(s))
    }
}

impl<'a> DoubleEndedIterator for Windows<'a> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a ByteStr> {
        self.inner.next_back().map(|s| ByteStr::from_slice(s))
    }
}

impl<'a> ExactSizeIterator for Windows<'a> {
    #[inline]
    fn len(&self) -> usize {
        self.inner.len()
    }
}

//impl<'a> FusedIterator for Windows<'a> {}

// Chunks
#[derive(Clone)]
pub struct Chunks<'a> {
    inner: core::slice::Chunks<'a, u8>
}

impl<'a> Chunks<'a> {
    #[inline]
    pub fn new(string: &'a ByteStr, size: usize) -> Self {
        Self { inner: string.as_slice().chunks(size) }
    }
}

impl<'a> core::fmt::Debug for Chunks<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        core::fmt::Debug::fmt(&self.inner, f)
    }
}

impl<'a> Iterator for Chunks<'a> {
    type Item = &'a ByteStr;
    
    #[inline]
    fn next(&mut self) -> Option<&'a ByteStr> {
        self.inner.next().map(|s| ByteStr::from_slice(s))
    }
    
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
    
    #[inline]
    fn count(self) -> usize {
        self.inner.count()
    }
    
    #[inline]
    fn nth(&mut self, n: usize) -> Option<&'a ByteStr> {
        self.inner.nth(n).map(|s| ByteStr::from_slice(s))
    }
    
    #[inline]
    fn last(self) -> Option<&'a ByteStr> {
        self.inner.last().map(|s| ByteStr::from_slice(s))
    }
}

impl<'a> DoubleEndedIterator for Chunks<'a> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a ByteStr> {
        self.inner.next_back().map(|s| ByteStr::from_slice(s))
    }
}

impl<'a> ExactSizeIterator for Chunks<'a> {
    #[inline]
    fn len(&self) -> usize {
        self.inner.len()
    }
}

//impl<'a> FusedIterator for Chunks<'a> {}

// Chunks
pub struct ChunksMut<'a> {
    inner: core::slice::ChunksMut<'a, u8>
}

impl<'a> ChunksMut<'a> {
    #[inline]
    pub fn new(string: &'a mut ByteStr, size: usize) -> Self {
        Self { inner: string.as_mut_slice().chunks_mut(size) }
    }
}

impl<'a> core::fmt::Debug for ChunksMut<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        core::fmt::Debug::fmt(&self.inner, f)
    }
}

impl<'a> Iterator for ChunksMut<'a> {
    type Item = &'a ByteStr;
    
    #[inline]
    fn next(&mut self) -> Option<&'a ByteStr> {
        self.inner.next().map(|s| ByteStr::from_slice(s))
    }
    
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
    
    #[inline]
    fn count(self) -> usize {
        self.inner.count()
    }
    
    #[inline]
    fn nth(&mut self, n: usize) -> Option<&'a ByteStr> {
        self.inner.nth(n).map(|s| ByteStr::from_slice(s))
    }
    
    #[inline]
    fn last(self) -> Option<&'a ByteStr> {
        self.inner.last().map(|s| ByteStr::from_slice(s))
    }
}

impl<'a> DoubleEndedIterator for ChunksMut<'a> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a ByteStr> {
        self.inner.next_back().map(|s| ByteStr::from_slice(s))
    }
}

impl<'a> ExactSizeIterator for ChunksMut<'a> {
    #[inline]
    fn len(&self) -> usize {
        self.inner.len()
    }
}

//impl<'a> FusedIterator for ChunksMut<'a> {}

// SplitBase
#[derive(Clone, Debug)]
struct SplitBase<'a, M: Matcher> {
    string: &'a ByteStr,
    matcher: M,
    finished: bool,
}

impl<'a, M: Matcher> SplitBase<'a, M> {
    #[inline]
    fn new(string: &'a ByteStr, matcher: M) -> Self {
        Self {
            string: string,
            matcher: matcher,
            finished: false,
        }
    }
    
    #[inline]
    fn remaining(&mut self) -> &'a ByteStr {
        core::mem::replace(&mut self.string, ByteStr::empty())
    }
}

impl<'a, M: ForwardSearcher> SplitBase<'a, M> {
    #[inline]
    pub fn next_forwards(&mut self) -> Option<&'a ByteStr> {
        if !self.finished {
            match self.matcher.find(self.string) {
                Some((a, b)) => {
                    let (ret, rest) = self.string.split_at(b);
                    self.string = rest;
                    Some(&ret[.. a])
                }
                None => {
                    let ret = core::mem::replace(&mut self.string, ByteStr::empty());
                    self.finished = true;
                    Some(ret)
                }
            }
        } else {
            None
        }
    }
}

impl<'a, M: ReverseSearcher> SplitBase<'a, M> {
    #[inline]
    pub fn next_backwards(&mut self) -> Option<&'a ByteStr> {
        if !self.finished {
            match self.matcher.rfind(self.string) {
                Some((a, b)) => {
                    let (rest, ret) = self.string.split_at(b);
                    self.string = &rest[.. a];
                    Some(ret)
                }
                None => {
                    let ret = core::mem::replace(&mut self.string, ByteStr::empty());
                    self.finished = true;
                    Some(ret)
                }
            }
        } else {
            None
        }
    }
}

// SplitBaseMut
#[derive(Debug)]
struct SplitBaseMut<'a, M: Matcher> {
    string: &'a mut ByteStr,
    matcher: M,
    finished: bool,
}

impl<'a, M: Matcher> SplitBaseMut<'a, M> {
    #[inline]
    fn new(string: &'a mut ByteStr, matcher: M) -> Self {
        Self {
            string: string,
            matcher: matcher,
            finished: false,
        }
    }
    
    #[inline]
    fn remaining(&mut self) -> &'a mut ByteStr {
        core::mem::replace(&mut self.string, ByteStr::empty_mut())
    }
}

impl<'a, M: ForwardSearcher> SplitBaseMut<'a, M> {
    fn next_forwards(&mut self) -> Option<&'a mut ByteStr> {
        if !self.finished {
            match self.matcher.find(self.string) {
                Some((a, b)) => {
                    let string = core::mem::replace(&mut self.string, ByteStr::empty_mut());
                    let (ret, rest) = string.split_at_mut(b);
                    self.string = rest;
                    Some(&mut ret[.. a])
                }
                None => {
                    let ret = core::mem::replace(&mut self.string, ByteStr::empty_mut());
                    self.finished = true;
                    Some(ret)
                }
            }
        } else {
            None
        }
    }
}

impl<'a, M: ReverseSearcher> SplitBaseMut<'a, M> {
    fn next_backwards(&mut self) -> Option<&'a mut ByteStr> {
        if !self.finished {
            match self.matcher.rfind(self.string) {
                Some((a, b)) => {
                    let string = core::mem::replace(&mut self.string, ByteStr::empty_mut());
                    let (rest, ret) = string.split_at_mut(b);
                    self.string = &mut rest[.. a];
                    Some(ret)
                }
                None => {
                    let ret = core::mem::replace(&mut self.string, ByteStr::empty_mut());
                    self.finished = true;
                    Some(ret)
                }
            }
        } else {
            None
        }
    }
}


// Split
#[derive(Clone, Debug)]
pub struct Split<'a, M: ForwardSearcher> {
    base: SplitBase<'a, M>,
}

impl<'a, M: ForwardSearcher> Split<'a, M> {
    #[inline]
    pub fn new(string: &'a ByteStr, matcher: M) -> Self {
        Self { base: SplitBase::new(string, matcher) }
    }
}

impl<'a, M: ForwardSearcher> Iterator for Split<'a, M> {
    type Item = &'a ByteStr;
    
    #[inline]
    fn next(&mut self) -> Option<&'a ByteStr> {
        self.base.next_forwards()
    }
}

impl<'a, M: DoubleEndedSearcher> DoubleEndedIterator for Split<'a, M> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a ByteStr> {
        self.base.next_backwards()
    }
}

// SplitMut
#[derive(Debug)]
pub struct SplitMut<'a, M: ForwardSearcher> {
    base: SplitBaseMut<'a, M>,
}

impl<'a, M: ForwardSearcher> SplitMut<'a, M> {
    #[inline]
    pub fn new(string: &'a mut ByteStr, matcher: M) -> Self {
        Self { base: SplitBaseMut::new(string, matcher) }
    }
}

impl<'a, M: ForwardSearcher> Iterator for SplitMut<'a, M> {
    type Item = &'a mut ByteStr;
    
    #[inline]
    fn next(&mut self) -> Option<&'a mut ByteStr> {
        self.base.next_forwards()
    }
}

impl<'a, M: DoubleEndedSearcher> DoubleEndedIterator for SplitMut<'a, M> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a mut ByteStr> {
        self.base.next_backwards()
    }
}

// RSplit
#[derive(Clone, Debug)]
pub struct RSplit<'a, M: ReverseSearcher> {
    base: SplitBase<'a, M>,
}

impl<'a, M: ReverseSearcher> RSplit<'a, M> {
    #[inline]
    pub fn new(string: &'a ByteStr, matcher: M) -> Self {
        Self { base: SplitBase::new(string, matcher) }
    }
}

impl<'a, M: ReverseSearcher> Iterator for RSplit<'a, M> {
    type Item = &'a ByteStr;
    
    #[inline]
    fn next(&mut self) -> Option<&'a ByteStr> {
        self.base.next_backwards()
    }
}

impl<'a, M: DoubleEndedSearcher> DoubleEndedIterator for RSplit<'a, M> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a ByteStr> {
        self.base.next_forwards()
    }
}

// RSplitMut
#[derive(Debug)]
pub struct RSplitMut<'a, M: ReverseSearcher> {
    base: SplitBaseMut<'a, M>,
}

impl<'a, M: ReverseSearcher> RSplitMut<'a, M> {
    #[inline]
    pub fn new(string: &'a mut ByteStr, matcher: M) -> Self {
        Self { base: SplitBaseMut::new(string, matcher) }
    }
}

impl<'a, M: ReverseSearcher> Iterator for RSplitMut<'a, M> {
    type Item = &'a mut ByteStr;
    
    #[inline]
    fn next(&mut self) -> Option<&'a mut ByteStr> {
        self.base.next_backwards()
    }
}

impl<'a, M: DoubleEndedSearcher> DoubleEndedIterator for RSplitMut<'a, M> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a mut ByteStr> {
        self.base.next_forwards()
    }
}

// SplitN
#[derive(Clone, Debug)]
pub struct SplitN<'a, M: ForwardSearcher> {
    base: SplitBase<'a, M>,
    remaining: usize,
}

impl<'a, M: ForwardSearcher> SplitN<'a, M> {
    #[inline]
    pub fn new(string: &'a ByteStr, n: usize, matcher: M) -> Self {
        Self {
            base: SplitBase::new(string, matcher),
            remaining: n,
        }
    }
}

impl<'a, M: ForwardSearcher> Iterator for SplitN<'a, M> {
    type Item = &'a ByteStr;
    
    #[inline]
    fn next(&mut self) -> Option<&'a ByteStr> {
        if self.remaining == 0 {
            None
        } else if self.remaining == 1 {
            self.remaining = 0;
            Some(self.base.remaining())
        } else {
            self.remaining -= 1;
            self.base.next_forwards()
        }
    }
}

// SplitNMut
#[derive(Debug)]
pub struct SplitNMut<'a, M: ForwardSearcher> {
    base: SplitBaseMut<'a, M>,
    remaining: usize,
}

impl<'a, M: ForwardSearcher> SplitNMut<'a, M> {
    #[inline]
    pub fn new(string: &'a mut ByteStr, n: usize, matcher: M) -> Self {
        Self {
            base: SplitBaseMut::new(string, matcher),
            remaining: n,
        }
    }
}

impl<'a, M: ForwardSearcher> Iterator for SplitNMut<'a, M> {
    type Item = &'a mut ByteStr;
    
    #[inline]
    fn next(&mut self) -> Option<&'a mut ByteStr> {
        if self.remaining == 0 {
            None
        } else if self.remaining == 1 {
            self.remaining = 0;
            Some(self.base.remaining())
        } else {
            self.remaining -= 1;
            self.base.next_forwards()
        }
    }
}

// RSplitN
#[derive(Clone, Debug)]
pub struct RSplitN<'a, M: ReverseSearcher> {
    base: SplitBase<'a, M>,
    remaining: usize,
}

impl<'a, M: ReverseSearcher> RSplitN<'a, M> {
    #[inline]
    pub fn new(string: &'a ByteStr, n: usize, matcher: M) -> Self {
        Self {
            base: SplitBase::new(string, matcher),
            remaining: n,
        }
    }
}

impl<'a, M: ReverseSearcher> Iterator for RSplitN<'a, M> {
    type Item = &'a ByteStr;
    
    #[inline]
    fn next(&mut self) -> Option<&'a ByteStr> {
        if self.remaining == 0 {
            None
        } else if self.remaining == 1 {
            self.remaining = 0;
            Some(self.base.remaining())
        } else {
            self.remaining -= 1;
            self.base.next_backwards()
        }
    }
}

// RSplitNMut
#[derive(Debug)]
pub struct RSplitNMut<'a, M: ReverseSearcher> {
    base: SplitBaseMut<'a, M>,
    remaining: usize,
}

impl<'a, M: ReverseSearcher> RSplitNMut<'a, M> {
    #[inline]
    pub fn new(string: &'a mut ByteStr, n: usize, matcher: M) -> Self {
        Self {
            base: SplitBaseMut::new(string, matcher),
            remaining: n,
        }
    }
}

impl<'a, M: ReverseSearcher> Iterator for RSplitNMut<'a, M> {
    type Item = &'a mut ByteStr;
    
    #[inline]
    fn next(&mut self) -> Option<&'a mut ByteStr> {
        if self.remaining == 0 {
            None
        } else if self.remaining == 1 {
            self.remaining = 0;
            Some(self.base.remaining())
        } else {
            self.remaining -= 1;
            self.base.next_backwards()
        }
    }
}

// MatchesBase
#[derive(Clone, Debug)]
struct MatchesBase<'a, M: Matcher> {
    string: &'a ByteStr,
    matcher: M,
    pos: usize,
}

impl<'a, M: Matcher> MatchesBase<'a, M> {
    #[inline]
    fn new(string: &'a ByteStr, matcher: M) -> Self {
        Self {
            string: string,
            matcher: matcher,
            pos: 0,
        }
    }
}

impl<'a, M: ForwardSearcher> MatchesBase<'a, M> {
    fn next_forwards(&mut self) -> Option<(usize, &'a ByteStr)> {
        match self.matcher.find(self.string) {
            Some((a, b)) => {
                let (ret, rest) = self.string.split_at(b);
                let ret_pos = self.pos + a;
                self.string = rest;
                self.pos += b;
                Some((ret_pos, &ret[a ..]))
            }
            None => {
                self.string = ByteStr::empty();
                None
            }
        }
    }
}

impl<'a, M: ReverseSearcher> MatchesBase<'a, M> {
    fn next_backwards(&mut self) -> Option<(usize, &'a ByteStr)> {
        match self.matcher.rfind(self.string) {
            Some((a, b)) => {
                let (rest, ret) = self.string[.. b].split_at(a);
                self.string = rest;
                Some((a, ret))
            }
            None => {
                self.string = ByteStr::empty();
                None
            }
        }
    }
}

// MatchesBaseMut
#[derive(Debug)]
struct MatchesBaseMut<'a, M: Matcher> {
    string: &'a mut ByteStr,
    matcher: M,
    pos: usize,
}

impl<'a, M: Matcher> MatchesBaseMut<'a, M> {
    #[inline]
    fn new(string: &'a mut ByteStr, matcher: M) -> Self {
        Self {
            string: string,
            matcher: matcher,
            pos: 0,
        }
    }
}

impl<'a, M: ForwardSearcher> MatchesBaseMut<'a, M> {
    fn next_forwards(&mut self) -> Option<(usize, &'a mut ByteStr)> {
        match self.matcher.find(self.string) {
            Some((a, b)) => {
                let string = core::mem::replace(&mut self.string, ByteStr::empty_mut());
                let (ret, rest) = string.split_at_mut(b);
                let ret_pos = self.pos + a;
                self.string = rest;
                self.pos += b;
                Some((ret_pos, &mut ret[a ..]))
            }
            None => {
                self.string = ByteStr::empty_mut();
                None
            }
        }
    }
}

impl<'a, M: ReverseSearcher> MatchesBaseMut<'a, M> {
    fn next_backwards(&mut self) -> Option<(usize, &'a mut ByteStr)> {
        match self.matcher.rfind(self.string) {
            Some((a, b)) => {
                let string = core::mem::replace(&mut self.string, ByteStr::empty_mut());
                let (rest, ret) = string[.. b].split_at_mut(a);
                self.string = rest;
                Some((a, ret))
            }
            None => {
                self.string = ByteStr::empty_mut();
                None
            }
        }
    }
}

// Matches
#[derive(Clone, Debug)]
pub struct Matches<'a, M: ForwardSearcher> {
    base: MatchesBase<'a, M>,
}

impl<'a, M: ForwardSearcher> Matches<'a, M> {
    #[inline]
    pub fn new(string: &'a ByteStr, matcher: M) -> Self {
        Self { base: MatchesBase::new(string, matcher) }
    }
}

impl<'a, M: ForwardSearcher> Iterator for Matches<'a, M> {
    type Item = &'a ByteStr;
    
    #[inline]
    fn next(&mut self) -> Option<&'a ByteStr> {
        self.base.next_forwards().map(|(_, s)| s)
    }
}

impl<'a, M: DoubleEndedSearcher> DoubleEndedIterator for Matches<'a, M> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a ByteStr> {
        self.base.next_backwards().map(|(_, s)| s)
    }
}

// MatchesMut
#[derive(Debug)]
pub struct MatchesMut<'a, M: ForwardSearcher> {
    base: MatchesBaseMut<'a, M>,
}

impl<'a, M: ForwardSearcher> MatchesMut<'a, M> {
    #[inline]
    pub fn new(string: &'a mut ByteStr, matcher: M) -> Self {
        Self { base: MatchesBaseMut::new(string, matcher) }
    }
}

impl<'a, M: ForwardSearcher> Iterator for MatchesMut<'a, M> {
    type Item = &'a mut ByteStr;
    
    #[inline]
    fn next(&mut self) -> Option<&'a mut ByteStr> {
        self.base.next_forwards().map(|(_, s)| s)
    }
}

impl<'a, M: DoubleEndedSearcher> DoubleEndedIterator for MatchesMut<'a, M> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a mut ByteStr> {
        self.base.next_backwards().map(|(_, s)| s)
    }
}

// RMatches
#[derive(Clone, Debug)]
pub struct RMatches<'a, M: ReverseSearcher> {
    base: MatchesBase<'a, M>,
}

impl<'a, M: ReverseSearcher> RMatches<'a, M> {
    #[inline]
    pub fn new(string: &'a ByteStr, matcher: M) -> Self {
        Self { base: MatchesBase::new(string, matcher) }
    }
}

impl<'a, M: ReverseSearcher> Iterator for RMatches<'a, M> {
    type Item = &'a ByteStr;
    
    #[inline]
    fn next(&mut self) -> Option<&'a ByteStr> {
        self.base.next_backwards().map(|(_, s)| s)
    }
}

impl<'a, M: DoubleEndedSearcher> DoubleEndedIterator for RMatches<'a, M> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a ByteStr> {
        self.base.next_forwards().map(|(_, s)| s)
    }
}

// RMatchesMut
#[derive(Debug)]
pub struct RMatchesMut<'a, M: ReverseSearcher> {
    base: MatchesBaseMut<'a, M>,
}

impl<'a, M: ReverseSearcher> RMatchesMut<'a, M> {
    #[inline]
    pub fn new(string: &'a mut ByteStr, matcher: M) -> Self {
        Self { base: MatchesBaseMut::new(string, matcher) }
    }
}

impl<'a, M: ReverseSearcher> Iterator for RMatchesMut<'a, M> {
    type Item = &'a mut ByteStr;
    
    #[inline]
    fn next(&mut self) -> Option<&'a mut ByteStr> {
        self.base.next_backwards().map(|(_, s)| s)
    }
}

impl<'a, M: DoubleEndedSearcher> DoubleEndedIterator for RMatchesMut<'a, M> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a mut ByteStr> {
        self.base.next_forwards().map(|(_, s)| s)
    }
}

// MatchIndices
#[derive(Clone, Debug)]
pub struct MatchIndices<'a, M: ForwardSearcher> {
    base: MatchesBase<'a, M>,
}

impl<'a, M: ForwardSearcher> MatchIndices<'a, M> {
    #[inline]
    pub fn new(string: &'a ByteStr, matcher: M) -> Self {
        Self { base: MatchesBase::new(string, matcher) }
    }
}

impl<'a, M: ForwardSearcher> Iterator for MatchIndices<'a, M> {
    type Item = (usize, &'a ByteStr);
    
    #[inline]
    fn next(&mut self) -> Option<(usize, &'a ByteStr)> {
        self.base.next_forwards()
    }
}

impl<'a, M: DoubleEndedSearcher> DoubleEndedIterator for MatchIndices<'a, M> {
    #[inline]
    fn next_back(&mut self) -> Option<(usize, &'a ByteStr)> {
        self.base.next_backwards()
    }
}

// MatchIndicesMut
#[derive(Debug)]
pub struct MatchIndicesMut<'a, M: ForwardSearcher> {
    base: MatchesBaseMut<'a, M>,
}

impl<'a, M: ForwardSearcher> MatchIndicesMut<'a, M> {
    #[inline]
    pub fn new(string: &'a mut ByteStr, matcher: M) -> Self {
        Self { base: MatchesBaseMut::new(string, matcher) }
    }
}

impl<'a, M: ForwardSearcher> Iterator for MatchIndicesMut<'a, M> {
    type Item = (usize, &'a mut ByteStr);
    
    #[inline]
    fn next(&mut self) -> Option<(usize, &'a mut ByteStr)> {
        self.base.next_forwards()
    }
}

impl<'a, M: DoubleEndedSearcher> DoubleEndedIterator for MatchIndicesMut<'a, M> {
    #[inline]
    fn next_back(&mut self) -> Option<(usize, &'a mut ByteStr)> {
        self.base.next_backwards()
    }
}

// RMatchIndices
#[derive(Clone, Debug)]
pub struct RMatchIndices<'a, M: ReverseSearcher> {
    base: MatchesBase<'a, M>,
}

impl<'a, M: ReverseSearcher> RMatchIndices<'a, M> {
    #[inline]
    pub fn new(string: &'a ByteStr, matcher: M) -> Self {
        Self { base: MatchesBase::new(string, matcher) }
    }
}

impl<'a, M: ReverseSearcher> Iterator for RMatchIndices<'a, M> {
    type Item = (usize, &'a ByteStr);
    
    #[inline]
    fn next(&mut self) -> Option<(usize, &'a ByteStr)> {
        self.base.next_backwards()
    }
}

impl<'a, M: DoubleEndedSearcher> DoubleEndedIterator for RMatchIndices<'a, M> {
    #[inline]
    fn next_back(&mut self) -> Option<(usize, &'a ByteStr)> {
        self.base.next_forwards()
    }
}

// RMatchIndicesMut
#[derive(Debug)]
pub struct RMatchIndicesMut<'a, M: ReverseSearcher> {
    base: MatchesBaseMut<'a, M>,
}

impl<'a, M: ReverseSearcher> RMatchIndicesMut<'a, M> {
    #[inline]
    pub fn new(string: &'a mut ByteStr, matcher: M) -> Self {
        Self { base: MatchesBaseMut::new(string, matcher) }
    }
}

impl<'a, M: ReverseSearcher> Iterator for RMatchIndicesMut<'a, M> {
    type Item = (usize, &'a mut ByteStr);
    
    #[inline]
    fn next(&mut self) -> Option<(usize, &'a mut ByteStr)> {
        self.base.next_backwards()
    }
}

impl<'a, M: DoubleEndedSearcher> DoubleEndedIterator for RMatchIndicesMut<'a, M> {
    #[inline]
    fn next_back(&mut self) -> Option<(usize, &'a mut ByteStr)> {
        self.base.next_forwards()
    }
}
