// compile-fail

#![deny(warnings)]

use auto_enums::enum_derive;

struct Foo<A>(A);

#[enum_derive(Transpose)]
enum Enum1<A, B> {
    //~^ ERROR `enum_derive(Transpose)` all fields need to be generics
    A(Foo<A>),
    B(B),
}

fn main() {}
