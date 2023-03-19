use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long, default_value_t=10, value_name="minutes")]
    work: u8,
    #[arg(short, long, default_value_t=2, value_name="minutes")]
    short: u8,
    #[arg(short, long, default_value_t=5, value_name="minutes")]
    long: u8,
}

fn main() {
    let args = Args::parse();
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
