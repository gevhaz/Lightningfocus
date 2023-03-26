# Lightningfocus

## Description

Lightningfocus is a command line Pomodoro timer written in Rust. It is intended
for rotating between a small number of tasks but works just as well as a classic
Pomodoro timer.

## Installation

As Lightningfocus is in the early stages of development, the best way to install
it is to clone the repository and install with `cargo`, like so:

```command
$ git clone https://github.com/gevhaz/Lightningfocus
$ cargo install
````

You will need to have the system package `libnotify` installed (or your
distribution's equivalent).

## Usage

The following commands assume that Lightningfocus is installed, but you can run
any of them by prepening `cargo run` to them when the root of the repository is
your current working directory.

You can (not yet) start a Pomodoro session with the default intervals like such:

```command
$ focus
```

The default intervals are:

| **Interval type** | **Length in minutes** |
| ----------------- | --------------------- |
| Work              | 10                    |
| Short break       | 2                     |
| Long break        | 5                     |

Once you've run the command, a Work interval starts, and you are shown a desktop
notification and a message in your terminal. You are notified in the same way
when it is time for breaks or to work again.

## Capabilities

- [x] Allow setting duration for work, short break, and long break intervals via
      command line.
- [ ] Show current interval type and name in standard output.
- [ ] Show time passed for current interval in standard output.
- [x] Show notifications when switching between work and breaks.
- [ ] Allow interactive selection of tasks. The user should be able to enter
      tasks one-by-one which will then be looped through.
- [x] It should be possible to use the app as a Pomodoro timer without naming
      any tasks.
- [ ] Allow setting default intervals in a configuration file.
