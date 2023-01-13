use std::env;

use tt_rust::run;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mode = &args[1];

    run(mode);
}
