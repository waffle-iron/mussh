// Copyright (c) 2016 mussh developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! mussh - SSH Multiplexing
#![cfg_attr(feature="cargo-clippy", allow(unseparated_literal_suffix))]
#![deny(missing_docs)]
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate slog;

extern crate chrono;
extern crate clap;
extern crate serde;
extern crate slog_async;
extern crate slog_term;
extern crate ssh2;
extern crate toml;

mod config;
mod error;
mod run;

use std::process;

/// mussh entry point
fn main() {
    match run::run() {
        Ok(i) => process::exit(i),
        Err(_e) => process::exit(1),
    }
}
