use clap::Parser;

#[derive(Debug, Clone, Parser)]
pub struct Args {
    #[arg(long, short)]
    pub quick: bool,
}
