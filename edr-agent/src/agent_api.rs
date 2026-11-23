// src/agent_api.rs
//
// Security pieces of the agent's HTTP API: the bearer token check, the CORS
// policy, and how integration secrets are shown and stored.
//
// The token comes from EDR_AGENT_TOKEN. With no token of 32 or more characters
// the agent does not serve the API at all, unless EDR_DEV_NO_AUTH=1 is set on
// purpose for local work. Sensing and detection keep running either way.

use axum::{
    extract::{Request, State},
    http::{header, HeaderMap, HeaderValue, Method, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    Router,
};
use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::io::Write;
use std::path::Path;
use tower_http::cors::{AllowOrigin, CorsLayer};

pub const TOKEN_ENV: &str = "EDR_AGENT_TOKEN";
pub const DEV_NO_AUTH_ENV: &str = "EDR_DEV_NO_AUTH";
pub const CORS_ORIGINS_ENV: &str = "EDR_AGENT_CORS_ORIGINS";
pub const MIN_TOKEN_LENGTH: usize = 32;

/// True only for EDR_DEV_NO_AUTH=1 exactly, so "true" or "yes" do not open the API by accident.
pub fn dev_no_auth() -> bool {
    std::env::var(DEV_NO_AUTH_ENV).as_deref() == Ok("1")
}

/// The expected token, kept as its SHA-256 digest. None means dev mode, set on purpose.
#[derive(Clone)]
pub struct AgentAuth {
    digest: Option<[u8; 32]>,
}

impl AgentAuth {
    pub fn from_env() -> Result<Self, String> {
        let token = std::env::var(TOKEN_ENV).ok();
        Self::from_values(token.as_deref(), dev_no_auth())
    }

    pub fn from_values(token: Option<&str>, dev_no_auth: bool) -> Result<Self, String> {
        match token {
            Some(t) if !t.is_empty() => {
                if t.chars().count() < MIN_TOKEN_LENGTH {
                    return Err(format!(
                        "{TOKEN_ENV} is shorter than {MIN_TOKEN_LENGTH} characters. Make one with: \
                         python3 -c \"import secrets; print(secrets.token_urlsafe(32))\""
                    ));
                }
                Ok(Self {
                    digest: Some(Sha256::digest(t.as_bytes()).into()),
                })
            }
            _ if dev_no_auth => Ok(Self { digest: None }),
            _ => Err(format!(
                "{TOKEN_ENV} is not set, so the API would be open to anyone on this host. \
                 Set {TOKEN_ENV} (32+ characters), or {DEV_NO_AUTH_ENV}=1 on a dev machine"
            )),
        }
    }

    /// True in dev mode, where every request is let through.
    pub fn is_open(&self) -> bool {
        self.digest.is_none()
    }

    /// Checks `Authorization: Bearer <token>`. Both sides are hashed to 32 bytes first,
    /// so the compare never depends on the supplied length, then CRYPTO_memcmp compares
    /// them in constant time and the response time does not leak how much of a guess
    /// was right.
    pub fn allows(&self, headers: &HeaderMap) -> bool {
        let Some(expected) = &self.digest else {
            return true;
        };
        let Some(value) = headers
            .get(header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
        else {
            return false;
        };
        let (scheme, supplied) = value.split_once(' ').unwrap_or((value, ""));
        if !scheme.eq_ignore_ascii_case("bearer") || supplied.is_empty() {
            return false;
        }
        let got: [u8; 32] = Sha256::digest(supplied.as_bytes()).into();
        openssl::memcmp::eq(&got, expected)
    }
}

/// Middleware: 401 with `WWW-Authenticate: Bearer` unless the request carries the token.
pub async fn require_bearer(State(auth): State<AgentAuth>, req: Request, next: Next) -> Response {
    if auth.allows(req.headers()) {
        return next.run(req).await;
    }
    (
        StatusCode::UNAUTHORIZED,
        [(header::WWW_AUTHENTICATE, "Bearer")],
        "missing or invalid bearer token\n",
    )
        .into_response()
}

/// Joins the two halves of the API. Every route in `protected` gets the token check,
/// including nested routers, static data directories and SSE streams, so a route added
/// there later is covered without anyone remembering to add it. `public` holds only
/// /healthz and the static UI shell, which carries no data: a browser cannot attach a
/// header to a page load, so the sign-in field has to load before the token exists.
pub fn secure_router(public: Router, protected: Router, auth: AgentAuth, cors: CorsLayer) -> Router {
    public
        .merge(protected.route_layer(middleware::from_fn_with_state(auth, require_bearer)))
        .layer(cors)
}

/// Exact origins allowed to call the API from another page, from a comma list.
/// "*" is refused, because any origin would then be allowed.
pub fn parse_origins(raw: &str) -> Vec<HeaderValue> {
    let mut out = Vec::new();
    for item in raw.split(',').map(str::trim).filter(|s| !s.is_empty()) {
        if item == "*" {
            log::warn!("{CORS_ORIGINS_ENV}: \"*\" ignored, list exact origins");
            continue;
        }
        match HeaderValue::from_str(item) {
            Ok(v) => out.push(v),
            Err(_) => log::warn!("{CORS_ORIGINS_ENV}: {item:?} is not a valid origin, ignored"),
        }
    }
    out
}

/// With an empty list no origin gets CORS headers, so only the agent's own pages
/// (same origin) can read responses. Credentials are never allowed: the token is a
/// header the page sets, not a cookie the browser would send by itself.
pub fn cors_layer(origins: Vec<HeaderValue>) -> CorsLayer {
    CorsLayer::new()
        .allow_origin(AllowOrigin::list(origins))
        .allow_methods([Method::GET, Method::POST, Method::DELETE])
        .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE])
        .allow_credentials(false)
}

