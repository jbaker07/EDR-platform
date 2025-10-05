const express = require('express');
const pino = require('pino');
const { getBus, shapeAndValidate } = require('./publish');

const PORT = Number(process.env.PUBLISH_PORT || process.env.PORT || 8787);

const logger = pino({ name: 'edr-publish-service' });
const app = express();
const bus = getBus({ logger });

app.use(express.json({ limit: '1mb' }));

app.use((req, res, next) => {
  req.log = logger.child({ reqId: `${Date.now()}-${Math.random().toString(16).slice(2, 8)}` });
  req.log.info({ method: req.method, url: req.originalUrl }, 'incoming request');
  res.on('finish', () => {
    req.log.info({ status: res.statusCode }, 'response sent');
  });
  next();
});

app.get('/healthz', (req, res) => {
  res.json({ ok: true, connectors: bus.connectors.size });
});

app.get('/api/connectors/status', async (req, res) => {
  try {
    const status = await bus.getStatus();
    res.json({ connectors: status });
  } catch (err) {
    req.log.error({ err }, 'failed to get connector status');
    res.status(500).json({ error: 'status_failed', message: err.message });
  }
});

app.post('/api/connectors/reload', (req, res) => {
  try {
    bus.reload();
    res.status(204).end();
  } catch (err) {
    req.log.error({ err }, 'failed to reload connectors');
    res.status(500).json({ error: 'reload_failed', message: err.message });
  }
});

app.post('/api/incidents', async (req, res) => {
  const payload = req.body;
  if (!payload || typeof payload !== 'object') {
    res.status(400).json({ error: 'invalid_body' });
    return;
  }
  try {
    const canonical = await bus.publish(payload);
    res.status(202).json({
      status: 'queued',
      incident_id: canonical.incident_id,
      correlation_group_id: canonical.correlation_group_id
    });
  } catch (err) {
    req.log.warn({ err, details: err.details }, 'incident rejected');
    res.status(400).json({
      error: 'validation_failed',
      message: err.message,
      details: err.details ?? []
    });
  }
});

// Optional helper to inspect shaping without queueing
app.post('/api/incidents/shape', (req, res) => {
  try {
    const canonical = shapeAndValidate(req.body);
    res.json(canonical);
  } catch (err) {
    res.status(400).json({ error: 'validation_failed', message: err.message, details: err.details ?? [] });
  }
});

app.post('/api/incidents/canonical', async (req, res) => {
  try {
    const canonical = shapeAndValidate(req.body);
    await bus.publishCanonical(canonical);
    res.status(202).json({ status: 'queued', incident_id: canonical.incident_id });
  } catch (err) {
    res.status(400).json({ error: 'canonical_failed', message: err.message, details: err.details ?? [] });
  }
});

app.listen(PORT, () => {
  logger.info({ port: PORT }, 'publish service listening');
});
