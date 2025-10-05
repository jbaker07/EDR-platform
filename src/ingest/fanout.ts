import { NormalizedAlertV2 } from "../types/alerts";
import { sendToOpenSearch } from "../sinks/opensearch";
import { sendToSplunk } from "../sinks/splunk";
import { sendToSentinel } from "../sinks/sentinel";

export interface FanoutResult {
  alert: NormalizedAlertV2;
  results: Record<string, { ok: boolean; id?: string; error?: string }>;
}

function cloneAlert(alert: NormalizedAlertV2): NormalizedAlertV2 {
  return {
    ...alert,
    subject: { ...alert.subject },
    modalities: alert.modalities ? JSON.parse(JSON.stringify(alert.modalities)) : undefined,
    ttp_evidence: alert.ttp_evidence ? [...alert.ttp_evidence] : [],
    multistage: alert.multistage ? JSON.parse(JSON.stringify(alert.multistage)) : [],
    observations: alert.observations ? [...alert.observations] : [],
    evidence_edges: alert.evidence_edges ? [...alert.evidence_edges] : [],
    sinks: { ...(alert.sinks ?? {}) }
  };
}

export async function fanoutNormalizedAlert(alert: NormalizedAlertV2): Promise<FanoutResult> {
  const resultAlert = cloneAlert(alert);
  const results: Record<string, { ok: boolean; id?: string; error?: string }> = {};

  const osUrl = process.env.SINK_OPENSEARCH_URL;
  if (osUrl) {
    const res = await sendToOpenSearch(alert, {
      url: osUrl,
      index: process.env.SINK_OPENSEARCH_INDEX,
      username: process.env.SINK_OPENSEARCH_USER,
      password: process.env.SINK_OPENSEARCH_PASS
    });
    results.opensearch = res;
    resultAlert.sinks = resultAlert.sinks ?? {};
    resultAlert.sinks.opensearch = res.ok;
  }

  const splunkUrl = process.env.SINK_SPLUNK_HEC_URL;
  const splunkToken = process.env.SINK_SPLUNK_TOKEN;
  if (splunkUrl && splunkToken) {
    const res = await sendToSplunk(alert, {
      url: splunkUrl,
      token: splunkToken,
      index: process.env.SINK_SPLUNK_INDEX
    });
    results.splunk = res;
    resultAlert.sinks = resultAlert.sinks ?? {};
    resultAlert.sinks.splunk = res.ok;
  }

  const sentinelUrl = process.env.SINK_SENTINEL_DCR_URL;
  const sentinelKey = process.env.SINK_SENTINEL_KEY;
  if (sentinelUrl && sentinelKey) {
    const res = await sendToSentinel(alert, {
      url: sentinelUrl,
      key: sentinelKey,
      workspaceId: process.env.SINK_SENTINEL_WORKSPACE_ID
    });
    results.sentinel = res;
    resultAlert.sinks = resultAlert.sinks ?? {};
    resultAlert.sinks.sentinel = res.ok;
  }

  return { alert: resultAlert, results };
}
