use anyhow::{Context, ensure};

#[derive(Debug)]
pub struct Config {
    pub database_url: String,
    pub public_origin: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Self::from_lookup(|name| std::env::var(name).ok())
    }

    pub fn secure_cookies(&self) -> bool {
        self.public_origin.starts_with("https://")
    }

    fn from_lookup(lookup: impl Fn(&str) -> Option<String>) -> anyhow::Result<Self> {
        let required = |name: &str| lookup(name).with_context(|| format!("{name} must be set"));
        let public_origin = required("PUBLIC_URL")?;
        let scheme_ok =
            public_origin.starts_with("http://") || public_origin.starts_with("https://");
        let host = public_origin.split_once("://").map_or("", |(_, rest)| rest);
        ensure!(
            scheme_ok && !host.is_empty() && !host.contains('/'),
            "PUBLIC_URL must be an origin like https://example.com, got {public_origin}"
        );
        Ok(Self {
            database_url: required("DATABASE_URL")?,
            public_origin,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Config;

    fn config(public_url: &'static str) -> anyhow::Result<Config> {
        Config::from_lookup(move |name| match name {
            "DATABASE_URL" => Some("postgres://db/app".to_string()),
            "PUBLIC_URL" => Some(public_url.to_string()),
            _ => None,
        })
    }

    #[test]
    fn reads_required_variables() {
        let config = config("https://notes.example.com").unwrap();
        assert_eq!(config.database_url, "postgres://db/app");
        assert!(config.secure_cookies());
    }

    #[test]
    fn plain_http_turns_secure_cookies_off() {
        assert!(!config("http://localhost:8090").unwrap().secure_cookies());
    }

    #[test]
    fn rejects_a_url_that_is_not_an_origin() {
        for bad in [
            "localhost:8090",
            "https://",
            "https://example.com/",
            "ftp://example.com",
        ] {
            assert!(config(bad).is_err(), "{bad} should be rejected");
        }
    }

    #[test]
    fn missing_variable_names_it() {
        let error = Config::from_lookup(|_| None).unwrap_err();
        assert_eq!(error.to_string(), "PUBLIC_URL must be set");
    }
}
