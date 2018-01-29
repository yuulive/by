// Copyright 2018 Eduardo Sánchez Muñoz
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use core::iter::FromIterator;
use ByteStr;
#[cfg(not(feature="no_std"))]
use ByteString;

#[test]
fn find_test_1() {
    let string = ByteStr::from_slice(b"0123456789 0123456789");
    assert_eq!(string.find(b"345"), Some(3));
    assert_eq!(string.rfind(b"345"), Some(17));
}

#[test]
fn find_test_2() {
    let string = ByteStr::from_slice(b"0123456789 0123456789");
    assert_eq!(string.find(b'5'), Some(5));
    assert_eq!(string.rfind(b'5'), Some(17));
}

#[cfg(not(feature="no_std"))]
#[test]
fn split_test_1() {
    let mut string = ByteString::from_slice(b"aaaabaaaabaaaa");
    let expected = [
        ByteStr::empty(),
        ByteStr::from_slice(b"ab"),
        ByteStr::from_slice(b"ab"),
        ByteStr::from_slice(b"a"),
    ];
    
    assert_eq!(Vec::from_iter(string.split(b"aaa")), expected);
    assert_eq!(Vec::from_iter(string.split_mut(b"aaa")), expected);
}

#[cfg(not(feature="no_std"))]
#[test]
fn split_test_2() {
    let mut string = ByteString::from_slice(b"aaaabaaaabaaaa");
    let expected = [
        ByteStr::empty(),
        ByteStr::from_slice(b"ba"),
        ByteStr::from_slice(b"ba"),
        ByteStr::from_slice(b"a"),
    ];
    
    assert_eq!(Vec::from_iter(string.rsplit(b"aaa")), expected);
    assert_eq!(Vec::from_iter(string.rsplit_mut(b"aaa")), expected);
}

#[cfg(not(feature="no_std"))]
#[test]
fn split_test_3() {
    let mut string = ByteString::from_slice(b"aaaaXbbbbXcccc");
    let expected = [
        ByteStr::from_slice(b"aaaa"),
        ByteStr::from_slice(b"bbbb"),
        ByteStr::from_slice(b"cccc"),
    ];
    
    assert_eq!(Vec::from_iter(string.split(b'X')), expected);
    assert_eq!(Vec::from_iter(string.split_mut(b'X')), expected);
    assert_eq!(Vec::from_iter(string.rsplit(b'X').rev()), expected);
    assert_eq!(Vec::from_iter(string.rsplit_mut(b'X').rev()), expected);
}

#[test]
#[cfg(not(feature="no_std"))]
fn split_test_4() {
    let mut string = ByteString::from_slice(b"aaaaXbbbbXcccc");
    let expected = [
        ByteStr::from_slice(b"cccc"),
        ByteStr::from_slice(b"bbbb"),
        ByteStr::from_slice(b"aaaa"),
    ];
    
    assert_eq!(Vec::from_iter(string.rsplit(b'X')), expected);
    assert_eq!(Vec::from_iter(string.rsplit_mut(b'X')), expected);
    assert_eq!(Vec::from_iter(string.split(b'X').rev()), expected);
    assert_eq!(Vec::from_iter(string.split_mut(b'X').rev()), expected);
}

#[cfg(not(feature="no_std"))]
#[test]
fn split_test_5() {
    let mut string = ByteString::from_slice(b"aaaaXbbbbXcccc");
    let expected = [
        ByteStr::from_slice(b"aaaa"),
        ByteStr::from_slice(b"bbbbXcccc"),
    ];
    
    assert_eq!(Vec::from_iter(string.splitn(2, b'X')), expected);
    assert_eq!(Vec::from_iter(string.splitn_mut(2, b'X')), expected);
}

#[cfg(not(feature="no_std"))]
#[test]
fn split_test_6() {
    let mut string = ByteString::from_slice(b"aaaaXbbbbXcccc");
    let expected = [
        ByteStr::from_slice(b"cccc"),
        ByteStr::from_slice(b"aaaaXbbbb"),
    ];
    
    assert_eq!(Vec::from_iter(string.rsplitn(2, b'X')), expected);
    assert_eq!(Vec::from_iter(string.rsplitn_mut(2, b'X')), expected);
}

#[cfg(not(feature="no_std"))]
#[test]
fn match_test_1() {
    let mut string = ByteString::from_slice(b"XaaaaXaaaaX");
    let expected = [
        (1, ByteStr::from_slice(b"aaa")),
        (6, ByteStr::from_slice(b"aaa")),
    ];
    
    assert_eq!(Vec::from_iter(string.match_indices(b"aaa")), expected);
    assert_eq!(Vec::from_iter(string.match_indices_mut(b"aaa").map(|(i, s)| (i, s as _))), expected);
}

#[cfg(not(feature="no_std"))]
#[test]
fn match_test_2() {
    let mut string = ByteString::from_slice(b"XaaaaXaaaaX");
    let expected = [
        (7, ByteStr::from_slice(b"aaa")),
        (2, ByteStr::from_slice(b"aaa")),
    ];
    
    assert_eq!(Vec::from_iter(string.rmatch_indices(b"aaa")), expected);
    assert_eq!(Vec::from_iter(string.rmatch_indices_mut(b"aaa").map(|(i, s)| (i, s as _))), expected);
}

#[cfg(not(feature="no_std"))]
#[test]
fn match_test_3() {
    let mut string = ByteString::from_slice(b"aaaaXaaaaXaaaa");
    let expected = [
        (4, ByteStr::from_slice(b"X")),
        (9, ByteStr::from_slice(b"X")),
    ];
    
    assert_eq!(Vec::from_iter(string.match_indices(b'X')), expected);
    assert_eq!(Vec::from_iter(string.match_indices_mut(b'X').map(|(i, s)| (i, s as _))), expected);
    assert_eq!(Vec::from_iter(string.rmatch_indices(b'X').rev()), expected);
    assert_eq!(Vec::from_iter(string.rmatch_indices_mut(b'X').rev().map(|(i, s)| (i, s as _))), expected);
}

#[cfg(not(feature="no_std"))]
#[test]
fn match_test_4() {
    let mut string = ByteString::from_slice(b"aaaaXaaaaXaaaa");
    let expected = [
        (9, ByteStr::from_slice(b"X")),
        (4, ByteStr::from_slice(b"X")),
    ];
    
    assert_eq!(Vec::from_iter(string.rmatch_indices(b'X')), expected);
    assert_eq!(Vec::from_iter(string.rmatch_indices_mut(b'X').map(|(i, s)| (i, s as _))), expected);
    assert_eq!(Vec::from_iter(string.match_indices(b'X').rev()), expected);
    assert_eq!(Vec::from_iter(string.match_indices_mut(b'X').rev().map(|(i, s)| (i, s as _))), expected);
}
