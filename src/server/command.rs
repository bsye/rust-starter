use anyhow::{Context, bail};

const USAGE: &str = "usage: rust-starter [seed <email> <count>]";

#[derive(Debug, PartialEq)]
pub enum Command {
    Serve,
    Seed { email: String, count: u64 },
}

impl Command {
    pub fn parse(mut args: impl Iterator<Item = String>) -> anyhow::Result<Self> {
        let command = match args.next().as_deref() {
            None => Self::Serve,
            Some("seed") => Self::Seed {
                email: args.next().context(USAGE)?,
                count: args
                    .next()
                    .context(USAGE)?
                    .parse()
                    .context("count must be a whole number")?,
            },
            Some(other) => bail!("unknown command {other}; {USAGE}"),
        };
        if args.next().is_some() {
            bail!(USAGE);
        }
        Ok(command)
    }
}

#[cfg(test)]
mod tests {
    use super::Command;

    fn parse(args: &[&str]) -> anyhow::Result<Command> {
        Command::parse(args.iter().map(|arg| arg.to_string()))
    }

    #[test]
    fn no_arguments_serves() {
        assert_eq!(parse(&[]).unwrap(), Command::Serve);
    }

    #[test]
    fn seed_reads_email_and_count() {
        assert_eq!(
            parse(&["seed", "you@example.com", "10000"]).unwrap(),
            Command::Seed {
                email: "you@example.com".to_string(),
                count: 10_000,
            }
        );
    }

    #[test]
    fn seed_without_a_count_shows_usage() {
        let error = parse(&["seed", "you@example.com"]).unwrap_err();
        assert!(error.to_string().starts_with("usage:"));
    }

    #[test]
    fn seed_rejects_a_count_that_is_not_a_number() {
        let error = parse(&["seed", "you@example.com", "many"]).unwrap_err();
        assert_eq!(error.to_string(), "count must be a whole number");
    }

    #[test]
    fn unknown_command_is_rejected() {
        let error = parse(&["migrate"]).unwrap_err();
        assert!(error.to_string().starts_with("unknown command migrate"));
    }

    #[test]
    fn extra_arguments_are_rejected() {
        assert!(parse(&["seed", "you@example.com", "10", "more"]).is_err());
    }
}
