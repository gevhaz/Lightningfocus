mod args;

use clap::Parser;

fn main() {
    let args = args::Args::parse();
    println!("The work interval is set to {} minutes.", args.work);
    println!("The short break interval is set to {} minutes.", args.short);
    println!("The long break interval is set to {} minutes.", args.long);
    loop {
        for _ in 0..4 {
            println!("Work for {} minutes.", args.work);
            println!("Take a short break for {} minutes.", args.short);
        }
        println!("Take a long break for {} minutes.", args.long);
    }
}
