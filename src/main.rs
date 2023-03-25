mod args;

use clap::Parser;

fn main() {
    let args = args::Args::parse();
    if let Err(e) = run(args) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

pub fn run(args: args::Args) -> Result<(), String> {
    println!("The work interval is set to {} minutes.", args.work);
    println!("The short break interval is set to {} minutes.", args.short);
    println!("The long break interval is set to {} minutes.", args.long);
    loop {
        println!("Work for {} minutes.", args.work);
        for _ in 0..3 {
            println!("Take a short break for {} minutes.", args.short);
            println!("Work for {} minutes.", args.work);
        }
        println!("Take a long break for {} minutes.", args.long);
    }
}
