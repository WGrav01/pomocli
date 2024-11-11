use std::ffi::OsString;

fn help_message() {
    let help: &str = "
    Usage: pomocli [OPTIONS]

    Options:
        When starting a new timer:
            -f <HH:MM:SS>, --focus-time <HH:MM:SS>: Time spent in a focus session (default: 25 minutes)
            -b <HH:MM:SS>, --break-time <HH:MM:SS>: Time spent during a short break (default: 5 minutes)
            -B <HH:MM:SS>, --break-long-time <HH:MM:SS>: Time spent during a long break (default: 15 minutes)
            -c <number>, --cycle <number>: Number of cycles before long break (default: 4)
        Interacting with a running timer:
            -p, --pause: Pause the timer
            -r, --resume: Resume the timer
            -s, --stop: Stop the timer
            -S, --skip: Skip the current cycle
            -a, --attach: Attach to a running timer
        Misc:
            -h, --help: Show this help message and exit, will ignore all other arguments
            -v, --verbose: Show detailed debug information

    Version [placeholder]
    ";

    println!("{}", help);
}

struct Settings {
    focus_time: u32,
    break_time: u32,
    break_long_time: u32,
    cycle: u32,
    pause: bool,
    resume: bool,
    stop: bool,
    skip: bool,
    attach: bool,
    verbose: bool,
    help: bool
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            focus_time: 25,
            break_time: 5,
            break_long_time: 15,
            cycle: 4,
            pause: false,
            resume: false,
            stop: false,
            skip: false,
            attach: false,
            verbose: false,
            help: false
        }
    }
}

fn parse_args(args: Vec<OsString>) -> Settings {
    // TODO: parse args
}

fn main() {
}