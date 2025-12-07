#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_public;

use rustc_middle::ty::TyCtxt;
use std::ops::ControlFlow;

fn main() {
    let rustc_args: Vec<_> = std::env::args().collect();
    _ = rustc_public::run_with_tcx!(&rustc_args, analysis);
}

fn analysis(tcx: TyCtxt) -> ControlFlow<(), ()> {
    let local_crate = rustc_public::local_crate();
    println!("{:?}", local_crate.name);
    for f in local_crate.fn_defs() {}

    ControlFlow::Break(())
}
