// Render `Passaporte` → serde_yaml::Value matching the
// `lareira-passaporte` chart's values.yaml schema. The chart consumes
// these values verbatim; the renderer is the typed bridge.

use crate::config::{IdpBackend, Passaporte, SocialProvider};
use serde::Serialize;
use serde_yaml::{Mapping, Value};

/// Strongly-typed mirror of `lareira-passaporte`'s values.yaml schema.
/// Round-trips through serde_yaml; serves as both the renderer output
/// and a doc of the chart's contract.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PassaporteValues {
    pub enabled: bool,
    pub cloudflared: CloudflaredValues,
    pub keda: KedaValues,
    pub federated: Vec<String>,
    pub scopes: Vec<String>,
    pub session_duration: u64,
    pub authentik: AuthentikValues,
    pub compliance: ComplianceValues,
    pub service_monitor: ServiceMonitorValues,
}

#[derive(Debug, Clone, Serialize)]
pub struct CloudflaredValues {
    pub expose: bool,
    pub hostname: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct KedaValues {
    pub enabled: bool,
    #[serde(rename = "cooldownPeriod")]
    pub cooldown_period: u64,
    #[serde(rename = "coldStartBudget")]
    pub cold_start_budget: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct AuthentikValues {
    pub outposts: AuthentikOutposts,
    pub server: AuthentikServer,
}

#[derive(Debug, Clone, Serialize)]
pub struct AuthentikOutposts {
    pub discover: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct AuthentikServer {
    pub autoscaling: AuthentikAutoscaling,
}

#[derive(Debug, Clone, Serialize)]
pub struct AuthentikAutoscaling {
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct ComplianceValues {
    pub authn: ComplianceAuthn,
}

#[derive(Debug, Clone, Serialize)]
pub struct ComplianceAuthn {
    #[serde(rename = "allowList")]
    pub allow_list: ComplianceAllowList,
}

#[derive(Debug, Clone, Serialize)]
pub struct ComplianceAllowList {
    #[serde(rename = "unauthenticatedIngress")]
    pub unauthenticated_ingress: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceMonitorValues {
    pub enabled: bool,
    pub interval: String,
}

impl Passaporte {
    /// Render to typed values. The Helm chart consumes these.
    #[must_use]
    pub fn render(&self) -> PassaporteValues {
        match self.idp {
            IdpBackend::Authentik => {}
        }
        PassaporteValues {
            enabled: true,
            cloudflared: CloudflaredValues {
                expose: true,
                hostname: self.host.clone(),
            },
            keda: KedaValues {
                enabled: self.keda.enabled,
                cooldown_period: self.keda.cooldown_period_secs,
                cold_start_budget: self.keda.cold_start_budget_secs,
            },
            federated: self
                .federated
                .iter()
                .map(|p| match p {
                    SocialProvider::Google => "google".into(),
                    SocialProvider::Github => "github".into(),
                })
                .collect(),
            scopes: self.scopes.clone(),
            session_duration: self.session_duration_secs,
            authentik: AuthentikValues {
                outposts: AuthentikOutposts { discover: true },
                server: AuthentikServer {
                    autoscaling: AuthentikAutoscaling { enabled: false },
                },
            },
            compliance: ComplianceValues {
                authn: ComplianceAuthn {
                    allow_list: ComplianceAllowList {
                        unauthenticated_ingress: true,
                    },
                },
            },
            service_monitor: ServiceMonitorValues {
                enabled: true,
                interval: "30s".into(),
            },
        }
    }

    /// Render directly to YAML — convenience for the (defpassaporte …)
    /// → values.yaml flow.
    pub fn render_yaml(&self) -> Result<String, serde_yaml::Error> {
        serde_yaml::to_string(&self.render())
    }

    /// Render to a generic serde_yaml::Value for deeper inspection.
    pub fn render_value(&self) -> Result<Value, serde_yaml::Error> {
        serde_yaml::to_value(self.render())
    }
}

/// Helper: fetch a field from a rendered Value by dotted path.
/// Useful in tests + for downstream tools that want a single value.
#[must_use]
pub fn pick<'a>(value: &'a Value, path: &str) -> Option<&'a Value> {
    let mut current = value;
    for segment in path.split('.') {
        match current {
            Value::Mapping(m) => current = m.get(Value::String(segment.into()))?,
            _ => return None,
        }
    }
    Some(current)
}

#[allow(dead_code)]
fn ensure_mapping(v: &Value) -> Option<&Mapping> {
    match v {
        Value::Mapping(m) => Some(m),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fleet_default_renders_canonical_values() {
        let p = Passaporte::fleet_default();
        let v = p.render();
        assert!(v.enabled);
        assert!(v.cloudflared.expose);
        assert_eq!(v.cloudflared.hostname, "auth.quero.cloud");
        assert!(v.keda.enabled);
        assert_eq!(v.keda.cooldown_period, 600);
        assert_eq!(v.keda.cold_start_budget, 15);
        assert_eq!(v.federated, vec!["google".to_string()]);
        assert_eq!(v.session_duration, 86400);
        assert!(v.compliance.authn.allow_list.unauthenticated_ingress);
    }

    #[test]
    fn render_yaml_round_trips() {
        let p = Passaporte::fleet_default();
        let s = p.render_yaml().unwrap();
        // Sanity: contains the canonical hostname + federated provider.
        assert!(s.contains("auth.quero.cloud"));
        assert!(s.contains("google"));
        assert!(s.contains("unauthenticatedIngress"));
    }

    #[test]
    fn pick_extracts_nested_field() {
        let p = Passaporte::fleet_default();
        let v = p.render_value().unwrap();
        let host = pick(&v, "cloudflared.hostname").unwrap();
        assert_eq!(host, &Value::String("auth.quero.cloud".into()));
    }
}
