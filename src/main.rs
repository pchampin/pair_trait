/// This code is just an example demonstrating my problem with the Pait trait below.

use std::borrow::{Borrow, Cow};

pub trait Pair {
    type Item: Borrow<str>;
    fn first(&self) -> &Self::Item;
    fn second(&self) -> &Self::Item;
}

impl<T> Pair for (T,T) where T:Borrow<str> {
    type Item = T;
    fn first(&self) -> &Self::Item { &self.0 }
    fn second(&self) -> &Self::Item { &self.1 }
}

impl<T> Pair for [T;2] where T:Borrow<str> {
    type Item = T;
    fn first(&self) -> &Self::Item { &self[0] }
    fn second(&self) -> &Self::Item { &self[1] }
}

fn test1() {
    // demonstrating that the trait above works for different T's,
    // with different lifetimes.

    let p1: (String, String) = ("A".to_string(), "B".to_string());
    println!("{:?}", p1.first());

    let p2: (&'static str, &'static str) = ("A", "B");
    println!("{:?}", p2.first());

    let p3: (&str, &str);
    let txt = "AB";
    p3  = (&txt[..1], &txt[1..]);
    println!("{:?}", p3.first());

    let p4: (Cow<str>, Cow<str>) = (
        Cow::Borrowed(&txt[..1]),
        Cow::Owned("B".to_string()),
    );
    println!("{:?}", p4.first());
}

// Now I need an implementation of Pair that contains Cow<str>'s,
// but also owns the data borrowed (if any) by those Cow<str>'s.
#[macro_use] extern crate rental;
rental! {
    pub mod self_sustained_pair {
        use std::borrow::Cow;
        #[rental(covariant)]
        pub struct SelfSustainedPair {
            line: String,
            pair: (Cow<'line, str>, Cow<'line, str>),
        }
    }
}
pub use self::self_sustained_pair::*;

/// trait implementation -- does not compile
/*
impl<'a> Pair for SelfSustainedPair {
    type Item = Cow<'_, str>;
    fn first(&self) -> &Self::Item { self.suffix().first() }
    fn second(&self) -> &Self::Item { self.suffix().second() }
}
*/

fn test2() {
    let p1: SelfSustainedPair;
    let line = "Ab".to_string();
    p1 = SelfSustainedPair::new(
        line,
        |line| (
            Cow::Borrowed(&line[..1]),
            Cow::Owned(line[1..].to_ascii_uppercase().to_string()),
        )
    );
    println!("{:?}", p1.suffix().first());
    //println!("{:?}", p1.first()); // requires the Pair trait
}

fn main() {
    test1();
    test2();
}
