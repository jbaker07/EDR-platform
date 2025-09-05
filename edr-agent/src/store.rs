use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result};
use rusqlite::{params, Connection};

/// Trait so you can swap backends later (e.g., LMDB).
pub trait EventStore {
    fn put_event(&self, event_json: &str) -> Result<i64>;
    fn put_alert(&self, alert_json: &str) -> Result<i64>;
    fn record_feedback(&self, alert_rowid: i64, label: Option<i32>, note: Option<&str>) -> Result<()>;
    fn get_recent_alerts(&self, limit: usize) -> Result<Vec<String>>;
    fn cleanup_ttl(&self) -> Result<usize>;
    fn compact(&self) -> Result<()>;
    fn mark_cold_start(&self) -> Result<()>;
}

/// Default store = SQLite (enable `lmdb` feature if you add LMDB later)
pub type DefaultStore = SqliteStore;

pub struct SqliteStore {
    conn: Connection,
    ttl_seconds: u64,
}

impl SqliteStore {
    pub fn open(path: &Path, ttl_seconds: u64) -> Result<Self> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).ok();
        }
        let conn = Connection::open(path)
            .with_context(|| format!("opening sqlite at {}", path.display()))?;

        // Pragmas: WAL for concurrency; durability reasonably strong
        conn.pragma_update(None, "journal_mode", &"WAL")?;
        conn.pragma_update(None, "synchronous", &"NORMAL")?;
        conn.pragma_update(None, "temp_store", &"MEMORY")?;
        conn.pragma_update(None, "mmap_size", &((1usize << 26) as i64))?; // 64MB

        // Tables
        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS events (
              id INTEGER PRIMARY KEY,
              ts INTEGER NOT NULL,
              body TEXT NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_events_ts ON events(ts);

            CREATE TABLE IF NOT EXISTS alerts (
              id INTEGER PRIMARY KEY,
              ts INTEGER NOT NULL,
              body TEXT NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_alerts_ts ON alerts(ts);

            CREATE TABLE IF NOT EXISTS feedback (
              id INTEGER PRIMARY KEY,
              ts INTEGER NOT NULL,
              alert_id INTEGER NOT NULL,
              label INTEGER,           -- 1=malicious, 0=benign, NULL=unknown
              note TEXT,
              FOREIGN KEY(alert_id) REFERENCES alerts(id) ON DELETE CASCADE
            );

            CREATE TABLE IF NOT EXISTS meta (
              k TEXT PRIMARY KEY,
              v TEXT
            );
            "#,
        )?;

        Ok(Self { conn, ttl_seconds })
    }

    #[inline]
    fn now_secs() -> i64 {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64
    }
}

impl EventStore for SqliteStore {
    fn put_event(&self, event_json: &str) -> Result<i64> {
        let ts = Self::now_secs();
        self.conn.execute("INSERT INTO events(ts, body) VALUES (?,?)", params![ts, event_json])?;
        Ok(self.conn.last_insert_rowid())
    }

    fn put_alert(&self, alert_json: &str) -> Result<i64> {
        let ts = Self::now_secs();
        self.conn.execute("INSERT INTO alerts(ts, body) VALUES (?,?)", params![ts, alert_json])?;
        Ok(self.conn.last_insert_rowid())
    }

    fn record_feedback(&self, alert_rowid: i64, label: Option<i32>, note: Option<&str>) -> Result<()> {
        let ts = Self::now_secs();
        self.conn.execute(
            "INSERT INTO feedback(ts, alert_id, label, note) VALUES (?,?,?,?)",
            params![ts, alert_rowid, label, note.unwrap_or_default()],
        )?;
        Ok(())
    }

    fn get_recent_alerts(&self, limit: usize) -> Result<Vec<String>> {
        let mut stmt = self.conn.prepare("SELECT body FROM alerts ORDER BY ts DESC LIMIT ?")?;
        let rows = stmt
            .query_map(params![limit as i64], |row| row.get::<_, String>(0))?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(rows)
    }

    fn cleanup_ttl(&self) -> Result<usize> {
        if self.ttl_seconds == 0 {
            return Ok(0);
        }
        let cutoff = Self::now_secs() - (self.ttl_seconds as i64);
        let ev = self.conn.execute("DELETE FROM events WHERE ts < ?", params![cutoff])?;
        let al = self.conn.execute("DELETE FROM alerts WHERE ts < ?", params![cutoff])?;
        Ok(ev + al)
    }

    fn compact(&self) -> Result<()> {
        self.conn.execute_batch("PRAGMA optimize; VACUUM;")?;
        Ok(())
    }

    fn mark_cold_start(&self) -> Result<()> {
        let ts = Self::now_secs().to_string();
        self.conn.execute(
            "INSERT INTO meta(k, v) VALUES('cold_start_ts', ?) ON CONFLICT(k) DO UPDATE SET v=excluded.v",
            params![ts],
        )?;
        Ok(())
    }
}

#[cfg(feature = "lmdb")]
pub mod lmdb_backend {
    // Optional LMDB backend scaffold (fill in if/when you enable).
    // Keep it behind a feature to avoid pulling LMDB into default builds.
    // use heed / lmdb-rs here.
}
