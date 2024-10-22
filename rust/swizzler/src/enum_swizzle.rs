use std::mem;

#[derive(Debug)]
pub enum MultiVariateEnum {
    A { name: String },
    B { name: String },
    C,
    D,
}

pub fn swizzle(e: &mut MultiVariateEnum) {
    use MultiVariateEnum::*;
    *e = match e {
 
        A { name } => B {
            name: mem::take(name),
        },
        B { name } => A {
            name: mem::take(name),
        },
        C => D,
        D => C,
    }
}
