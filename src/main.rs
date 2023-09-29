use pact::read_file;
use sarge::prelude::*;

fn main() {
    let parser = ArgumentParser::new();
    let files = parser.parse().expect("failed to parse arguments");
    
    if files.len() < 1 {
        panic!("not enough input");
    }

    let file = &files[0];

    let mut rim = read_file(file).expect("failed to read file");
    rim.run().expect("failed to run program");
}
