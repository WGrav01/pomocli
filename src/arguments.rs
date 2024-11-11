use clap::Parser;

#[derive(Parser, Debug)]
#[command(about, long_about)]
pub struct Args {
    /// Time spent in a focus session
    #[arg(
        global = true,
        short, 
        long, 
        default_value_t = String::from("25:00")
    )]
    focus_time: String,

    /// Time spent during a short break
    #[arg(
        global = true,
        short, 
        long, 
        default_value_t = String::from("5:00")
    )]
    break_time: String,

    /// Time spent during a long break
    #[arg(
        global = true,
        short, 
        long, 
        default_value_t = String::from("15:00")
    )]
    long_break_time: String,

    /// Number of cycles before long break
    #[arg(
        global = true,
        short, 
        long, 
        default_value_t = 4
    )]
    cycles: u32,

    /// Disable notifications
    #[arg(
        global = true,
        short,
        long,
        default_value_t = false 
    )]
    no_notify: bool,

    /// Pause an existing timer
    #[arg(
        global = true,
        short, 
        long, 
        default_value_t = false
    )]
    pause: bool,

    /// Resume an existing timer
    #[arg(
        global = true,
        short, 
        long, 
        default_value_t = false
    )]
    resume: bool,

    /// Delete an existing timer
    #[arg(
        global = true,
        short, 
        long, 
        default_value_t = false
    )]
    delete: bool,

    /// Skip the current cycle in an existing timer
    #[arg(
        global = true,
        short, 
        long, 
        default_value_t = false
    )]
    skip: bool,

    /// View the status of an existing timer without attaching
    #[arg(
        global = true,
        short,
        long,
        default_value_t = false
    )]
    view: bool,

    /// Attach to an existing timer
    #[arg(
        global = true,
        short, 
        long, 
        default_value_t = false
    )]
    attach: bool
}
