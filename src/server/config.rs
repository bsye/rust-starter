use anyhow::{Context, bail, ensure};

use crate::analytics::AnalyticsSettings;

const ANALYTICS_TOKEN: &str = "POSTHOG_PROJECT_TOKEN";
const ANALYTICS_HOST: &str = "POSTHOG_HOST";

#[derive(Debug)]
pub struct Config {
    pub database_url: String,
    pub public_origin: String,
    pub analytics: Option<AnalyticsSettings>,
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
        Ok(Self {
            public_origin: origin("PUBLIC_URL", required("PUBLIC_URL")?)?,
            database_url: required("DATABASE_URL")?,
            analytics: analytics(&lookup)?,
        })
    }
}

fn analytics(lookup: impl Fn(&str) -> Option<String>) -> anyhow::Result<Option<AnalyticsSettings>> {
    match (lookup(ANALYTICS_TOKEN), lookup(ANALYTICS_HOST)) {
        (None, None) => Ok(None),
        (Some(token), Some(host)) => Ok(Some(AnalyticsSettings {
            token,
            host: origin(ANALYTICS_HOST, host)?,
        })),
        _ => bail!("{ANALYTICS_TOKEN} and {ANALYTICS_HOST} must be set together"),
    }
}

fn origin(name: &str, value: String) -> anyhow::Result<String> {
    let scheme_ok = value.starts_with("http://") || value.starts_with("https://");
    let host = value.split_once("://").map_or("", |(_, rest)| rest);
    ensure!(
        scheme_ok && !host.is_empty() && !host.contains('/'),
        "{name} must be an origin like https://example.com, got {value}"
    );
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::Config;

    fn config_with(
        public_url: &'static str,
        analytics: &'static [(&'static str, &'static str)],
    ) -> anyhow::Result<Config> {
        Config::from_lookup(move |name| match name {
            "DATABASE_URL" => Some("postgres://db/app".to_string()),
            "PUBLIC_URL" => Some(public_url.to_string()),
            _ => analytics
                .iter()
                .find(|(key, _)| *key == name)
                .map(|(_, value)| value.to_string()),
        })
    }

    fn config(public_url: &'static str) -> anyhow::Result<Config> {
        config_with(public_url, &[])
    }

    fn analytics(pairs: &'static [(&'static str, &'static str)]) -> anyhow::Result<Config> {
        config_with("https://notes.example.com", pairs)
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

    #[test]
    fn analytics_is_off_when_both_variables_are_unset() {
        assert_eq!(config("https://notes.example.com").unwrap().analytics, None);
    }

    #[test]
    fn analytics_reads_token_and_host() {
        let settings = analytics(&[
            ("POSTHOG_PROJECT_TOKEN", "phc_test"),
            ("POSTHOG_HOST", "https://eu.i.posthog.com"),
        ])
        .unwrap()
        .analytics
        .unwrap();
        assert_eq!(settings.token, "phc_test");
        assert_eq!(settings.host, "https://eu.i.posthog.com");
    }

    #[test]
    fn analytics_needs_both_variables() {
        for pairs in [
            &[("POSTHOG_PROJECT_TOKEN", "phc_test")][..],
            &[("POSTHOG_HOST", "https://eu.i.posthog.com")][..],
        ] {
            let error = analytics(pairs).unwrap_err();
            assert_eq!(
                error.to_string(),
                "POSTHOG_PROJECT_TOKEN and POSTHOG_HOST must be set together"
            );
        }
    }

    #[test]
    fn analytics_host_must_be_an_origin() {
        let error = analytics(&[
            ("POSTHOG_PROJECT_TOKEN", "phc_test"),
            ("POSTHOG_HOST", "eu.i.posthog.com"),
        ])
        .unwrap_err();
        assert!(
            error
                .to_string()
                .starts_with("POSTHOG_HOST must be an origin")
        );
    }
}
