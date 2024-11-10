use std::ffi::OsString;

fn help_message() {
    println!("{}", "Usage: pomocli [options]");
    println!("{}", "Options");
    println!("{}", "-h, --help: Display this help message");
    println!("{}", "-f, --focus_time: Set focus time in minutes (default: 25)");
    println!("{}", "-v, --version: Display version information");
    println!("{}", "-s, --short_break_time: Set short break time in minutes");
    println!("{}", "-l, --long_break_time: Set long break time in minutes");
    println!("{}", "-c, --cycles: Set number of pomodoro cycles until log break");
    println!("{}", "-a, --attach: Attach to an existing timer");

    println!("");
    println!("{}", "Version [placeholder]")
}

fn main() {
    let args: Vec<_> = std::env::args_os().collect();

    if args.contains(&OsString::from("--help")) || args.contains(&OsString::from("-h")) {
        help_message();
        return;
    }
}