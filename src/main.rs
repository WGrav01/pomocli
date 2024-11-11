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

fn main() {
    // TODO: Parse command line arguments
}