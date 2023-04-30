mod args;

extern crate libnotify;

use clap::Parser;
use core::time::Duration;
use crossterm::{
    event::{self, read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    cursor::{Hide, Show},
};
use std::error::Error;
use std::io;
use std::io::BufRead;
use std::process::{Command, Stdio};
use std::str;
use std::thread;
use tui::{
    backend::{Backend, CrosstermBackend},
    // layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders},
    Frame,
    Terminal,
};
use tui_input::backend::crossterm as be;
use tui_input::backend::crossterm::EventHandler;
use tui_input::Input;

const MINUTE: u64 = 1;

fn main() -> Result<(), Box<dyn Error>> {
    let args = args::Args::parse();
    if let Err(e) = run(args) {
        eprintln!("{e}");
        std::process::exit(1);
    }
    Ok(())
}

pub fn run(_args: args::Args) -> Result<(), String> {
    enable_raw_mode().map_err(|e| format!("Failed enabling crossterm raw mode: {e}"))?;
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    execute!(stdout, Hide, EnterAlternateScreen, EnableMouseCapture)
        .map_err(|e| format!("Error switching to alternate screen: {e}"))?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal =
        Terminal::new(backend).map_err(|e| format!("Error initializing TUI: {e}"))?;
    let mut input: Input = "Hello".into();
    be::write(
        &mut terminal.backend_mut(),
        input.value(),
        input.cursor(),
        (0, 0),
        15,
    )
    .map_err(|e| format!("Failed to render UI {e}"))?;
    terminal.flush().map_err(|e| format!("Failed to flush STDOUT: {e}"))?;

    libnotify::init("lightningfocus").map_err(|e| format!("Failed to intialize libnotify: {e}"))?;

    let mut tasks: Vec<String> = vec![];
    loop {
        // let tasks: Vec<String> = prompt_for_tasks().unwrap();
        let event = read().map_err(|e| format!("Failed reading key {e}"))?;
        match event {
            Event::Key(KeyEvent { code, .. }) => match code {
                KeyCode::Esc | KeyCode::Enter => {
                    tasks.push("Test".to_owned());
                    break;
                }
                _ => {
                    if input.handle_event(&event).is_some() {
                        be::write(
                            &mut terminal.backend_mut(),
                            input.value(),
                            input.cursor(),
                            (0, 0),
                            15,
                        )
                        .map_err(|e| format!("Failed to render UI {e}"))?;
                        // terminal.draw()
                        terminal.flush().map_err(|e| format!("Failed to flush STDOUT: {e}"))?;
                    }
                }
            },
            _ => {}
        }
    }
    // let mut interval_number: usize = 0;
    loop {
        terminal
            .draw(main_screen)
            .map_err(|e| format!("Failed drawing terminal: {e}"))?;
        if let Event::Key(key) = event::read().map_err(|e| format!("Failed reading key {e}"))? {
            if let KeyCode::Char('q') = key.code {
                break;
            }
        }

        // if tasks.len() == 0 {
        //     switch_interval("Work", args.work)
        //         .map_err(|e| format!("Failed to switch interval: {e}"))?;
        // } else {
        //     switch_interval(
        //         &format!("Work on task '{}'", tasks[interval_number % tasks.len()]),
        //         args.work,
        //     )
        //     .map_err(|e| format!("Failed to switch interval: {e}"))?;
        // }
        // for _ in 0..3 {
        //     switch_interval("Take a short break", args.short)
        //         .map_err(|e| format!("Failed to switch interval: {e}"))?;
        //     interval_number += 1;
        //     if tasks.len() == 0 {
        //         switch_interval("Work", args.work)
        //             .map_err(|e| format!("Failed to switch interval: {e}"))?;
        //     } else {
        //         switch_interval(
        //             &format!("Work on task '{}'", tasks[interval_number % tasks.len()]),
        //             args.work,
        //         )
        //         .map_err(|e| format!("Failed to switch interval: {e}"))?;
        //     }
        // }
        // switch_interval("Take a long break", args.long)
        //     .map_err(|e| format!("Failed to switch interval: {e}"))?;
        // interval_number += 1;
    }
    disable_raw_mode().map_err(|e| format!("Failed disabling crossterm raw mode: {e}"))?;
    execute!(
        terminal.backend_mut(),
        Show,
        LeaveAlternateScreen,
        DisableMouseCapture,
    )
    .map_err(|e| format!("Failed to leave alternate screen {e}"))?;
    terminal
        .show_cursor()
        .map_err(|e| format!("Failed showing cursor {e}"))?;

    Ok(())
}

fn main_screen<B: Backend>(f: &mut Frame<B>) {
    let size = f.size();
    let block = Block::default()
        .title("Lightningfocus")
        .borders(Borders::ALL);
    f.render_widget(block, size);
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
