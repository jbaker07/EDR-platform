const dgram = require('dgram');

const SEV_MAP = {
  low: 3,
  medium: 7,
  high: 10
};

function escape(value) {
  return String(value ?? '')
    .replace(/\\/g, '\\\\')
    .replace(/=/g, '\\=')
    .replace(/\|/g, '\\|');
}

function severityToNumber(severity = 'low') {
  return SEV_MAP[severity] ?? 3;
}

function formatCEF(incident) {
  const sev = severityToNumber(incident.severity);
  const header = [
    'CEF:0',
    'EDR',
    'Correlation',
    '1.0',
    escape(incident.incident_id),
    escape(incident.summary || 'Correlated incident'),
    sev
  ].join('|');

  const ext = [
    `rt=${escape(incident.observed_at)}`,
    `cs1Label=CorrelationId cs1=${escape(incident.correlation_group_id)}`,
    `cs2Label=Verdict cs2=${escape(incident.verdict)}`,
    `cs3Label=Techniques cs3=${escape((incident.linkage?.techniques || []).join(','))}`,
    `cat=${escape(incident.severity)}`,
    `msg=${escape(incident.summary)}`
  ];

  return `${header}|${ext.join(' ')}`;
}

function createConnector(config) {
  const host = config.host || '127.0.0.1';
  const port = Number(config.port || 514);
  const socket = dgram.createSocket('udp4');

  socket.unref?.();

  async function publish(incident) {
    const message = Buffer.from(formatCEF(incident));
    await new Promise((resolve, reject) => {
      socket.send(message, port, host, (err) => {
        if (err) reject(err);
        else resolve();
      });
    });
  }

  return {
    type: 'syslog_cef',
    publish,
    async health() {
      return true; // UDP fire-and-forget
    }
  };
}

module.exports = {
  createConnector
};
