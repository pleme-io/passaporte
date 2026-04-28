# passaporte — repo-level agent instructions

> Brazilian-Portuguese for "passport." This repo implements
> **passaporte**, the typed identity primitive of saguão.

## Frame

- [`pleme-io/theory/SAGUAO.md`](https://github.com/pleme-io/theory/blob/main/SAGUAO.md) §III.1
- `blackmatter-pleme/skills/saguao/SKILL.md`

## What this repo owns

- The typed config form `(defpassaporte …)` (Rust struct + TataraDomain derive)
- The Helm chart `lareira-passaporte` (succeeds `lareira-authentik`)
- JWT verification helpers consumed by crachá, vigia, varanda
- The fleet IdP's pleme-io defaults (KEDA scale-to-zero, observability wiring, Cloudflare Tunnel ingress)

## What this repo does NOT own

- **Authentik upstream** — `goauthentik/authentik` is consumed verbatim. Don't fork.
- **Authorization** — that's crachá. passaporte authenticates; crachá authorizes.
- **Per-cluster enforcement** — that's vigia. passaporte issues tokens; vigia validates them at the ingress.
- **Google's IdP** — Google does the actual user authentication; Authentik consumes Google as a federated source. The user model lives in Authentik, populated on first sign-in.

## Conventions

- The Lisp form is the source of truth; Helm values are rendered from it. Do not hand-edit Helm values — edit the form.
- The Rust crate is library-only (substrate `rust-library` shape). No binaries.
- JWT helpers wrap kenshou; do not re-implement OIDC primitives here.
- The `lareira-passaporte` chart's `cloudflared.expose: true` always sets `cloudflared.hostname: auth.quero.cloud` — never anything else; this is a fleet primitive.

## Pillar 1 reminder

Rust + tatara-lisp + Nix + YAML. **No shell.**
