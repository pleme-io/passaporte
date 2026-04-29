// passaporte — typed identity primitive for the saguão fleet.
//
// Provides:
//   1. The typed `Passaporte` config struct (renders to lareira-passaporte
//      Helm values).
//   2. A `render` impl producing serde_yaml::Value matching the chart's
//      values.yaml schema.
//
// JWT verification helpers live in vigia (the data-plane consumer);
// keeping them out of this crate avoids forcing a JWKS dependency on
// every passaporte authoring use site.

#![allow(clippy::module_name_repetitions)]

pub mod config;
pub mod render;

pub use config::{IdpBackend, KedaConfig, Passaporte, SocialProvider};
pub use render::PassaporteValues;

/// Register `(defpassaporte …)` with the global tatara-lisp dispatcher.
#[cfg(feature = "tatara-lisp")]
pub fn register_lisp_surfaces() {
    tatara_lisp::domain::register::<Passaporte>();
}
