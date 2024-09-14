use clap::*;

#[derive(Debug, Clone, Parser)]
pub struct Args {
    #[arg(long, short)]
    pub quick: bool,

    #[arg(long, short)]
    pub extensions: Vec<CjkExt>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum CjkExt {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    #[value(alias = "compat")]
    Compatability,
}

impl CjkExt {
    pub const fn range(&self) -> core::ops::RangeInclusive<u32> {
        match self {
            Self::A => 0x03400..=0x04dbf,
            Self::B => 0x20000..=0x2a6df,
            Self::C => 0x2a700..=0x2b73f,
            Self::D => 0x2b740..=0x2b81f,
            Self::E => 0x2b820..=0x2ceaf,
            Self::F => 0x2ceb0..=0x2ebef,
            Self::G => 0x30000..=0x3134f,
            Self::H => 0x31350..=0x323af,
            Self::I => 0x2ebf0..=0x2ee5f,
            Self::Compatability => 0x09f00..=0x0faff,
        }
    }
}
