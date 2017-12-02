#![feature(proc_macro, conservative_impl_trait, generators)]
extern crate futures_await as futures;
extern crate tokio_core;

use std::io;
use futures::prelude::*;
use tokio_core::reactor::Core;

fn main() {
    println!("Hello, world!");

    let mut core = Core::new().unwrap();

    core.run(base_fn()).unwrap();
}

#[async]
fn base_fn() -> Result<(), io::Error> {
    await!(recurse_fn())?;
    Ok(())
}

#[async]
fn recurse_fn() -> Result<(), io::Error> {
    await!(recurse_fn())
}