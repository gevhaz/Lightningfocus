mod args;

extern crate libnotify;

use clap::Parser;
use core::time::Duration;
use std::io;
use std::io::BufRead;
use std::process::{Command, Stdio};
use std::str;
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
    let body = &format!("{} for {} minutes", interval_message, duration);
    let summary = "Lightningfocus";
    Command::new("canberra-gtk-play")
        .arg("--id")
        .arg("complete")
        .arg("--description")
        .arg("Lightningfocus pomodoro notification")
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| format!("Couldn't play notification sound: {e}"))?;

    let notification = libnotify::Notification::new(summary, Some(body.as_ref()), None);
    notification
        .show()
        .map_err(|e| format!("Failed to show notification: {e}"))?;
    println!("{body}");
    thread::sleep(Duration::from_secs(MINUTE * duration));
    Ok(())
}

pub fn prompt_for_tasks() -> Result<Vec<String>, String> {
    println!("What tasks do you want to work on? (empty starts the timer)");
    let mut tasks: Vec<String> = vec![];

    let mut lines = io::stdin().lock().lines();
    while let Some(line) = lines.next() {
        let last_input = line.unwrap();
        if last_input.len() == 0 {
            break;
        }
        tasks.push(last_input);
    }
    Ok(tasks)
}

pub fn run(args: args::Args) -> Result<(), String> {
    libnotify::init("lightningfocus").map_err(|e| format!("Failed to intialize libnotify: {e}"))?;

    let tasks: Vec<String> = prompt_for_tasks().unwrap();
    let mut interval_number: usize = 0;
    loop {
        if tasks.len() == 0 {
            switch_interval("Work", args.work)
                .map_err(|e| format!("Failed to switch interval: {e}"))?;
        } else {
            switch_interval(
                &format!("Work on task '{}'", tasks[interval_number % tasks.len()]),
                args.work,
            )
            .map_err(|e| format!("Failed to switch interval: {e}"))?;
        }
        for _ in 0..3 {
            switch_interval("Take a short break", args.short)
                .map_err(|e| format!("Failed to switch interval: {e}"))?;
            interval_number += 1;
            if tasks.len() == 0 {
                switch_interval("Work", args.work)
                    .map_err(|e| format!("Failed to switch interval: {e}"))?;
            } else {
                switch_interval(
                    &format!("Work on task '{}'", tasks[interval_number % tasks.len()]),
                    args.work,
                )
                .map_err(|e| format!("Failed to switch interval: {e}"))?;
            }
        }
        switch_interval("Take a long break", args.long)
            .map_err(|e| format!("Failed to switch interval: {e}"))?;
        interval_number += 1;
    }
}
