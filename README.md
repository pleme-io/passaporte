# passaporte — typed identity primitive for the saguão fleet

> Brazilian-Portuguese for "passport." The credential that proves
> who you are.

`passaporte` is the **typed identity primitive** of the saguão
fleet identity + authz + portal architecture. It is the typed Rust
wrapper around the upstream Authentik OIDC provider, federated to
Google as a social IdP. **One per fleet**, hosted at
`auth.quero.cloud`.

**Canonical architecture:** [`pleme-io/theory/SAGUAO.md`](https://github.com/pleme-io/theory/blob/main/SAGUAO.md) §III.1.

**Status:** scaffold. **Phase 1 / Phase 5** of the saguão migration.
Phase 1 (the deployed Authentik wrapped by the existing
`lareira-authentik` chart) is **live but suspended on rio**;
this repo packages the same chart as `lareira-passaporte` along
with the typed Rust authoring surface.

## What it is

| Surface | What |
|---|---|
| `passaporte` (Rust crate) | Typed config form — `(defpassaporte …)` via `#[derive(TataraDomain)]`. Renders to `lareira-passaporte` Helm values. JWT verification helpers built on `kenshou`. |
| `charts/lareira-passaporte` | Helm chart — wraps upstream `goauthentik/authentik` with pleme-io defaults (KEDA HTTP scale-to-zero, always-on outpost + worker, Cloudflare Tunnel ingress, ServiceMonitor + PrometheusRule, vector log shipping). Supersedes `pleme-io/helmworks/charts/lareira-authentik`. |

## Why a typed Rust wrapper over Authentik (vs replacing Authentik)

Authentik is a mature OIDC provider with ~10 years of work behind
it. Replacing it with a from-scratch Rust IdP is a multi-month
security-critical project with no compounding payoff. Wrapping it
with a typed config form gives:

- **Lisp authoring surface** — `(defpassaporte …)` instead of
  clicking through Authentik's admin UI.
- **JWT verification helpers** that crachá, vigia, and varanda all
  consume — built on the shared `kenshou` OIDC library.
- **Optionality** — the IdP backend can later be swapped to a Rust
  implementation without breaking the consumer surface.

## Usage (target shape)

```clojure
(defpassaporte
  :idp           authentik
  :host          auth.quero.cloud
  :federated     [google]
  :scopes        [openid email profile groups]
  :session-duration 24h
  :keda
  {:enabled true
   :cooldown-period 600s
   :cold-start-budget 15s})
```

Renders to `lareira-passaporte` HelmRelease values. Apply via Flux
to the control-plane cluster (today: rio).

## Repo layout

```
passaporte/
├── README.md                       (this file)
├── CLAUDE.md                       (per-repo agent instructions)
├── flake.nix                       (substrate rust-library)
├── Cargo.toml
├── Cargo.lock                      (TBD)
├── Cargo.nix                       (TBD)
├── .envrc / .gitignore
├── src/
│   └── lib.rs                      (typed config + JWT helpers)
├── charts/
│   └── lareira-passaporte/         (Helm chart, succeeds lareira-authentik)
└── examples/
    └── fleet.lisp                  (canonical example passaporte declaration)
```

## Migration from lareira-authentik

The existing `pleme-io/helmworks/charts/lareira-authentik` chart is
the Phase-1 implementation of passaporte. Migration:

1. Move the chart to `pleme-io/passaporte/charts/lareira-passaporte`.
2. Rename the chart name in `Chart.yaml`.
3. Update consumer references (today: `clusters/rio/infrastructure/authentik/release.yaml`).
4. Update the `helmworks/charts/pleme-lib/templates/_compliance_authn.tpl`
   helper's default outpost host (currently `authentik.identity.svc.cluster.local`)
   to point at the new namespace if it changes.

## Cross-references

- [`SAGUAO.md` §III.1](https://github.com/pleme-io/theory/blob/main/SAGUAO.md)
- `blackmatter-pleme/skills/saguao/SKILL.md`
- Companion repos: `pleme-io/cracha` (authz), `pleme-io/vigia` (data plane), `pleme-io/varanda` (PWA)

## License

MIT.
