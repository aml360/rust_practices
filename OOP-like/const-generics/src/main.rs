#![feature(generic_const_exprs)]

struct S<const V: u32> {}

enum If<const COND: bool> {}
trait True {}
impl True for If<true> {}

impl<const V: u32> S<V> {
    const fn ordinal(&self) -> &'static str {
        match V {
            1 => "first",
            2 => "second",
            3 => "third",
            4 => "fourth",
            5 => "fifth",
            6 => "sixth",
            7 => "seventh",
            8 => "eighth",
            9 => "ninth",
            10 => "tenth",
            _ => "",
        }
    }
}

fn main() {
    let s1: S<1> = S {};
    let s3: S<3> = S {};
    let s4: S<4> = S {};
    let s2: S<2> = S {};
    // Complete the secuence
    println!("{}", s1.ordinal());
    println!("{}", s2.ordinal());
    println!("{}", s3.ordinal());
    println!("{}", s4.ordinal());
}
