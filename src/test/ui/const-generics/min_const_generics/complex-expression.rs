#![feature(min_const_generics)]

fn test<const N: usize>() {}

fn ok<const M: usize>() -> [u8; M] {
    [0; { M }]
}

struct Break0<const N: usize>([u8; { N + 1 }]);
//~^ ERROR generic parameters must not be used inside const evaluations

struct Break1<const N: usize>([u8; { { N } }]);
//~^ ERROR generic parameters must not be used inside const evaluations

fn break2<const N: usize>() {
    let _: [u8; N + 1];
    //~^ ERROR generic parameters must not be used inside const evaluations
}

fn break3<const N: usize>() {
    let _ = [0; N + 1];
    //~^ ERROR generic parameters must not be used inside const evaluations
}

trait Foo {
    const ASSOC: usize;
}

fn main() {}
