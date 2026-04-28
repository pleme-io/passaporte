// Typed Passaporte config.

use serde::{Deserialize, Serialize};

/// One typed identity primitive — names the IdP backend, federations,
/// session policy, and KEDA tuning.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Passaporte {
    pub idp: IdpBackend,
    pub host: String,

    #[serde(default)]
    pub federated: Vec<SocialProvider>,

    #[serde(default = "default_scopes")]
    pub scopes: Vec<String>,

    /// Session lifetime in seconds. None = use Authentik's default (24h).
    #[serde(default = "default_session_secs")]
    pub session_duration_secs: u64,

    #[serde(default)]
    pub keda: KedaConfig,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IdpBackend {
    Authentik,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SocialProvider {
    Google,
    Github,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct KedaConfig {
    pub enabled: bool,
    pub cooldown_period_secs: u64,
    pub cold_start_budget_secs: u64,
}

impl Default for KedaConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            cooldown_period_secs: 600,
            cold_start_budget_secs: 15,
        }
    }
}

fn default_scopes() -> Vec<String> {
    vec![
        "openid".into(),
        "email".into(),
        "profile".into(),
        "groups".into(),
    ]
}

const fn default_session_secs() -> u64 {
    24 * 3600
}

impl Passaporte {
    /// The canonical fleet passaporte — Authentik at `auth.quero.cloud`,
    /// Google federated, 24h sessions, KEDA enabled.
    #[must_use]
    pub fn fleet_default() -> Self {
        Self {
            idp: IdpBackend::Authentik,
            host: "auth.quero.cloud".into(),
            federated: vec![SocialProvider::Google],
            scopes: default_scopes(),
            session_duration_secs: default_session_secs(),
            keda: KedaConfig::default(),
        }
    }
}
