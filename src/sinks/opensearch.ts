import { NormalizedAlertV2 } from "../types/alerts";

export interface OpenSearchConfig {
  url: string;
  index?: string;
  username?: string;
  password?: string;
}

function dailyIndexName(basePattern: string): string {
  const base = basePattern.endsWith("-*") ? basePattern.slice(0, -2) : basePattern;
  const now = new Date();
  const yyyy = now.getUTCFullYear();
  const mm = String(now.getUTCMonth() + 1).padStart(2, "0");
  const dd = String(now.getUTCDate()).padStart(2, "0");
  return `${base}-${yyyy}.${mm}.${dd}`;
}

function buildDocument(alert: NormalizedAlertV2): Record<string, unknown> {
  return {
    ...alert,
    "@timestamp": alert.observed_at,
    observed_at: alert.observed_at,
    host: {
      name: alert.subject.hostname,
      ip: alert.subject.ip
    },
    user: {
      name: alert.subject.user
    },
    event: {
      kind: "alert",
      severity: Math.max(1, Math.min(3, Math.ceil(alert.scoring.risk / 34)))
    }
  };
}

async function request(path: string, init: RequestInit, config: OpenSearchConfig) {
  const headers = new Headers(init.headers ?? {});
  headers.set("content-type", "application/json");
  if (config.username && config.password) {
    const credentials = Buffer.from(`${config.username}:${config.password}`).toString("base64");
    headers.set("authorization", `Basic ${credentials}`);
  }
  return fetch(`${config.url}${path}`, { ...init, headers });
}

export async function sendToOpenSearch(
  alert: NormalizedAlertV2,
  config: OpenSearchConfig
): Promise<{ ok: boolean; id?: string; error?: string }> {
  const indexPattern = config.index ?? "edr-correlated-v2-*";
  const index = dailyIndexName(indexPattern);
  const doc = buildDocument(alert);
  const action = { update: { _index: index, _id: alert.correlation_group_id, retry_on_conflict: 3 } };
  const payload = `${JSON.stringify(action)}\n${JSON.stringify({ doc, doc_as_upsert: true })}\n`;

  const response = await request("/_bulk", { method: "POST", body: payload, headers: { "content-type": "application/x-ndjson" } }, config);
  if (!response.ok) {
    const text = await response.text().catch(() => response.statusText);
    return { ok: false, error: `OpenSearch bulk failed: ${response.status} ${text}` };
  }
  const body = await response.json();
  if (body.errors) {
    const failure = body.items?.find((item: any) => item.update?.error)?.update?.error;
    return { ok: false, error: `OpenSearch bulk error: ${JSON.stringify(failure)}` };
  }
  return { ok: true, id: alert.correlation_group_id };
}
