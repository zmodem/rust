#![feature(rustc_attrs)]
//@ edition: 2024
//@ compile-flags: -Cinstrument-coverage
//@ needs-profiler-runtime

// Check that instrumenting a crate with a comptime function doesn't ICE.
// (The function itself doesn't need to be instrumented, and probably shouldn't be.)
// Regression test for <https://github.com/rust-lang/rust/pull/161808>.

#[rustc_comptime]
fn comptime_fn() {}

fn main() {}
