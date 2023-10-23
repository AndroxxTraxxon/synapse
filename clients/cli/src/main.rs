mod commands;

use structopt::StructOpt;
use commands::CLI;

fn main() {
    let args = CLI::from_args();
    println!("{:?}", args);
}
