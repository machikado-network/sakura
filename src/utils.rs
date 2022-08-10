use colored::Colorize;
use std::ffi::OsStr;
use std::process::Command;

macro_rules! error {
    ( $x:expr ) => {
        $x.bright_red().bold()
    };
}

pub fn run_command_and_wait<I, S>(cmd: &str, args: I)
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let r = Command::new(cmd).args(args).spawn().unwrap();
    let output = r.wait_with_output().unwrap();
    if !output.status.success() {
        println!("    {} when running `{cmd}`", error!("Error"))
    }
}
