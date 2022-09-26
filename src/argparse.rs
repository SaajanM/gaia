use std::path::PathBuf;

use clap::Parser;

pub fn parse() -> CliArgs {
    CliArgs::parse()
}

#[derive(Parser, Debug)]
#[clap(author="Saajan Maslanka",version,about="A procedural hex based terrain generator",long_about =None)]
pub struct CliArgs {
    #[clap(short = 'p', long, default_value_t = 1)]
    pub thread_count: usize,
    #[clap(short, long)]
    pub world_path: PathBuf,
    #[clap(short = 'x', long, default_value_t = false)]
    pub overwrite: bool,
}
