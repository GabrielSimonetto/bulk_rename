use std::env;
use std::process;

use bulk_rename::Args;

fn main() {
    let args = Args::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Parent Dir: {}", args.parent_dir);
    println!("Input Pattern: {}", args.input_pattern);
    println!("Output Pattern: {}", args.output_pattern);

    if let Err(e) = bulk_rename::run(args) {
        eprintln!("Application Error: {}", e);

        process::exit(1);
    }
}
