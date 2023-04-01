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

pub fn switch_interval(interval_message: &str, duration: u64) -> Result<(), String> {
    let message = &format!("{} for {} minutes", interval_message, duration);
    let notification = libnotify::Notification::new(message, None, None);
    notification
        .show()
        .map_err(|e| format!("Failed to show notification: {e}"))?;
    println!("{message}");
    thread::sleep(Duration::from_secs(MINUTE * duration));
    Ok(())
}

pub fn run(args: args::Args) -> Result<(), String> {
    libnotify::init("lightningfocus").map_err(|e| format!("Failed to intialize libnotify: {e}"))?;
    loop {
        switch_interval("Work", args.work)
            .map_err(|e| format!("Failed to switch interval: {e}"))?;
        for _ in 0..3 {
            switch_interval("Take a short break", args.short)
                .map_err(|e| format!("Failed to switch interval: {e}"))?;
            switch_interval("Work", args.work)
                .map_err(|e| format!("Failed to switch interval: {e}"))?;
        }
        switch_interval("Take a long break", args.long)
            .map_err(|e| format!("Failed to switch interval: {e}"))?;
    }
}
