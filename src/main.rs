#![allow(unused)]
// #![feature(associated_type_defaults)]

use std::{process::Output, rc::Rc};
pub mod combinators;
pub mod data;
pub mod formats;
pub mod system;
mod dev;

fn main() {
    let source = "Hello World!";
    dev::dev();
}


// impl<O> Callable for Func<I, O>

// impl Fn for Callable {
//     fn call(&self, args: Args) -> Self::Output {
//         unimplemented!()
//     }
// }