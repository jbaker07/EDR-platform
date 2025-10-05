function normalizeUrl(url) {
  if (!url) return '';
  return url.endsWith('/') ? url.slice(0, -1) : url;
}

function createConnector(config, logger) {
  const url = normalizeUrl(config.url);
  const token = config.token;
  if (!url) {
    throw new Error('splunk_hec connector requires url');
  }
  if (!token) {
    throw new Error('splunk_hec connector requires token (use env substitution)');
  }
  const endpoint = `${url}/services/collector/event`;

  async function publish(incident) {
    const payload = {
      time: Math.floor(Date.parse(incident.observed_at || Date.now()) / 1000),
      host: incident.entities?.host || 'unknown',
      source: 'edr-correlation',
      sourcetype: '_json',
      event: incident
    };

    const res = await fetch(endpoint, {
      method: 'POST',
      headers: {
        'content-type': 'application/json',
        Authorization: `Splunk ${token}`
      },
      body: JSON.stringify(payload)
    });

    if (!res.ok) {
      const text = await res.text().catch(() => res.statusText);
      throw new Error(`Splunk HEC failed: ${res.status} ${text}`);
    }
  }

  return {
    type: 'splunk_hec',
    async publish(incident) {
      await publish(incident);
    },
    async health() {
      try {
        const res = await fetch(`${endpoint}?health=1`, {
          headers: { Authorization: `Splunk ${token}` }
        });
        return res.ok;
      } catch (err) {
        logger?.warn?.({ err }, 'Splunk health check failed');
        return false;
      }
    }
  };
}

module.exports = {
  createConnector
};
