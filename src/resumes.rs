use std::fmt;

use clap::ValueEnum;

#[derive(ValueEnum, Clone, Debug)]
pub enum Available {
    Basic,
}

impl fmt::Display for Available {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Basic => write!(f, "basic"),
        }
    }
}
