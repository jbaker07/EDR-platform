// src/redact.rs
// Secrets in command lines are masked before a command line is stored, logged, sent or
// matched. The sensors see every argv, so `mysql -pS3cret` or
// `curl -H "Authorization: Bearer ..."` would otherwise land in kv_telemetry.jsonl, the
// incident list and the agent log.
//
// What is masked (the flag or key name stays, so a detection can still see that a
// password was passed):
//   flags whose name holds password, passwd, passphrase, pass, pwd, secret, token,
//     api key, access key, private key, client secret or credential, as --flag=value or
//     --flag value (a value that starts with "-" is the next flag and is kept)
//   key=value and "key": "value" pairs with the same words in the key (PWD and OLDPWD,
//     which hold directories, are kept)
//   Authorization and Proxy-Authorization header values, Bearer tokens, and the
//     X-Api-Key, X-Auth-Token, Private-Token, X-Amz-Security-Token and Cookie headers
//   tool specific short flags: mysql -pX, sshpass -p X, redis-cli -a X, openssl -k X,
//     sqlcmd -P X, mongosh -p X, curl -u user:X, smbclient -U user%X
//   AWS access key ids (AKIA..., ASIA...), and AWS secret keys after their key name or
//     as a 40 character token of mixed case and digits
//   PEM private key blocks, from the BEGIN line to the END line
//   user:password@ in URLs
//   GitHub, Slack and Stripe live tokens, and JSON Web Tokens
//
// Configuration:
//   EDR_REDACT=0               turn masking off (logged as a warning, for debugging only)
//   EDR_REDACT_FLAGS=a,b       more flag names whose value is masked, such as --db-auth
//   EDR_REDACT_KEYS=a,b        more key names for key=value pairs

use std::borrow::Cow;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::OnceLock;

use regex::{Captures, Regex};

pub const MASK: &str = "[REDACTED]";

static REDACTED: AtomicU64 = AtomicU64::new(0);

/// Command lines that had at least one secret masked since start.
pub fn redactions_total() -> u64 {
    REDACTED.load(Ordering::Relaxed)
}

pub struct Redactor {
    enabled: bool,
    /// The agent's own redactor, which a reloaded policy can turn on.
    global: bool,
    rules: Vec<(Regex, Rule)>,
    extra_flags: Vec<String>,
    extra_keys: Vec<String>,
}

#[derive(Clone, Copy)]
enum Rule {
    PrivateKey,
    UrlCreds,
    AuthHeader,
    SecretHeader,
    Bearer,
    Flag,
    JsonPair,
    KeyValue,
    AwsSecretAfterName,
    AwsKeyId,
    KnownToken,
}

const WORDS: &str = r"password|passwd|passphrase|pass|pwd|secret|token|api[_-]?key|apikey|access[_-]?key|private[_-]?key|client[_-]?secret|auth[_-]?key|credentials?";

impl Redactor {
    pub fn from_env() -> Self {
        let off = std::env::var("EDR_REDACT")
            .map(|v| matches!(v.trim().to_ascii_lowercase().as_str(), "0" | "off" | "false" | "no"))
            .unwrap_or(false);
        let list = |name: &str| -> Vec<String> {
            std::env::var(name)
                .unwrap_or_default()
                .split(',')
                .map(|s| s.trim().trim_start_matches('-').to_ascii_lowercase())
                .filter(|s| !s.is_empty())
                .collect()
        };
        Self::new(!off, list("EDR_REDACT_FLAGS"), list("EDR_REDACT_KEYS"))
    }

