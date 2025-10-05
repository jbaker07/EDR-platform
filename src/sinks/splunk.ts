import { NormalizedAlertV2 } from "../types/alerts";

export interface SplunkConfig {
  url: string;
  token: string;
  index?: string;
}

function flattenAlert(alert: NormalizedAlertV2): Record<string, unknown> {
  const out: Record<string, unknown> = {
    correlation_group_id: alert.correlation_group_id,
    sequence_id: alert.sequence_id,
    stage_index: alert.stage_index,
    campaign_id: alert.campaign_id,
    summary: alert.summary,
    explain_snippet: alert.explain_snippet,
    observed_at: alert.observed_at,
    risk: alert.scoring.risk,
    confidence: alert.scoring.confidence,
    corroborations: alert.scoring.corroborations,
    novelty: alert.scoring.novelty,
    deep_link: alert.deep_link
  };

  if (alert.subject.hostname) out["subject.hostname"] = alert.subject.hostname;
  if (alert.subject.endpoint_id) out["subject.endpoint_id"] = alert.subject.endpoint_id;
  if (alert.subject.ip) out["subject.ip"] = Array.isArray(alert.subject.ip) ? alert.subject.ip.join(",") : alert.subject.ip;
  if (alert.subject.user) out["subject.user"] = alert.subject.user;

  alert.observations.forEach((obs, index) => {
    const prefix = `obs${index}`;
    out[`${prefix}.vendor`] = obs.vendor;
    if (obs.product) out[`${prefix}.product`] = obs.product;
    out[`${prefix}.event_id`] = obs.event_id;
    if (obs.source_index) out[`${prefix}.source_index`] = obs.source_index;
    out[`${prefix}.observed_at`] = obs.observed_at;
    if (obs.severity !== undefined) out[`${prefix}.severity`] = obs.severity;
    if (obs.confidence !== undefined) out[`${prefix}.confidence`] = obs.confidence;
  });

  alert.evidence_edges?.forEach((edge, index) => {
    const prefix = `edge${index}`;
    out[`${prefix}.kind`] = edge.kind;
    out[`${prefix}.src`] = edge.src;
    out[`${prefix}.dst`] = edge.dst;
    if (edge.at) out[`${prefix}.at`] = edge.at;
    if (edge.vendor_source) out[`${prefix}.vendor_source`] = edge.vendor_source;
  });

  const modalities = alert.modalities ?? {};
  Object.entries(modalities).forEach(([modality, entries]) => {
    (entries ?? []).forEach((entry: Record<string, unknown>, idx: number) => {
      Object.entries(entry).forEach(([key, value]) => {
        if (value === undefined) return;
        out[`mod.${modality}${idx}.${key}`] = value;
      });
    });
  });

  alert.multistage.forEach((stage, index) => {
    const prefix = `stage${index}`;
    out[`${prefix}.name`] = stage.stage;
    out[`${prefix}.score`] = stage.score;
    out[`${prefix}.edges_supported`] = stage.edges_supported;
  });

  return out;
}

export async function sendToSplunk(
  alert: NormalizedAlertV2,
  config: SplunkConfig
): Promise<{ ok: boolean; id?: string; error?: string }> {
  const payload = {
    time: Math.floor(Date.parse(alert.observed_at) / 1000),
    host: alert.subject.hostname ?? alert.subject.endpoint_id ?? "unknown",
    index: config.index,
    sourcetype: "edr:correlated:v2",
    event: flattenAlert(alert)
  };

  const response = await fetch(config.url, {
    method: "POST",
    headers: {
      "content-type": "application/json",
      Authorization: `Splunk ${config.token}`
    },
    body: JSON.stringify(payload)
  });

  if (!response.ok) {
    const text = await response.text().catch(() => response.statusText);
    return { ok: false, error: `Splunk HEC failed: ${response.status} ${text}` };
  }
  return { ok: true, id: alert.correlation_group_id };
}
