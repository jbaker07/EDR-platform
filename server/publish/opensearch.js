const DEFAULT_TEMPLATE_NAME = 'edr-correlated-template';
const DEFAULT_PATTERN = 'edr-correlated-*';

function toISOString(value) {
  if (!value) return new Date().toISOString();
  const d = value instanceof Date ? value : new Date(value);
  if (Number.isNaN(d.getTime())) return new Date().toISOString();
  return d.toISOString();
}

function dailyIndex(base) {
  const today = new Date();
  const yyyy = today.getUTCFullYear();
  const mm = String(today.getUTCMonth() + 1).padStart(2, '0');
  const dd = String(today.getUTCDate()).padStart(2, '0');
  if (base.endsWith('-*')) {
    return `${base.slice(0, -2)}-${yyyy}.${mm}.${dd}`;
  }
  return `${base}-${yyyy}.${mm}.${dd}`;
}

async function ensureTemplate(url, logger, templateName = DEFAULT_TEMPLATE_NAME, pattern = DEFAULT_PATTERN) {
  const res = await fetch(`${url}/_index_template/${encodeURIComponent(templateName)}`);
  if (res.status === 404) {
    const body = {
      index_patterns: [pattern],
      template: {
        settings: {
          number_of_shards: 1,
          number_of_replicas: 1
        },
        mappings: {
          dynamic: true,
          properties: {
            '@timestamp': { type: 'date' },
            observed_at: { type: 'date' },
            observed_until: { type: 'date' },
            severity: { type: 'keyword' },
            verdict: { type: 'keyword' },
            tags: { type: 'keyword' },
            tactics: { type: 'keyword' },
            summary: { type: 'text' },
            scores: {
              properties: {
                risk: { type: 'float' },
                support: { type: 'float' },
                confidence: { type: 'float' },
                rgcn: { type: 'float' },
                detectors: {
                  type: 'nested',
                  properties: {
                    kind: { type: 'keyword' },
                    score: { type: 'float' }
                  }
                }
              }
            },
            entities: {
              properties: {
                host: { type: 'keyword' },
                user: { type: 'keyword' },
                process: {
                  properties: {
                    exe: { type: 'keyword' },
                    cmd: { type: 'text' },
                    pid: { type: 'long' }
                  }
                }
              }
            },
            linkage: {
              properties: {
                alerts: { type: 'keyword' },
                hosts: { type: 'keyword' },
                users: { type: 'keyword' },
                executables: { type: 'keyword' },
                techniques: { type: 'keyword' },
                tags: { type: 'keyword' }
              }
            }
          }
        }
      }
    };

    const putRes = await fetch(`${url}/_index_template/${encodeURIComponent(templateName)}`, {
      method: 'PUT',
      headers: { 'content-type': 'application/json' },
      body: JSON.stringify(body)
    });
    if (!putRes.ok) {
      const text = await putRes.text().catch(() => putRes.statusText);
      throw new Error(`OpenSearch template install failed: ${putRes.status} ${text}`);
    }
    logger?.info?.({ template: templateName, pattern }, 'installed OpenSearch correlated template');
  }
}

function createConnector(config, logger) {
  const { url, index = 'edr-correlated' } = config;
  if (!url) {
    throw new Error('opensearch connector requires url');
  }
  let templateReady = false;

  async function init() {
    if (!templateReady) {
      await ensureTemplate(url, logger);
      templateReady = true;
    }
  }

  async function publish(incident) {
    await init();
    const targetIndex = dailyIndex(index);
    const id = incident.correlation_group_id;
    const doc = {
      ...incident,
      '@timestamp': incident.observed_at,
      ingested_at: toISOString(Date.now())
    };

    const action = { update: { _index: targetIndex, _id: id, retry_on_conflict: 3 } };
    const update = { doc, doc_as_upsert: true };
    const payload = `${JSON.stringify(action)}\n${JSON.stringify(update)}\n`;

    const res = await fetch(`${url}/_bulk`, {
      method: 'POST',
      headers: { 'content-type': 'application/x-ndjson' },
      body: payload
    });
    if (!res.ok) {
      const text = await res.text().catch(() => res.statusText);
      throw new Error(`OpenSearch bulk failed: ${res.status} ${text}`);
    }
    const body = await res.json();
    if (body.errors) {
      const failure = body.items?.find((item) => item.update && item.update.error)?.update?.error;
      throw new Error(`OpenSearch bulk reported errors: ${JSON.stringify(failure)}`);
    }
  }

  return {
    type: 'opensearch',
    publish,
    async health() {
      try {
        await init();
        const res = await fetch(`${url}`);
        return res.ok;
      } catch (err) {
        logger?.warn?.({ err }, 'OpenSearch health check failed');
        return false;
      }
    }
  };
}

module.exports = {
  createConnector
};
