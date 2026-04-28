;; fleet.lisp — canonical passaporte declaration for the pleme-io
;; homelab fleet. One per fleet; renders to lareira-passaporte
;; HelmRelease values; applied to whichever cluster currently hosts
;; the saguão control plane (today: rio).

(defpassaporte
  :idp        authentik
  :host       auth.quero.cloud

  ;; Federated social IdPs. Google is the family-facing primary;
  ;; add more as needed (github for tech-savvy members, etc.).
  :federated  [google]

  ;; OIDC scopes offered to relying parties (vigia, varanda, any
  ;; future caixa Servico that needs a session).
  :scopes     [openid email profile groups]

  ;; Default session lifetime; per-app overrides live in each
  ;; service's chart values via compliance.authn.oidc.sessionDuration.
  :session-duration 24h

  ;; KEDA HTTP scale-to-zero — Authentik server scales 0→1 on
  ;; inbound auth request. Worker (background tasks) stays at 1.
  :keda
  {:enabled              true
   :cooldown-period      600s     ; keep server warm 10 min after last request
   :cold-start-budget    15s})    ; alert if cold start exceeds budget
