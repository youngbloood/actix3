pub use clap::Clap;
use lazy_static::lazy_static;
use std::string::String;
/// This doc string acts as a help message when the user runs '--help'
/// as do all doc strings on fields
#[derive(Clap)]
#[clap(version = "1.0", author = "https://github.com/youngbloood")]
pub struct Opts {
    /// Sets a custom config file. Could have been an Option<T> with no default too
    #[clap(short, long, default_value = "./config.toml")]
    pub config: String,
    /// Some input. Because this isn't an Option<T> it's required to be used
    input: String,
    /// A level of verbosity, and can be used multiple times
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,
    // #[clap(subcommand)]
    // subcmd: SubCommand,
}

// #[derive(Clap)]
// enum SubCommand {
//     #[clap(version = "1.3", author = "Someone E. <someone_else@other.com>")]
//     Test(Test),
// }

// #[derive(Clap)]
// struct Test {
//     /// Print debug info
//     #[clap(short)]
//     debug: bool
// }



// lazy_static! {
//     #[derive(Debug)]
//     pub static ref CONFIG_PATH :String= Opts::parse().config;
// }
pub static mut CONFIG_PATH :String=String::new();