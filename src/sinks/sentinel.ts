import crypto from "node:crypto";
import { NormalizedAlertV2 } from "../types/alerts";

export interface SentinelConfig {
  url: string;
  key: string;
  workspaceId?: string;
}

function workspaceFromHost(url: URL): string | undefined {
  const hostParts = url.hostname.split(".");
  return hostParts.length ? hostParts[0] : undefined;
}

function toSentinelPayload(alert: NormalizedAlertV2): Record<string, unknown> {
  const subjectIp = Array.isArray(alert.subject.ip)
    ? alert.subject.ip.join(",")
    : alert.subject.ip;

  const payload: Record<string, unknown> = {
    Cgrp_s: alert.correlation_group_id,
    SequenceId_s: alert.sequence_id ?? null,
    StageIndex_i: alert.stage_index ?? null,
    CampaignId_s: alert.campaign_id ?? null,
    ObservedAt_t: alert.observed_at,
    Host_s: alert.subject.hostname ?? null,
    EndpointId_s: alert.subject.endpoint_id ?? null,
    Ip_s: subjectIp ?? null,
    User_s: alert.subject.user ?? null,
    Risk_d: alert.scoring.risk,
    Confidence_d: alert.scoring.confidence,
    Corroborations_d: alert.scoring.corroborations,
    Novelty_d: alert.scoring.novelty ?? null,
    Explain_s: alert.explain_snippet,
    DeepLink_s: alert.deep_link,
    TTPs_s: JSON.stringify(alert.ttp_evidence ?? []),
    Edges_s: JSON.stringify(alert.evidence_edges ?? []),
    Observations_s: JSON.stringify(alert.observations ?? [])
  };

  Object.keys(payload).forEach((key) => {
    if (payload[key] === undefined || payload[key] === null) {
      delete payload[key];
    }
  });

  return payload;
}

function buildSignature(key: string, contentLength: number, contentType: string, date: string, resource: string): string {
  const stringToSign = `POST\n${contentLength}\n${contentType}\nx-ms-date:${date}\n${resource}`;
  const decodedKey = Buffer.from(key, "base64");
  const hmac = crypto.createHmac("sha256", decodedKey);
  hmac.update(stringToSign, "utf8");
  return hmac.digest("base64");
}

export async function sendToSentinel(
  alert: NormalizedAlertV2,
  config: SentinelConfig
): Promise<{ ok: boolean; id?: string; error?: string }> {
  const payload = toSentinelPayload(alert);
  const body = JSON.stringify(payload);
  const url = new URL(config.url);
  const date = new Date().toUTCString();
  const resource = url.pathname + (url.search || "");
  const workspace = config.workspaceId ?? workspaceFromHost(url);

  if (!workspace) {
    return { ok: false, error: "Sentinel workspace id not provided or inferable" };
  }

  const signature = buildSignature(config.key, Buffer.byteLength(body, "utf8"), "application/json", date, resource);
  const authorization = `SharedKey ${workspace}:${signature}`;

  const response = await fetch(config.url, {
    method: "POST",
    headers: {
      "content-type": "application/json",
      "x-ms-date": date,
      "authorization": authorization
    },
    body
  });

  if (!response.ok) {
    const text = await response.text().catch(() => response.statusText);
    return { ok: false, error: `Sentinel ingest failed: ${response.status} ${text}` };
  }

  return { ok: true, id: alert.correlation_group_id };
}
