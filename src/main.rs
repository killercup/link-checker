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

#[macro_use] extern crate quick_error;

extern crate tendril;
extern crate html5ever;

use std::env;
use std::fs::File;
use std::io::{stderr, Write};
use std::process::exit;

mod errors;
mod lint;

use errors::LinkCheckerError;

fn handle_main(path: &str) -> Result<(), LinkCheckerError> {
    let mut file = try!(File::open(path));
    lint::lint_html_links(&mut file)
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
