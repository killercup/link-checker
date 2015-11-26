//! This is [not][zalgo] a good idea. Also, it's gonna be slow.
//!
//! [zalgo]: http://stackoverflow.com/a/1732454

#![deny(missing_docs,
        missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unsafe_code,
        unstable_features,
        unused_import_braces, unused_qualifications)]
#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]

extern crate regex;
#[macro_use]
extern crate quick_error;

use std::env;
use std::fs::File;
use std::io::{stderr, Read, Write};
use std::process::exit;

mod errors;
mod lint;

use errors::LinkCheckerError;

fn handle_main(path: &str) -> Result<(), LinkCheckerError> {
    let mut file = try!(File::open(path));
    let mut buffer = String::new();
    try!(file.read_to_string(&mut buffer));

    lint::lint_html_links(&buffer).map_err(errors::LinkCheckerError::MissingLinks)
}

fn main() {
    let path = env::args()
                   .skip(1)
                   .next()
                   .expect("First argument needs to be a path to an HTML file.");

    match handle_main(&path) {
        Ok(_) => println!("Yay!"),
        Err(err) => {
            writeln!(stderr(), "{}", err).unwrap();
            exit(1);
        }
    }
}
