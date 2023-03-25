use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    #[arg(short, long, default_value_t = 10, value_name = "minutes")]
    pub work: u64,
    #[arg(short, long, default_value_t = 2, value_name = "minutes")]
    pub short: u64,
    #[arg(short, long, default_value_t = 5, value_name = "minutes")]
    pub long: u64,
}
