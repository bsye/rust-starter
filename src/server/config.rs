use anyhow::Context;

#[derive(Debug)]
pub struct Config {
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Self::from_lookup(|name| std::env::var(name).ok())
    }

    fn from_lookup(lookup: impl Fn(&str) -> Option<String>) -> anyhow::Result<Self> {
        let required = |name: &str| lookup(name).with_context(|| format!("{name} must be set"));
        Ok(Self {
            database_url: required("DATABASE_URL")?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Config;

    #[test]
    fn reads_required_variables() {
        let config = Config::from_lookup(|_| Some("postgres://db/app".to_string())).unwrap();
        assert_eq!(config.database_url, "postgres://db/app");
    }

    #[test]
    fn missing_variable_names_it() {
        let error = Config::from_lookup(|_| None).unwrap_err();
        assert_eq!(error.to_string(), "DATABASE_URL must be set");
    }
}
