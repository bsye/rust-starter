use dioxus::fullstack::FullstackContext;
use posthog_rs::{ClientOptionsBuilder, Error, Event};
use tokio::signal::unix::{Signal, SignalKind, signal};

use super::{AnalyticsSettings, distinct_id};

const SHUTDOWN_TIMEOUT_MS: u64 = 5_000;

#[derive(Clone, Copy, Debug)]
pub enum AnalyticsEvent {
    SignedUp,
    SignedIn,
}

impl AnalyticsEvent {
    fn as_str(self) -> &'static str {
        match self {
            AnalyticsEvent::SignedUp => "user_signed_up",
            AnalyticsEvent::SignedIn => "user_signed_in",
        }
    }
}

pub async fn start(settings: Option<&AnalyticsSettings>) -> anyhow::Result<()> {
    let Some(settings) = settings else {
        return Ok(());
    };
    let options = ClientOptionsBuilder::default()
        .api_key(settings.token.clone())
        .host(settings.host.clone())
        .shutdown_timeout_ms(SHUTDOWN_TIMEOUT_MS)
        .build()?;
    match posthog_rs::init_global(options).await {
        Ok(()) => {
            tokio::spawn(flush_on_exit(signal(SignalKind::terminate())?));
            Ok(())
        }
        Err(Error::AlreadyInitialized) => Ok(()),
        Err(error) => Err(error.into()),
    }
}

pub fn capture(event: AnalyticsEvent, user_id: i64) {
    posthog_rs::capture(Event::new(event.as_str().to_string(), distinct_id(user_id)));
}

pub fn request_settings() -> Option<AnalyticsSettings> {
    let context =
        FullstackContext::current().expect("request_settings runs while rendering a request");
    context
        .extension::<Option<AnalyticsSettings>>()
        .expect("the analytics settings layer is installed")
}

async fn flush_on_exit(mut terminate: Signal) {
    tokio::select! {
        _ = terminate.recv() => {}
        _ = tokio::signal::ctrl_c() => {}
    }
    posthog_rs::shutdown().await;
    std::process::exit(0);
}
