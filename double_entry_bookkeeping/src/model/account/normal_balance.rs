use clap::ValueEnum;
use sqlx::Type;

#[derive(Debug, Clone, Type, ValueEnum)]
#[sqlx(type_name = "TEXT")]
#[sqlx(rename_all = "UPPERCASE")]
pub enum NormalBalance {
    Debit,
    Credit,
}
