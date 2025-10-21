use clap::ValueEnum;
use sqlx::Type;
use std::fmt::Display;

#[derive(Debug, Clone, Type, ValueEnum)]
#[sqlx(type_name = "TEXT")]
#[sqlx(rename_all = "UPPERCASE")]
pub enum NormalBalance {
    Debit,
    Credit,
}

impl Display for NormalBalance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
