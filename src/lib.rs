#![feature(core)]
#![feature(io)]
#![feature(plugin)]
#![plugin(regex_macros)]

use std::io::{Error, ErrorKind};
use std::option::Option;
use std::str::FromStr;


extern crate regex;
use regex::Regex;

static RE: Regex = regex!(r"repr: Os\((\d+)\)");

fn get_errno(e: &Error) -> Option<i32> {
    let s = format!("{:?}", e);
    match RE.captures(&s) {
        None => None,
        Some(captures) => {
            match captures.at(1) {
                None => None,
                Some(text) => Some(FromStr::from_str(text).unwrap())
            }
        }
    }
}

#[test]
fn returns_errno_when_present() {
    assert_eq!(get_errno(&Error::from_os_error(123)), Some(123));
}

#[test]
fn returns_nothing_when_no_errno() {
    assert_eq!(get_errno(&Error::new(ErrorKind::Other, "xxx", None)), None);
}

#[test]
fn returns_nothing_when_no_errno_even_though_it_looks_like_errno1() {
    assert_eq!(get_errno(&Error::new(ErrorKind::Other, "Os(321)", None)), None);
}

#[test]
fn returns_nothing_when_no_errno_even_though_it_looks_like_errno2() {
    assert_eq!(
        get_errno(&Error::new(ErrorKind::Other, "xxx", Some("Os(45)".to_string()))),
        None);
}
