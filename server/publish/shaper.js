const path = require('path');
const Ajv = require('ajv');
const addFormats = require('ajv-formats');

const schemaPath = path.join(__dirname, '..', 'schema', 'correlated_incident.json');
// eslint-disable-next-line import/no-dynamic-require, global-require
const schema = require(schemaPath);

const ajv = new Ajv({
  allErrors: true,
  strict: false,
  removeAdditional: 'failing'
});
addFormats(ajv);
const validate = ajv.compile(schema);

const MITRE_RE = /^T\d{4}(?:\.\d{3})?$/i;

function coerceString(value, fallback = '') {
  if (value == null) return fallback;
  if (typeof value === 'string') return value;
  if (typeof value === 'number' || typeof value === 'boolean') return String(value);
  return fallback;
}

function verdictFor(severity) {
  switch (severity) {
    case 'high':
      return 'contain';
    case 'medium':
      return 'investigate';
    default:
      return 'monitor';
  }
}

function unique(items) {
  return Array.from(new Set(items.filter(Boolean)));
}

function deriveCorrelationGroup(record, host, user, exe) {
  const base = [host || 'unknown-host', user || 'unknown-user', exe || 'unknown-exe'];
  return base.map((part) => coerceString(part, 'n/a').toLowerCase()).join('::');
}

function detectorsToScores(detectors = []) {
  if (!Array.isArray(detectors)) return [];
  return detectors
    .map((d) => ({
      kind: coerceString(d.kind, 'unknown'),
      score: Number(d.score ?? 0)
    }))
    .filter((d) => Number.isFinite(d.score));
}

function normalizeEvidence(findings = []) {
  if (!Array.isArray(findings)) return [];
  return findings
    .map((f) => ({
      source: coerceString(f.source, 'detector'),
      score: Number(f.score ?? 0),
      label: f.label != null ? coerceString(f.label) : null
    }))
    .filter((f) => Number.isFinite(f.score));
}

function toCanonical(record = {}) {
  const ts = Number(record.ts ?? record.time ?? record.timestamp ?? Date.now() / 1000);
  const observedAt = Number.isFinite(ts) ? new Date(ts * 1000) : new Date();

  const event = record.event || {};
  const host = coerceString(event.host ?? record.host ?? record.endpoint_id);
  const user = coerceString(event.user ?? record.user);
  const exe = coerceString(event.exe ?? event.binary_path ?? record.exe ?? record.process);
  const cmd = coerceString(event.cmdline ?? event.command_line ?? record.cmd);

  const severity = coerceString(record.severity, 'low').toLowerCase();
  const tags = Array.isArray(record.tags) ? record.tags.map((t) => coerceString(t)) : [];
  const mitre = tags.filter((t) => MITRE_RE.test(t)).map((t) => t.toUpperCase());

  const detectors = detectorsToScores(record.detectors);
  const evidence = normalizeEvidence(record.findings);

  const summary = Array.isArray(record.rationale) && record.rationale.length
    ? record.rationale.join(' ')
    : `${severity.toUpperCase()} activity for ${exe || 'process'} on ${host || 'host'}`;

  const incidentId = coerceString(record.id, `${Date.now()}`);
  const correlationId = coerceString(
    record.correlation_group_id,
    deriveCorrelationGroup(record, host, user, exe)
  );

  const risk = Number(record.risk ?? record.scores?.risk ?? 0);
  const support = Number(record.support ?? record.scores?.support ?? detectors.reduce((acc, d) => acc + d.score, 0));
  const rgcn = Number.isFinite(record.rgcn_score) ? Number(record.rgcn_score) : null;

  const linkage = {
    alerts: unique([incidentId]),
    hosts: unique([host]),
    users: unique([user]),
    executables: unique([exe]),
    techniques: unique(mitre),
    tags: unique(tags)
  };

  const canonical = {
    incident_id: incidentId,
    correlation_group_id: correlationId,
    observed_at: observedAt.toISOString(),
    observed_until: null,
    severity,
    verdict: verdictFor(severity),
    summary,
    tags: unique(tags),
    tactics: unique(mitre.map((t) => t.split('.')[0])),
    scores: {
      risk: Number.isFinite(risk) ? risk : 0,
      support: Number.isFinite(support) ? support : 0,
      confidence: null,
      rgcn,
      detectors
    },
    entities: {
      host,
      user,
      process: {
        exe,
        cmd: cmd || null,
        pid: record.pid ?? null
      }
    },
    evidence,
    timeline: Array.isArray(record.timeline) ? record.timeline : [],
    linkage,
    metadata: {
      source: record.source || 'decision-engine',
      raw: record
    }
  };

  return canonical;
}

function shapeAndValidate(record) {
  const canonical = toCanonical(record);
  if (!validate(canonical)) {
    const error = new Error('Invalid correlated incident payload');
    error.details = validate.errors;
    error.payload = canonical;
    throw error;
  }
  return canonical;
}

module.exports = {
  shapeAndValidate,
  toCanonical,
  schema,
  validate
};
