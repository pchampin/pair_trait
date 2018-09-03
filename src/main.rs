/// This code is just an example demonstrating my problem with the Pait trait below.

use std::borrow::{Borrow, Cow};

pub trait Pair {
    type Item: Borrow<str>;
    fn first(&self) -> &Self::Item;
    fn second(&self) -> &Self::Item;
}

// Exemple implementation
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

// Demonstrating that the trait above works for different T's,
// with different lifetimes.
fn test1() {

    let p1: (String, String) = ("A".to_string(), "B".to_string());
    println!("{:?}", p1.first());

    let p2: (&'static str, &'static str) = ("A", "B");
    println!("{:?}", p2.first());

    let txt = "AB".to_string();
    let p3: (&str, &str) = (&txt[..1], &txt[1..]);
    println!("{:?}", p3.first());


    let txt = "AB".to_string();
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

// Attempt #0
// This is an implementation of methods similar to the trait.
// It works. Note that no lifetime needs to be speficied.
impl SelfSustainedPair {
    pub fn first_<'a,'b> (&'a self) -> &'b Cow<'a, str> { self.suffix().first() }
    pub fn second_(&self) -> &Cow<str> { self.suffix().second() }
}

/*
// Attempt #1
/// This is a simiar attempt to implement the Trait.
/// It does not compile.
impl Pair for SelfSustainedPair {
    type Item = Cow<str>;
    fn first(&self) -> &Cow<str> { self.suffix().first() }
    fn second(&self) -> &Cow<str> { self.suffix().second() }
}
*/

/*
// Attempt #2
impl<'a> Pair for SelfSustainedPair {
    type Item = Cow<'a, str>;
    fn first(&self) -> &Cow<'a, str> { self.suffix().first() }
    fn second(&self) -> &Cow<'a, str> { self.suffix().second() }
}
*/

/*
// Attempt #3
// NB: this requires a modification of the Pair trait above
impl<'a> Pair<'a> for SelfSustainedPair {
    type Item = Cow<'a, str>;
    fn first(&self) -> &Cow<'a, str> { self.suffix().first() }
    fn second(&self) -> &Cow<'a, str> { self.suffix().second() }
}
*/

/*
// Attempt #4
impl Pair for SelfSustainedPair {
    type Item = Cow<'self, str>;
    fn first(&self) -> &Cow<str> { self.suffix().first() }
    fn second(&self) -> &Cow<str> { self.suffix().second() }
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
    println!("{:?}", p1.first_());
    //println!("{:?}", p1.first()); // requires the Pair trait
}

pub fn display_pairs<P: Pair, I: Iterator<Item=P>>(pairs: I) {
    for pair in pairs {
        println!("{:?} {:?}", pair.first().borrow(), pair.second().borrow());
    }
}


fn main() {
    test1();
    test2();
}
