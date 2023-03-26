mod args;

extern crate libnotify;

use clap::Parser;
use core::time::Duration;
use std::thread;

const MINUTE: u64 = 60;

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
    libnotify::init("lightningfocus").map_err(|e| format!("Failed to intialize libnotify: {e}"))?;
    let work_notification =
        libnotify::Notification::new(&format!("Work for {} mintues", args.work), None, None);
    let short_notification = libnotify::Notification::new(
        &format!("Take a short break for {} mintues", args.short),
        None,
        None,
    );
    let long_notification = libnotify::Notification::new(
        &format!("Take a long break for {} mintues", args.long),
        None,
        None,
    );
    loop {
        println!("Work for {} minutes.", args.work);
        work_notification.show().map_err(|e| format!("Failed to show notification: {e}"))?;
        thread::sleep(Duration::from_secs(MINUTE * args.work));
        for _ in 0..3 {
            println!("Take a short break for {} minutes.", args.short);
            short_notification.show().map_err(|e| format!("Failed to show notification: {e}"))?;
            thread::sleep(Duration::from_secs(MINUTE * args.short));
            println!("Work for {} minutes.", args.work);
            work_notification.show().map_err(|e| format!("Failed to show notification: {e}"))?;
            thread::sleep(Duration::from_secs(MINUTE * args.work));
        }
        println!("Take a long break for {} minutes.", args.long);
        long_notification.show().map_err(|e| format!("Failed to show notification: {e}"))?;
        thread::sleep(Duration::from_secs(MINUTE * args.long));
    }
}
