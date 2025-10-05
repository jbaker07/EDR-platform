const fs = require('fs');
const path = require('path');
const crypto = require('crypto');
const EventEmitter = require('events');
const pino = require('pino');

const { shapeAndValidate, toCanonical } = require('./shaper');
const { createConnector: createOpenSearch } = require('./opensearch');
const { createConnector: createSplunk } = require('./splunk_hec');
const { createConnector: createSyslog } = require('./syslog_cef');

const SECRET_KEYS = new Set(['token', 'password', 'secret', 'api_key', 'authorization', 'bearer']);
const DEFAULT_CONFIG_PATH = path.resolve(process.cwd(), 'config', 'connectors.json');
const BASE_BACKOFF = 1000; // 1s
const MAX_BACKOFF = 60_000; // 60s
const MAX_ATTEMPTS = 6;
const DEDUPE_LIMIT = 5000;

const factories = {
  opensearch: createOpenSearch,
  splunk_hec: createSplunk,
  syslog_cef: createSyslog
};

function delay(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

function resolveEnv(value) {
  if (typeof value === 'string') {
    const match = value.match(/^\$ENV\{([A-Z0-9_]+)\}$/i);
    if (match) {
      return process.env[match[1]] ?? '';
    }
    return value;
  }
  if (Array.isArray(value)) {
    return value.map((v) => resolveEnv(v));
  }
  if (value && typeof value === 'object') {
    const out = {};
    for (const [k, v] of Object.entries(value)) {
      out[k] = resolveEnv(v);
    }
    return out;
  }
  return value;
}

function redact(value, key) {
  if (key && SECRET_KEYS.has(key.toLowerCase())) {
    if (!value) return null;
    const len = String(value).length;
    return `***${len >= 4 ? String(value).slice(-4) : ''}`;
  }
  if (value && typeof value === 'object') {
    const out = Array.isArray(value) ? [] : {};
    for (const [k, v] of Object.entries(value)) {
      // eslint-disable-next-line no-param-reassign
      out[k] = redact(v, k);
    }
    return out;
  }
  return value;
}

function loadConfig(configPath = DEFAULT_CONFIG_PATH) {
  if (!fs.existsSync(configPath)) {
    return [];
  }
  const raw = JSON.parse(fs.readFileSync(configPath, 'utf8'));
  if (!Array.isArray(raw)) return [];
  return raw.map((entry, idx) => {
    const resolved = resolveEnv(entry);
    return {
      id: resolved.id || `${resolved.type || 'connector'}-${idx}`,
      ...resolved,
      enabled: Boolean(resolved.enabled)
    };
  });
}

class ConnectorState extends EventEmitter {
  constructor(def, logger) {
    super();
    this.def = def;
    this.logger = logger.child({ connector: def.id, type: def.type });
    const factory = factories[def.type];
    if (!factory) {
      throw new Error(`Unsupported connector type: ${def.type}`);
    }
    this.handler = factory(def, this.logger);
    this.queue = [];
    this.processing = false;
    this.lastError = null;
    this.lastSuccess = null;
    this.backoffBase = BASE_BACKOFF;
    this.health = null;
  }

  enqueue(job) {
    this.queue.push(job);
    if (!this.processing) {
      this.processQueue();
    }
  }

  async processQueue() {
    this.processing = true;
    while (this.queue.length) {
      const job = this.queue[0];
      const now = Date.now();
      if (job.notBefore && job.notBefore > now) {
        await delay(job.notBefore - now);
      }
      try {
        await this.handler.publish(job.incident);
        this.lastSuccess = new Date();
        this.lastError = null;
        this.queue.shift();
        job.attempt = 0;
        this.backoffBase = BASE_BACKOFF;
      } catch (err) {
        this.lastError = err.message || String(err);
        job.attempt += 1;
        this.logger.warn({ err, attempt: job.attempt }, 'publish failed; will retry');
        if (job.attempt >= MAX_ATTEMPTS) {
          this.logger.error({ err }, 'dropping incident after max retries');
          this.queue.shift();
        } else {
          const delayMs = Math.min(MAX_BACKOFF, this.backoffBase * 2 ** (job.attempt - 1));
          job.notBefore = Date.now() + delayMs;
          await delay(delayMs);
        }
      }
    }
    this.processing = false;
  }

  async checkHealth() {
    if (typeof this.handler.health === 'function') {
      try {
        this.health = await this.handler.health();
      } catch (err) {
        this.health = false;
        this.logger.warn({ err }, 'health check failed');
      }
    }
    return this.health;
  }

  status() {
    return {
      id: this.def.id,
      type: this.def.type,
      enabled: this.def.enabled,
      queueDepth: this.queue.length,
      lastError: this.lastError,
      lastSuccess: this.lastSuccess,
      healthy: this.health
    };
  }
}

class PublishBus {
  constructor(options = {}) {
    this.logger = options.logger || pino({ name: 'publish-bus' });
    this.configPath = options.configPath || DEFAULT_CONFIG_PATH;
    this.connectors = new Map();
    this.dedupe = new Map();
    this.dedupeOrder = [];
    this.reload();
  }

  reload() {
    const configs = loadConfig(this.configPath);
    const next = new Map();
    for (const def of configs) {
      if (!def.enabled) {
        next.set(def.id, null);
        continue;
      }
      try {
        const state = new ConnectorState(def, this.logger);
        next.set(def.id, state);
      } catch (err) {
        this.logger.error({ err, connector: def.id }, 'failed to initialise connector');
      }
    }
    this.connectors = next;
  }

  pruneDedupe() {
    while (this.dedupeOrder.length > DEDUPE_LIMIT) {
      const oldest = this.dedupeOrder.shift();
      if (!oldest) continue;
      if (typeof oldest === 'string') {
        this.dedupe.delete(oldest);
        continue;
      }
      const current = this.dedupe.get(oldest.id);
      if (current && current.hash === oldest.hash) {
        this.dedupe.delete(oldest.id);
      }
    }
  }

  shouldPublish(incident) {
    const id = incident.correlation_group_id;
    const hash = crypto
      .createHash('sha1')
      .update(JSON.stringify({
        incident_id: incident.incident_id,
        correlation_group_id: incident.correlation_group_id,
        observed_at: incident.observed_at,
        severity: incident.severity,
        verdict: incident.verdict,
        summary: incident.summary,
        scores: incident.scores,
        tags: incident.tags
      }))
      .digest('hex');
    const prev = this.dedupe.get(id);
    if (prev && prev.hash === hash) {
      return false;
    }
    const stamp = { id, hash };
    this.dedupe.set(id, { hash, updated: Date.now() });
    this.dedupeOrder.push(stamp);
    this.pruneDedupe();
    return true;
  }

  async publishCanonical(incident) {
    if (!this.shouldPublish(incident)) {
      this.logger.debug({ id: incident.correlation_group_id }, 'duplicate incident skipped');
      return;
    }
    for (const [id, state] of this.connectors.entries()) {
      if (!state) continue; // disabled connector placeholder
      state.enqueue({ incident, attempt: 0 });
      this.logger.debug({ connector: id, incident: incident.correlation_group_id }, 'queued incident');
    }
  }

  async publish(rawRecord) {
    const canonical = shapeAndValidate(rawRecord);
    await this.publishCanonical(canonical);
    return canonical;
  }

  async getStatus() {
    const configs = loadConfig(this.configPath);
    const base = configs.map((cfg) => ({
      id: cfg.id,
      type: cfg.type,
      enabled: cfg.enabled,
      config: redact(cfg)
    }));

    const results = [];
    for (const cfg of base) {
      const state = this.connectors.get(cfg.id);
      if (state) {
        await state.checkHealth();
        results.push({
          ...cfg,
          ...state.status()
        });
      } else {
        results.push({
          ...cfg,
          queueDepth: 0,
          lastError: null,
          healthy: cfg.enabled ? false : null,
          lastSuccess: null
        });
      }
    }
    return results;
  }
}

let singleton;

function getBus(options) {
  if (!singleton) {
    singleton = new PublishBus(options);
  }
  return singleton;
}

module.exports = {
  PublishBus,
  getBus,
  publishAll: (incident) => getBus().publishCanonical(incident),
  publishRaw: (record) => getBus().publish(record),
  shapeAndValidate,
  toCanonical
};
