use clap::Parser;

#[derive(Parser, Debug)]
#[command()]
pub struct Cli{
    /// Which day to run
    pub day: u8,

    // Run part2
    #[arg(short, long)]
    pub part2: bool,

    /// Any additional arguments to parse though to the day
    pub extra_args: Vec<String>,
}