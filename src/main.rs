use std::ffi::OsString;

fn help_message() {
    println!("{}", "Usage: pomocli [options]");
    println!("{}", "Options");
    println!("{}", "-h, --help: Display this help message and exit");
    println!("{}", "-f, --focus_time: Set focus time in HH:MM:SS (default: 25:00)");
    println!("{}", "-v, --verbose: Enable verbose debug output");
    println!("{}", "-s, --short_break_time: Set short break time in HH:MM:SS (default: 5:00)");
    println!("{}", "-l, --long_break_time: Set long break time in HH:MM:SS (default: 10:00)");
    println!("{}", "-c, --cycles: Set number of pomodoro cycles until log break (default: 4)");
    println!("{}", "-a, --auto_continue: Move onto the next pomodoro cycle automatically");
    println!("{}", "-A, --attach: Attach to an existing timer");

    println!(""); // A newline to separate the options from the version
    println!("{}", "Version [placeholder]")
}

fn main() {
    let args: Vec<_> = std::env::args_os().collect();

    let mut verbose: bool = false;

    if args.contains(&OsString::from("--verbose")) || args.contains(&OsString::from("-v")) {
        verbose = true;
        println!("Arguments received: {:?}", args);
    }

    if args.contains(&OsString::from("--help")) || args.contains(&OsString::from("-h")) {
        help_message();
        return;
    }
}