pub fn cors_from_env() -> CorsLayer {
    cors_layer(parse_origins(
        &std::env::var(CORS_ORIGINS_ENV).unwrap_or_default(),
    ))
}

// ---------------- integration secrets ----------------

/// "****" plus the last 4 characters for values long enough that 4 characters give
/// little away, "****" alone for shorter ones.
pub fn mask_secret(s: &str) -> String {
    let n = s.chars().count();
    if n == 0 {
        return String::new();
    }
    if n < 12 {
        return "****".into();
    }
    let tail: String = s.chars().skip(n - 4).collect();
    format!("****{tail}")
}

fn mask_value(v: &Value) -> Value {
    match v {
        Value::Null => Value::Null,
        Value::String(s) => Value::String(mask_secret(s)),
        _ => Value::String("****".into()),
    }
}

/// One stored integration as the API shows it: every value under "creds" masked.
/// The catalog does not say which fields are secret, so all of them are.
pub fn redact_integration(item: &Value) -> Value {
    let mut out = item.clone();
    if let Some(obj) = out.as_object_mut() {
        if let Some(creds) = obj.get_mut("creds") {
            let masked = match &*creds {
                Value::Object(m) => Value::Object(
                    m.iter()
                        .map(|(k, v)| (k.clone(), mask_value(v)))
                        .collect::<Map<String, Value>>(),
                ),
                other => mask_value(other),
            };
            *creds = masked;
        }
    }
    out
}

/// Body of GET /integrations.
pub fn integrations_view(stored: &HashMap<String, Value>) -> Value {
    let mut items: Vec<Value> = stored.values().map(redact_integration).collect();
    items.sort_by(|a, b| a["id"].to_string().cmp(&b["id"].to_string()));
    json!({ "items": items })
}

/// Writes a file only the owner can read (0600), through a temp file and a rename so
/// a reader never sees half a file. O_NOFOLLOW refuses a planted symlink at the temp
/// path, and the mode is set again in case the temp file already existed.
pub fn write_private_file(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
    let mut tmp_name = path.as_os_str().to_owned();
    tmp_name.push(".tmp");
    let tmp = std::path::PathBuf::from(tmp_name);
    {
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .mode(0o600)
            .custom_flags(libc::O_NOFOLLOW)
            .open(&tmp)?;
        f.set_permissions(std::fs::Permissions::from_mode(0o600))?;
        f.write_all(bytes)?;
        f.sync_all()?;
    }
    std::fs::rename(&tmp, path)
}

