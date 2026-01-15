use std::env::args;

use hydor::cli;

fn main() {
    cli::run(args().collect());
}
