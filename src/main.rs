extern crate gna;

use gna::cli;

fn main() {
    cli::run(std::env::args().collect());
}