/// Tightens a secrets file written by an older build with the default mode.
pub fn tighten_to_owner_only(path: &Path) {
    use std::os::unix::fs::PermissionsExt;
    if let Ok(meta) = std::fs::metadata(path) {
        if meta.permissions().mode() & 0o077 != 0 {
            let _ = std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::response::sse::{Event, Sse};
    use axum::routing::get;
    use std::convert::Infallible;
    use tower_http::services::ServeDir;

    const TOKEN: &str = "test-token-0123456789-abcdefghijklmnop";

    fn auth() -> AgentAuth {
        AgentAuth::from_values(Some(TOKEN), false).unwrap()
    }

    // Same layout as main.rs: a public half and a protected half that has a plain
    // route, an SSE stream, a nested router and a static directory.
    fn app(cors: CorsLayer) -> Router {
        let dir = std::env::temp_dir().join(format!("agent_api_static_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("graph_live.json"), b"{\"nodes\":[]}").unwrap();

        let public = Router::new().route("/healthz", get(|| async { "{\"ok\":true}" }));
        let nested = Router::new().route("/hits", get(|| async { "[]" }));
        let protected = Router::new()
            .route("/incidents", get(|| async { "[]" }))
            .route(
                "/alerts",
                get(|| async {
                    let s = futures_util::stream::iter(vec![Ok::<_, Infallible>(
                        Event::default().data("x"),
                    )]);
                    Sse::new(s)
                }),
            )
            .nest("/playbooks", nested)
            .nest_service("/json_files", ServeDir::new(dir));
        secure_router(public, protected, auth(), cors)
    }

    async fn serve(router: Router) -> String {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move {
            axum::serve(listener, router).await.unwrap();
        });
        format!("http://{addr}")
    }

    async fn status(base: &str, path: &str, authz: Option<&str>) -> u16 {
        let mut req = reqwest::Client::new().get(format!("{base}{path}"));
        if let Some(v) = authz {
            req = req.header("Authorization", v);
        }
        req.send().await.unwrap().status().as_u16()
    }

    const PROTECTED: [&str; 4] = [
        "/incidents",
        "/alerts",
        "/playbooks/hits",
        "/json_files/graph_live.json",
    ];

    #[tokio::test]
    async fn no_token_is_401_on_every_protected_route() {
        let base = serve(app(cors_layer(vec![]))).await;
        for p in PROTECTED {
            assert_eq!(status(&base, p, None).await, 401, "{p}");
        }
        let res = reqwest::get(format!("{base}/incidents")).await.unwrap();
        assert_eq!(res.headers()["www-authenticate"], "Bearer");
    }

    #[tokio::test]
    async fn wrong_token_is_401() {
        let base = serve(app(cors_layer(vec![]))).await;
        let longer = format!("Bearer {TOKEN}x");
        let shorter = format!("Bearer {}", &TOKEN[..TOKEN.len() - 1]);
        let basic = format!("Basic {TOKEN}");
        for v in [
            "Bearer wrong-token-wrong-token-wrong-token",
            longer.as_str(),
            shorter.as_str(),
            basic.as_str(),
            TOKEN,
            "Bearer ",
            "Bearer",
        ] {
            for p in PROTECTED {
                assert_eq!(status(&base, p, Some(v)).await, 401, "{p} with {v:?}");
            }
        }
    }

    #[tokio::test]
    async fn right_token_is_200_and_sse_streams_need_it_too() {
        let base = serve(app(cors_layer(vec![]))).await;
        let good = format!("Bearer {TOKEN}");
        let lower = format!("bearer {TOKEN}");
        for p in PROTECTED {
            assert_eq!(status(&base, p, Some(&good)).await, 200, "{p}");
            assert_eq!(status(&base, p, Some(&lower)).await, 200, "{p}");
        }
        let res = reqwest::Client::new()
            .get(format!("{base}/alerts"))
            .header("Authorization", &good)
            .send()
            .await
            .unwrap();
        assert_eq!(res.headers()["content-type"], "text/event-stream");
        assert!(res.text().await.unwrap().contains("data: x"));
    }

    #[tokio::test]
    async fn healthz_is_open_and_unknown_paths_are_404() {
        let base = serve(app(cors_layer(vec![]))).await;
        assert_eq!(status(&base, "/healthz", None).await, 200);
        assert_eq!(status(&base, "/no-such-route", None).await, 404);
    }

    #[tokio::test]
    async fn dev_mode_lets_requests_through() {
        let open = AgentAuth::from_values(None, true).unwrap();
        assert!(open.is_open());
        let protected = Router::new().route("/incidents", get(|| async { "[]" }));
        let base = serve(secure_router(Router::new(), protected, open, cors_layer(vec![]))).await;
        assert_eq!(status(&base, "/incidents", None).await, 200);
    }

    #[test]
    fn token_setup_fails_closed() {
        assert!(AgentAuth::from_values(None, false).is_err());
        assert!(AgentAuth::from_values(Some(""), false).is_err());
        let short = "a".repeat(MIN_TOKEN_LENGTH - 1);
        assert!(AgentAuth::from_values(Some(&short), false).is_err());
        // A short token is refused even in dev mode, so a typo cannot weaken a real setup.
        assert!(AgentAuth::from_values(Some(&short), true).is_err());
        let ok = "a".repeat(MIN_TOKEN_LENGTH);
        assert!(!AgentAuth::from_values(Some(&ok), false).unwrap().is_open());
        // A token wins over the dev flag.
        assert!(!AgentAuth::from_values(Some(&ok), true).unwrap().is_open());
    }

    #[tokio::test]
    async fn foreign_origin_gets_no_cors_headers() {
        let allowed = "http://localhost:5173";
        let base = serve(app(cors_layer(parse_origins(allowed)))).await;
        let client = reqwest::Client::new();
        let good = format!("Bearer {TOKEN}");

        let res = client
            .get(format!("{base}/incidents"))
            .header("Origin", "http://evil.example")
            .header("Authorization", &good)
            .send()
            .await
            .unwrap();
        assert!(res.headers().get("access-control-allow-origin").is_none());

        // Preflight from the foreign origin: no allow-origin, so the browser blocks it.
        let res = client
            .request(Method::OPTIONS, format!("{base}/incidents"))
            .header("Origin", "http://evil.example")
            .header("Access-Control-Request-Method", "GET")
            .header("Access-Control-Request-Headers", "authorization")
            .send()
            .await
            .unwrap();
        assert!(res.headers().get("access-control-allow-origin").is_none());

        let res = client
            .get(format!("{base}/incidents"))
            .header("Origin", allowed)
            .header("Authorization", &good)
            .send()
            .await
            .unwrap();
        assert_eq!(res.headers()["access-control-allow-origin"], allowed);
        assert!(res.headers().get("access-control-allow-credentials").is_none());
    }

    #[tokio::test]
    async fn no_allowlist_means_same_origin_only() {
        let base = serve(app(cors_layer(parse_origins("")))).await;
        let res = reqwest::Client::new()
            .get(format!("{base}/healthz"))
            .header("Origin", "http://localhost:5173")
            .send()
            .await
            .unwrap();
        assert!(res.headers().get("access-control-allow-origin").is_none());
    }

    #[test]
    fn wildcard_and_bad_origins_are_dropped() {
        let v = parse_origins(" http://a.example ,*, ,http://b.example:8080,bad\norigin");
        assert_eq!(v, vec!["http://a.example", "http://b.example:8080"]);
    }

    #[test]
    fn integrations_response_never_contains_a_stored_secret() {
        let secret = "ghp_SuperSecretValue1234567890WXYZ";
        let okta = "00aBcDeFgHiJkLmNoPqRsTuVwXyZ";
        let mut stored = HashMap::new();
        stored.insert(
            "github_audit".to_string(),
            json!({"id": "github_audit", "name": "GitHub Org Audit",
                   "creds": {"org": "acme", "token": secret}, "status": "syncing"}),
        );
        stored.insert(
            "okta_system_log".to_string(),
            json!({"id": "okta_system_log", "name": "Okta",
                   "creds": {"org_url": "https://acme.okta.com", "api_token": okta, "n": 42}}),
        );
        stored.insert(
            "odd".to_string(),
            json!({"id": "odd", "creds": "a-single-string-secret-value"}),
        );
        let body = integrations_view(&stored).to_string();
        for s in [secret, okta, "a-single-string-secret-value", "acme.okta.com", "\"acme\""] {
            assert!(!body.contains(s), "{s} leaked in {body}");
        }
        assert!(body.contains("****WXYZ"));
        assert!(body.contains("\"org\":\"****\""));
        assert!(body.contains("\"n\":\"****\""));
        assert!(body.contains("GitHub Org Audit"), "non-secret fields stay");
    }

    #[test]
    fn mask_keeps_only_a_short_tail() {
        assert_eq!(mask_secret(""), "");
        assert_eq!(mask_secret("short"), "****");
        assert_eq!(mask_secret("elevenchars"), "****");
        assert_eq!(mask_secret("twelve-chars"), "****hars");
    }

    #[test]
    fn secrets_file_is_owner_only_even_if_it_existed() {
        use std::os::unix::fs::PermissionsExt;
        let dir = std::env::temp_dir().join(format!("agent_api_perm_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("integrations.json");
        std::fs::write(&path, b"{}").unwrap();
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o644)).unwrap();
        // A leftover temp file with a wide mode must not keep that mode.
        let tmp = dir.join("integrations.json.tmp");
        std::fs::write(&tmp, b"old").unwrap();
        std::fs::set_permissions(&tmp, std::fs::Permissions::from_mode(0o666)).unwrap();

        write_private_file(&path, b"{\"k\":1}").unwrap();
        let mode = std::fs::metadata(&path).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode, 0o600);
        assert_eq!(std::fs::read(&path).unwrap(), b"{\"k\":1}");
        assert!(!tmp.exists());

        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o644)).unwrap();
        tighten_to_owner_only(&path);
        let mode = std::fs::metadata(&path).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode, 0o600);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_symlink_at_the_temp_path_is_refused() {
        let dir = std::env::temp_dir().join(format!("agent_api_link_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let target = dir.join("elsewhere");
        std::fs::write(&target, b"untouched").unwrap();
        let path = dir.join("integrations.json");
        let tmp = dir.join("integrations.json.tmp");
        let _ = std::fs::remove_file(&tmp);
        std::os::unix::fs::symlink(&target, &tmp).unwrap();
        assert!(write_private_file(&path, b"secret").is_err());
        assert_eq!(std::fs::read(&target).unwrap(), b"untouched");
        let _ = std::fs::remove_dir_all(&dir);
    }
}
