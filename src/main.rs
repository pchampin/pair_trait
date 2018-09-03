/// This is the SOLUTION to the problem, inspired by @E_net4's answer.
/// For the orginal problem, check the previous commit.

use std::borrow::{Borrow, Cow};

pub trait Pair<'a> {
    type Item: 'a + Borrow<str>;
    fn first(&'a self) -> &Self::Item;
    fn second(&'a self) -> &Self::Item;
}

// Exemple implementation
impl<'a, T: 'a> Pair<'a> for (T,T) where T:Borrow<str> {
    type Item = T;
    fn first(&'a self) -> &Self::Item { &self.0 }
    fn second(&'a self) -> &Self::Item { &self.1 }
}
impl<'a, T: 'a> Pair<'a> for [T;2] where T:Borrow<str> {
    type Item = T;
    fn first(&'a self) -> &Self::Item { &self[0] }
    fn second(&'a self) -> &Self::Item { &self[1] }
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

// Implementing the trait Pair for SelfSustainedPair
impl<'a> Pair<'a> for SelfSustainedPair {
    type Item = Cow<'a, str>;
    fn first(&'a self) -> &Cow<'a, str> { self.suffix().first() }
    fn second(&'a self) -> &Cow<'a, str> { self.suffix().second() }
}


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
    println!("{:?}", p1.first()); // requires the Pair trait
}

pub fn display_pairs<P, I: Iterator<Item=P>>(pairs: I) where
    for <'a> P: Pair<'a>
{
    for pair in pairs {
        println!("{:?} {:?}", pair.first().borrow(), pair.second().borrow());
    }
}


fn main() {
    test1();
    test2();

    let v = vec![ ("foo", "bar"), ("hello", "world")];
    display_pairs(v.into_iter());

    let v = vec!["ab", "cd", "ef"];
    display_pairs(v.into_iter()
        .map(|txt| SelfSustainedPair::new(
            txt.to_string(),
            |line| (
                Cow::Borrowed(&line[..1]),
                Cow::Owned(line[1..].to_ascii_uppercase().to_string()),
            )
        ))
    )
}
