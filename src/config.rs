use anyhow::Context;
use dotenvy::var;

pub struct Config {
    pub port: i32,
    pub database_url: String,
}

impl Config {
    fn int_var_or(name: &str, default_value: i32) -> i32 {
        Self::int_var(name).unwrap_or_else(|_| default_value)
    }

    fn int_var(name: &str) -> anyhow::Result<i32> {
        var(name)?
            .parse::<i32>()
            .context(format!("{} must be an integer", name))
    }

    pub fn from_env() -> anyhow::Result<Self> {
        let port = Self::int_var_or("PORT", 6969);
        let database_url = var("DATABASE_URL").context("DATABASE_URL must be set")?;
        Ok(Self { port, database_url })
    }
}
