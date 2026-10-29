mod browser;
#[cfg(feature = "server")]
mod server;

use serde::{Deserialize, Serialize};

pub use browser::use_analytics;
#[cfg(feature = "server")]
pub use server::{AnalyticsEvent, capture, start};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AnalyticsSettings {
    pub token: String,
    pub host: String,
}

pub fn distinct_id(user_id: i64) -> String {
    user_id.to_string()
}

pub fn request_settings() -> Option<AnalyticsSettings> {
    #[cfg(feature = "server")]
    return server::request_settings();
    #[cfg(not(feature = "server"))]
    unreachable!("the browser reads analytics settings from the server-rendered page")
}
