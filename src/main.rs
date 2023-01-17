use std::env;

use tt_rust::run;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mode = if args.len() == 2 {
        &args[1]
    } else {
        ""
    };

    run(mode);
}