    pub fn new(enabled: bool, extra_flags: Vec<String>, extra_keys: Vec<String>) -> Self {
        let r = |p: &str| Regex::new(p).expect("redaction pattern");
        let rules = vec![
            (
                r(r"(?s)-----BEGIN ([A-Z0-9]+ )*PRIVATE KEY-----.*?(-----END ([A-Z0-9]+ )*PRIVATE KEY-----|$)"),
                Rule::PrivateKey,
            ),
            (r(r"(?i)\b([a-z][a-z0-9+.-]*://)([^\s:/@]+):([^\s/@]+)@"), Rule::UrlCreds),
            (
                r(r#"(?i)\b((?:proxy-)?authorization\s*[:=]\s*)(?:(bearer|basic|token|digest|negotiate|ntlm|aws4-hmac-sha256)\s+)?([^\s'",]+)"#),
                Rule::AuthHeader,
            ),
            (
                r(r#"(?i)\b(x-api-key|x-auth-token|private-token|x-amz-security-token|x-access-token|set-cookie|cookie)(\s*:\s*)([^\s'"]+)"#),
                Rule::SecretHeader,
            ),
            (r(r"(?i)\b(bearer)\s+([A-Za-z0-9._~+/=-]{8,})"), Rule::Bearer),
            (
                // --name=value, or --name value where the value does not start with "-".
                r(&format!(
                    r#"(?i)(^|\s)(--?[a-z0-9_.-]*(?:{WORDS})[a-z0-9_.-]*)(?:(=)('[^']*'|"[^"]*"|\S*)|(\s+)('[^']*'|"[^"]*"|[^\s-]\S*))"#
                )),
                Rule::Flag,
            ),
            (
                r(&format!(r#"(?i)("[a-z0-9_.-]*(?:{WORDS})[a-z0-9_.-]*"\s*:\s*)"[^"]*""#)),
                Rule::JsonPair,
            ),
            (
                r(&format!(r#"(?i)\b([a-z0-9_.-]*(?:{WORDS})[a-z0-9_.-]*)=([^\s&'";]+)"#)),
                Rule::KeyValue,
            ),
            (
                r(r#"(?i)\b(aws_secret_access_key|secret_access_key|aws_secret_key|secretaccesskey)(\s+|\s*[=:]\s*)("?)([A-Za-z0-9/+=]{40})"#),
                Rule::AwsSecretAfterName,
            ),
            (
                r(r"\b((?:AKIA|ASIA|ABIA|ACCA|AGPA|AIDA|AIPA|ANPA|ANVA|AROA|APKA|ASCA)[A-Z0-9]{16})\b"),
                Rule::AwsKeyId,
            ),
            (
                r(r"\b(gh[pousr]_[A-Za-z0-9]{36,}|github_pat_[A-Za-z0-9_]{40,}|xox[abprs]-[A-Za-z0-9-]{10,}|sk_live_[A-Za-z0-9]{16,}|rk_live_[A-Za-z0-9]{16,}|eyJ[A-Za-z0-9_-]{8,}\.eyJ[A-Za-z0-9_-]{8,}\.[A-Za-z0-9_-]{8,})"),
                Rule::KnownToken,
            ),
        ];
        Self {
            enabled,
            global: false,
            rules,
            extra_flags,
            extra_keys,
        }
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    /// The command line with every secret masked. Masking twice changes nothing.
    pub fn redact<'a>(&self, s: &'a str) -> Cow<'a, str> {
        let on = self.enabled || (self.global && POLICY_REQUIRES.load(std::sync::atomic::Ordering::Relaxed));
        if !on || s.is_empty() {
            return Cow::Borrowed(s);
        }
        let mut out: Cow<'a, str> = Cow::Borrowed(s);
        for (re, rule) in &self.rules {
            if !re.is_match(&out) {
                continue;
            }
            let next = re
                .replace_all(&out, |c: &Captures| replace(*rule, c))
                .into_owned();
            out = Cow::Owned(next);
        }
        if let Some(t) = tool_flags(&out) {
            out = Cow::Owned(t);
        }
        if let Some(t) = self.extra(&out) {
            out = Cow::Owned(t);
        }
        if let Some(t) = mask_bare_aws_secrets(&out) {
            out = Cow::Owned(t);
        }
        // Owned means something was masked.
        if out == s {
            return Cow::Borrowed(s);
        }
        out
    }

    /// EDR_REDACT_FLAGS and EDR_REDACT_KEYS.
    fn extra(&self, s: &str) -> Option<String> {
        if self.extra_flags.is_empty() && self.extra_keys.is_empty() {
            return None;
        }
        let mut cur = s.to_string();
        for f in &self.extra_flags {
            let re = Regex::new(&format!(r"(^|\s)(--?{})(=|\s+)(\S+)", regex::escape(f))).ok()?;
            cur = re
                .replace_all(&cur, |c: &Captures| {
                    if c[3].trim().is_empty() && c[4].starts_with('-') {
                        c[0].to_string()
                    } else {
                        format!("{}{}{}{MASK}", &c[1], &c[2], &c[3])
                    }
                })
                .into_owned();
        }
        for k in &self.extra_keys {
            let re = Regex::new(&format!(r"(?i)\b({})=(\S+)", regex::escape(k))).ok()?;
            cur = re.replace_all(&cur, |c: &Captures| format!("{}={MASK}", &c[1])).into_owned();
        }
        (cur != s).then_some(cur)
    }
}

fn replace(rule: Rule, c: &Captures) -> String {
    match rule {
        Rule::PrivateKey => format!("{MASK} PRIVATE KEY"),
        Rule::UrlCreds => format!("{}{}:{MASK}@", &c[1], &c[2]),
        Rule::AuthHeader => match c.get(2) {
            Some(scheme) => format!("{}{} {MASK}", &c[1], scheme.as_str()),
            None if c[3].eq_ignore_ascii_case(MASK) => c[0].to_string(),
            None => format!("{}{MASK}", &c[1]),
        },
        Rule::SecretHeader => format!("{}{}{MASK}", &c[1], &c[2]),
        Rule::Bearer => format!("{} {MASK}", &c[1]),
        Rule::Flag => {
            let name = c[2].trim_start_matches('-').to_ascii_lowercase();
            let (sep, value) = match (c.get(3), c.get(4), c.get(5), c.get(6)) {
                (Some(eq), Some(v), _, _) => (eq.as_str(), v.as_str()),
                (_, _, Some(ws), Some(v)) => (ws.as_str(), v.as_str()),
                _ => return c[0].to_string(),
            };
            // --password-file /path names where the secret is, not the secret.
            let names_a_place = ["file", "path", "dir", "env", "stdin", "fd", "prompt", "cmd", "command"]
                .iter()
                .any(|end| name.ends_with(end));
            if !is_secret_name(&name) || names_a_place || value.is_empty() || value == MASK {
                c[0].to_string()
            } else {
                format!("{}{}{sep}{MASK}", &c[1], &c[2])
            }
        }
        Rule::JsonPair => {
            let key = c[1].trim_start_matches('"');
            let key = key.split('"').next().unwrap_or("").to_ascii_lowercase();
            if is_secret_name(&key) {
                format!("{}\"{MASK}\"", &c[1])
            } else {
                c[0].to_string()
            }
        }
        Rule::KeyValue => {
            let key = c[1].to_ascii_lowercase();
            if !is_secret_name(&key) || key == "pwd" || key == "oldpwd" || &c[2] == MASK {
                c[0].to_string()
            } else {
                format!("{}={MASK}", &c[1])
            }
        }
        Rule::AwsSecretAfterName => format!("{}{}{}{MASK}", &c[1], &c[2], &c[3]),
        Rule::AwsKeyId => format!("{}{MASK}", &c[1][..4]),
        Rule::KnownToken => MASK.to_string(),
    }
}

/// True when a flag or key name says it holds a secret. The name is split into words at
/// "-", "_" and ".", so --password, DB_PASSWORD, aws_secret_access_key and --api-key are
/// secrets and --passive, --bypass and author are not. A name that starts with "no"
/// (--no-password) is a switch.
pub fn is_secret_name(name: &str) -> bool {
    const ONE: &[&str] = &[
        "password", "passwd", "passphrase", "pass", "pwd", "secret", "secrets", "token", "apikey",
        "credential", "credentials", "creds", "authtoken", "accesskey", "secretkey", "privatekey",
        "clientsecret", "sessionkey",
    ];
    const FIRST_OF_PAIR: &[&str] = &["api", "access", "private", "auth", "session", "client", "secret", "encryption"];
    let words: Vec<&str> = name
        .split(|c: char| c == '-' || c == '_' || c == '.')
        .filter(|w| !w.is_empty())
        .collect();
    if words.first() == Some(&"no") {
        return false;
    }
    let one = words.iter().any(|w| {
        ONE.contains(w) || w.ends_with("password") || w.ends_with("passwd") || w.ends_with("secret") || w.ends_with("token")
    });
    let pair = words
        .windows(2)
        .any(|p| FIRST_OF_PAIR.contains(&p[0]) && matches!(p[1], "key" | "secret" | "token"));
    one || pair
}

/// Short flags that carry a secret only for some tools. Each command in a pipeline or
/// a `sh -c` string is checked by the name of its first word.
fn tool_flags(s: &str) -> Option<String> {
    static RULES: OnceLock<Vec<(Regex, Regex)>> = OnceLock::new();
    let rules = RULES.get_or_init(|| {
        let r = |p: &str| Regex::new(p).expect("tool pattern");
        vec![
            // mysql -pSECRET: attached only. "-p name" is a database name.
            (
                r(r"(?:^|[\s/;|&(])(mysql|mysqldump|mysqladmin|mysqlimport|mysqlshow|mysqlcheck|mariadb|mariadb-dump)\b"),
                r(r"(\s-p)([^\s\[]\S*)"),
            ),
            (r(r"(?:^|[\s/;|&(])(sshpass)\b"), r(r"(\s-p\s*)([^\s\[]\S*)")),
            (r(r"(?:^|[\s/;|&(])(redis-cli)\b"), r(r"(\s-a\s+)([^\s\[]\S*)")),
            (r(r"(?:^|[\s/;|&(])(openssl)\b"), r(r"(\s-k\s+)([^\s\[]\S*)")),
            (r(r"(?:^|[\s/;|&(])(sqlcmd|bcp)\b"), r(r"(\s-P\s*)([^\s\[]\S*)")),
            (r(r"(?:^|[\s/;|&(])(mongo|mongosh|mongodump|mongorestore)\b"), r(r"(\s-p\s*)([^\s\[-]\S*)")),
            (r(r"(?:^|[\s/;|&(])(curl)\b"), r(r"(\s(?:-u|--user|--proxy-user)\s+[^:\s]+:)([^\s\[]\S*)")),
            (r(r"(?:^|[\s/;|&(])(smbclient|rpcclient)\b"), r(r"(\s-U\s*[^%\s]+%)([^\s\[]\S*)")),
        ]
    });
    let mut cur: Option<String> = None;
    for (tool, flag) in rules {
        let text = cur.as_deref().unwrap_or(s);
        if !tool.is_match(text) || !flag.is_match(text) {
            continue;
        }
        let next = flag
            .replace_all(text, |c: &Captures| format!("{}{MASK}", &c[1]))
            .into_owned();
        cur = Some(next);
    }
    cur.filter(|t| t != s)
}

/// A whole token of exactly 40 characters from the AWS secret key alphabet with upper
/// and lower case letters and a digit. Tokens that start with "/" (paths) and tokens
/// with more than two "/" are left alone. Hex digests are lower case only, so a SHA-1
/// is not touched.
fn mask_bare_aws_secrets(s: &str) -> Option<String> {
    let b = s.as_bytes();
    let in_set = |c: u8| c.is_ascii_alphanumeric() || c == b'/' || c == b'+';
    let delim = |c: u8| c.is_ascii_whitespace() || matches!(c, b'\'' | b'"' | b'=' | b':' | b',');
    let mut out = String::with_capacity(s.len());
    let mut last = 0;
    let mut i = 0;
    while i < b.len() {
        if !in_set(b[i]) {
            i += 1;
            continue;
        }
        let start = i;
        while i < b.len() && in_set(b[i]) {
            i += 1;
        }
        // The run is ASCII and its ends sit next to ASCII bytes or the string ends, so
        // these slices fall on character boundaries.
        let run = &s[start..i];
        let left_ok = start == 0 || delim(b[start - 1]);
        let right_ok = i == b.len() || delim(b[i]);
        let looks = run.len() == 40
            && !run.starts_with('/')
            && run.matches('/').count() <= 2
            && run.bytes().any(|c| c.is_ascii_uppercase())
            && run.bytes().any(|c| c.is_ascii_lowercase())
            && run.bytes().any(|c| c.is_ascii_digit());
        if left_ok && right_ok && looks {
            out.push_str(&s[last..start]);
            out.push_str(MASK);
            last = i;
        }
    }
    if last == 0 {
        return None;
    }
    out.push_str(&s[last..]);
    Some(out)
}

static GLOBAL: OnceLock<Redactor> = OnceLock::new();
/// Set when a signed policy that requires masking is applied while the agent runs.
/// It only ever turns masking on, so a new policy can not switch it off before a
/// restart.
static POLICY_REQUIRES: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// A new signed policy. true turns masking on for the agent's redactor now.
pub fn policy_requires(on: bool) {
    if on {
        POLICY_REQUIRES.store(true, std::sync::atomic::Ordering::Relaxed);
    }
}

/// Called once at startup, before any reader runs, with the signed policy's
/// redact_command_lines. When the policy requires masking, EDR_REDACT=0 is refused.
pub fn init(policy_requires: bool) {
    let mut r = Redactor::from_env();
    if !r.enabled {
        if policy_requires {
            eprintln!("⚠️ EDR_REDACT=0 ignored: the signed policy requires command line masking");
            r.enabled = true;
        } else {
            eprintln!("⚠️ WARNING: EDR_REDACT=0, command line secrets are stored as they are");
        }
    }
    r.global = true;
    let _ = GLOBAL.set(r);
}

fn global() -> &'static Redactor {
    GLOBAL.get_or_init(|| {
        let mut r = Redactor::from_env();
        r.global = true;
        r
    })
}

/// The agent's redactor, configured from the environment on first use.
pub fn redact(s: &str) -> Cow<'_, str> {
    // The same command line comes back for every event of a process, and the sinks mask
    // again what the sources masked, so recent answers are kept per thread. A cached
    // answer is the one the rules gave for the same input, so it is the same.
    thread_local! {
        static RECENT: std::cell::RefCell<std::collections::HashMap<String, Option<String>>> =
            std::cell::RefCell::new(std::collections::HashMap::new());
    }
    const CACHE_CAP: usize = 4096;
    const CACHE_MAX_LEN: usize = 4096;
    if s.len() <= CACHE_MAX_LEN {
        if let Some(hit) = RECENT.with(|c| c.borrow().get(s).cloned()) {
            return match hit {
                Some(masked) => {
                    REDACTED.fetch_add(1, Ordering::Relaxed);
                    Cow::Owned(masked)
                }
                None => Cow::Borrowed(s),
            };
        }
    }
    let out = global().redact(s);
    if let Cow::Owned(_) = out {
        REDACTED.fetch_add(1, Ordering::Relaxed);
    }
    if s.len() <= CACHE_MAX_LEN {
        let entry = match &out {
            Cow::Owned(o) => Some(o.clone()),
            Cow::Borrowed(_) => None,
        };
        RECENT.with(|c| {
            let mut c = c.borrow_mut();
            if c.len() >= CACHE_CAP {
                c.clear();
            }
            c.insert(s.to_string(), entry);
        });
    }
    out
}

/// Owned form, for fields that are moved into a record.
pub fn redact_owned(s: String) -> String {
    match redact(&s) {
        Cow::Borrowed(_) => s,
        Cow::Owned(o) => o,
    }
}

/// /proc/<pid>/cmdline (NUL separated argv) as one line, masked.
pub fn cmdline_from_proc(bytes: &[u8]) -> String {
    let joined = bytes
        .split(|c| *c == 0)
        .filter(|part| !part.is_empty())
        .map(|part| String::from_utf8_lossy(part).into_owned())
        .collect::<Vec<_>>()
        .join(" ");
    redact_owned(joined)
}

/// Keys whose values are command lines in the telemetry maps.
pub const CMDLINE_KEYS: &[&str] = &[
    "command_line",
    "cmdline",
    "exec_argv",
    "cmd",
    "args",
    "argv",
    "command",
    "cmd_prefix",
    "proc_cmdline",
];

/// Masks every command line value in a telemetry map. The sinks call this as a second
/// line of defense, after the sources.
pub fn redact_map(m: &mut std::collections::HashMap<String, String>) {
    for k in CMDLINE_KEYS {
        if let Some(v) = m.get_mut(*k) {
            if let Cow::Owned(o) = redact(v) {
                *v = o;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn r(s: &str) -> String {
        Redactor::new(true, vec![], vec![]).redact(s).into_owned()
    }

    // Built at run time so no test source line holds a whole fake secret.
    fn fake(prefix: &str, body: &str) -> String {
        format!("{prefix}{body}")
    }

    #[test]
    fn password_and_token_flags() {
        assert_eq!(r("app --password hunter2 --verbose"), "app --password [REDACTED] --verbose");
        assert_eq!(r("app --password=hunter2"), "app --password=[REDACTED]");
        assert_eq!(r("gh --token 'abc def'"), "gh --token [REDACTED]");
        assert_eq!(r("svc --db-password=s3 --api-key k1 --client-secret c1"), "svc --db-password=[REDACTED] --api-key [REDACTED] --client-secret [REDACTED]");
        assert_eq!(r("openssl enc -aes-256-cbc -pass pass:demo -in a"), "openssl enc -aes-256-cbc -pass [REDACTED] -in a");
        // A flag that names a file, and a word flag followed by the next flag, are kept.
        assert_eq!(r("app --password-file /run/secrets/db"), "app --password-file /run/secrets/db");
        assert_eq!(r("app --no-password --quiet"), "app --no-password --quiet");
    }

    #[test]
    fn key_value_and_json_pairs() {
        assert_eq!(r("env PASSWORD=hunter2 DB_TOKEN=t0k ./run"), "env PASSWORD=[REDACTED] DB_TOKEN=[REDACTED] ./run");
        assert_eq!(r("curl https://x/api?user=a&api_key=K9&x=1"), "curl https://x/api?user=a&api_key=[REDACTED]&x=1");
        assert_eq!(r(r#"curl -d {"user":"a","password":"p4ss"}"#), r#"curl -d {"user":"a","password":"[REDACTED]"}"#);
        // PWD is a directory, not a password.
        assert_eq!(r("env PWD=/home/a OLDPWD=/tmp"), "env PWD=/home/a OLDPWD=/tmp");
    }

    #[test]
    fn authorization_and_bearer() {
        assert_eq!(r("curl -H 'Authorization: Bearer abc.def.ghi' https://h"), "curl -H 'Authorization: Bearer [REDACTED]' https://h");
        assert_eq!(r("curl -H Authorization:Basic dXNlcjpwYXNz https://h"), "curl -H Authorization:Basic [REDACTED] https://h");
        assert_eq!(r("wget --header=X-Api-Key: k123456 u"), "wget --header=X-Api-Key: [REDACTED] u");
        assert_eq!(r("http GET h bearer abcdefgh12345"), "http GET h bearer [REDACTED]");
        assert_eq!(r("curl -b 'Cookie: session=abc123' h"), "curl -b 'Cookie: [REDACTED]' h");
    }

    #[test]
    fn aws_key_ids_and_secret_keys() {
        let id = fake("AKIA", "IOSFODNN7EXAMPLE");
        let secret = fake("wJalrXUtnFEMI/K7MDENG/", "bPxRfiCYEXAMPLEKEY");
        assert_eq!(secret.len(), 40);
        let line = format!("aws configure set aws_access_key_id {id}");
        assert_eq!(r(&line), "aws configure set aws_access_key_id AKIA[REDACTED]");
        let line = format!("aws configure set aws_secret_access_key {secret}");
        assert_eq!(r(&line), "aws configure set aws_secret_access_key [REDACTED]");
        // A bare 40 character secret with no name next to it.
        let line = format!("run {secret} now");
        assert_eq!(r(&line), "run [REDACTED] now");
        // A git SHA-1 is lower case hex and a long path starts with "/": both kept.
        let sha = "da39a3ee5e6b4b0d3255bfef95601890afd80709";
        assert_eq!(r(&format!("git show {sha}")), format!("git show {sha}"));
        let path = "/opt/Build2024/releases/Alpha9/BetaX1234";
        assert_eq!(r(&format!("ls {path}")), format!("ls {path}"));
    }

    #[test]
    fn private_keys_and_url_credentials() {
        let begin = fake("-----BEGIN OPENSSH PRIVATE", " KEY-----");
        let end = fake("-----END OPENSSH PRIVATE", " KEY-----");
        let line = format!("echo {begin} b3BlbnNzaC1rZXk {end} > k");
        assert_eq!(r(&line), "echo [REDACTED] PRIVATE KEY > k");
        // A block cut off by the argv limit is masked to the end.
        let line = format!("echo {begin} b3BlbnNzaC1rZXk");
        assert_eq!(r(&line), "echo [REDACTED] PRIVATE KEY");
        assert_eq!(r("git clone https://bob:hunter2@git.example/x.git"), "git clone https://bob:[REDACTED]@git.example/x.git");
        assert_eq!(r("psql postgres://app:pw1@db:5432/app"), "psql postgres://app:[REDACTED]@db:5432/app");
    }

    #[test]
    fn tool_specific_short_flags() {
        assert_eq!(r("mysql -uroot -pS3cret shop"), "mysql -uroot -p[REDACTED] shop");
        // "-p shop" names a database and the client prompts for the password.
        assert_eq!(r("mysql -u root -p shop"), "mysql -u root -p shop");
        assert_eq!(r("sshpass -p hunter2 ssh h"), "sshpass -p [REDACTED] ssh h");
        assert_eq!(r("redis-cli -a hunter2 ping"), "redis-cli -a [REDACTED] ping");
        assert_eq!(r("curl -u bob:hunter2 https://h"), "curl -u bob:[REDACTED] https://h");
        assert_eq!(r("smbclient //h/s -U bob%hunter2"), "smbclient //h/s -U bob%[REDACTED]");
        assert_eq!(r("/bin/sh -c mysql -pX1 db"), "/bin/sh -c mysql -p[REDACTED] db");
        // Other tools keep their -p.
        assert_eq!(r("ssh -p 2222 host"), "ssh -p 2222 host");
        assert_eq!(r("mkdir -p /a/b"), "mkdir -p /a/b");
    }

    #[test]
    fn known_token_shapes() {
        let gh = fake("ghp_", &"A1b2C3d4".repeat(5));
        assert_eq!(r(&format!("git push https://{gh}@github.com/x")), "git push https://[REDACTED]@github.com/x");
        let jwt = format!("{}.{}.{}", fake("eyJ", "hbGciOiJIUzI1NiJ9"), fake("eyJ", "zdWIiOiIxMjM0NTY3ODkwIn0"), "dozjgNryP4J3jVmNHl0w5N_XgL0n3I9PlFUP0THsR8U");
        assert_eq!(r(&format!("tool --jwt {jwt}")), "tool --jwt [REDACTED]");
    }

    #[test]
    fn plain_commands_are_untouched_and_masking_twice_changes_nothing() {
        for s in [
            "ls -la /home/alice",
            "tar -czf /tmp/a.tgz /etc",
            "unshare --mount --uts --ipc --fork sleep 2",
            "rm -rf /backup/daily",
            "dd if=/dev/zero of=/dev/null bs=1M count=20000",
        ] {
            assert_eq!(r(s), s);
        }
        let once = r("mysql -pX app --token t PASSWORD=p");
        assert_eq!(r(&once), once);
    }

    #[test]
    fn extra_names_and_the_off_switch() {
        let red = Redactor::new(true, vec!["db-auth".into()], vec!["LICENSE".into()]);
        assert_eq!(red.redact("app --db-auth xyz LICENSE=abc").into_owned(), "app --db-auth [REDACTED] LICENSE=[REDACTED]");
        let off = Redactor::new(false, vec![], vec![]);
        assert_eq!(off.redact("app --password hunter2").into_owned(), "app --password hunter2");
    }
}
