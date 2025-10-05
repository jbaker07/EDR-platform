import crypto from "node:crypto";
import { NormalizedAlertV2, StageHypothesis, VendorObservation, EvidenceEdge } from "../types/alerts";
import { isDuplicate, noveltyScore, sha1 } from "../lib/dedupe";

const WINDOW_MINUTES = Number(process.env.CORR_WINDOW_MIN ?? 15);
const WINDOW_MS = WINDOW_MINUTES * 60 * 1000;
const SEQUENCE_PAD = 5;
const DEFAULT_DEEP_LINK = process.env.UI_BASE_URL ?? "https://localhost/ui/graph.html";

interface DerivedContext {
  endpointId?: string;
  hostname?: string;
  ip?: string;
  user?: string;
  processName?: string;
  processPid?: number;
  processPpid?: number;
  processCmd?: string;
  processSha?: string;
  dstIp?: string;
  srcIp?: string;
  domain?: string;
  tactic?: string;
  technique?: string;
  subtechnique?: string;
  action?: string;
  severity?: number;
  confidence?: number;
}

interface ObservationRecord {
  observation: VendorObservation;
  timestamp: number;
  derived: DerivedContext;
}

interface CorrelationCluster {
  id: string;
  subjectKey: string;
  subject: NormalizedAlertV2["subject"];
  observations: ObservationRecord[];
  evidenceEdges: EvidenceEdge[];
  modalities: NonNullable<NormalizedAlertV2["modalities"]>;
  ttpEvidence: NormalizedAlertV2["ttp_evidence"];
  multistage: StageHypothesis[];
  createdAt: number;
  lastObserved: number;
  sequence: number;
}

const clustersBySubject = new Map<string, CorrelationCluster[]>();

function toDate(value: string | undefined): number {
  if (!value) return Date.now();
  const ts = Date.parse(value);
  return Number.isFinite(ts) ? ts : Date.now();
}

function pick<T>(...candidates: Array<T | undefined>): T | undefined {
  for (const candidate of candidates) {
    if (candidate !== undefined && candidate !== null && candidate !== "") {
      return candidate;
    }
  }
  return undefined;
}

function asArray<T>(value: T | T[] | undefined): T[] | undefined {
  if (value === undefined) return undefined;
  return Array.isArray(value) ? value : [value];
}

function normalizeString(value: unknown): string | undefined {
  if (value === undefined || value === null) return undefined;
  const str = String(value).trim();
  return str.length ? str : undefined;
}

function extractNumber(value: unknown): number | undefined {
  if (value === undefined || value === null) return undefined;
  const num = Number(value);
  return Number.isFinite(num) ? num : undefined;
}

function extractContext(observation: VendorObservation): DerivedContext {
  const f = observation.fields || {} as Record<string, any>;
  const endpointId = normalizeString(
    pick(
      f.endpoint_id,
      f.endpointId,
      f["host.id"],
      f["agent.id"],
      f["observer.id"],
      f["device.id"]
    )
  );
  const hostname = normalizeString(
    pick(
      f.hostname,
      f["host"],
      f["host.name"],
      f["observer.name"],
      f["device.hostname"],
      f["agent.hostname"],
      f["computer"],
      f["computer_name"]
    )
  );
  const ip = normalizeString(
    pick(
      f.ip,
      f["host.ip"],
      f["source.ip"],
      f["client.ip"],
      Array.isArray(f["host.ip"]) ? f["host.ip"][0] : undefined
    )
  );
  const user = normalizeString(
    pick(
      f.user,
      f["user.name"],
      f["username"],
      f["subject.user"],
      f["actor.user"],
      observation.fields?.user?.name
    )
  );
  const process = f.process ?? observation.fields?.process ?? {};
  const processName = normalizeString(
    pick(
      process.name,
      process.exe,
      process.executable,
      f["process.name"],
      f["rule.process"],
      observation.fields?.event?.process
    )
  );
  const processPid = extractNumber(pick(process.pid, f["process.pid"], process.process_id));
  const processPpid = extractNumber(pick(process.ppid, f["process.ppid"], process.parent_pid));
  const processCmd = normalizeString(pick(process.cmdline, f["process.command_line"], f.cmd, f.command));
  const processSha = normalizeString(
    pick(
      process.sha256,
      process.hash?.sha256,
      f["process.hash.sha256"],
      f["file.hash.sha256"],
      f.sha256
    )
  );
  const dstIp = normalizeString(
    pick(
      f.dest_ip,
      f.destination_ip,
      f["destination.ip"],
      f["network.destination.ip"],
      f["dst_ip"],
      f["network.direction" === "egress" ? "dest.ip" : undefined]
    )
  );
  const srcIp = normalizeString(pick(f.src_ip, f.source_ip, f["source.ip"], f["client.ip"]));
  const domain = normalizeString(pick(f.domain, f["dns.question.name"], f["http.host"], observation.fields?.dest_domain));

  const tactic = normalizeString(
    pick(
      f.tactic,
      f["mitre.tactic"],
      observation.fields?.alert?.tactic,
      f["threat.technique.tactic"],
      f["rule.tactic"]
    )
  );
  const technique = normalizeString(
    pick(
      f.technique,
      f["mitre.technique"],
      f["rule.technique"],
      f["alert.signature"],
      f["technique.id"],
      f["threat.technique.name"],
      f["threat.technique.id"]
    )
  );
  const subtechnique = normalizeString(
    pick(
      f.subtechnique,
      f["mitre.subtechnique"],
      f["threat.technique.subtechnique.id"],
      f["technique.subtechnique"]
    )
  );
  const action = normalizeString(pick(f.action, f.event_action, process.action));
  const severity = extractNumber(pick(observation.severity, f.severity, f["alert.severity"], f["event.severity"]));
  const confidence = extractNumber(pick(observation.confidence, f.confidence, f["alert.confidence"], f["event.confidence"]));

  return {
    endpointId,
    hostname,
    ip,
    user,
    processName,
    processPid,
    processPpid,
    processCmd,
    processSha,
    dstIp,
    srcIp,
    domain,
    tactic,
    technique,
    subtechnique,
    action,
    severity,
    confidence
  };
}

function subjectKey(ctx: DerivedContext, observation: VendorObservation): string {
  return (
    ctx.endpointId ||
    ctx.hostname ||
    asArray(ctx.ip)?.[0] ||
    observation.vendor ||
    "unknown"
  ).toLowerCase();
}

function buildSubject(ctx: DerivedContext): NormalizedAlertV2["subject"] {
  const ipArray = asArray(ctx.ip);
  return {
    endpoint_id: ctx.endpointId,
    hostname: ctx.hostname,
    ip: ipArray && (ipArray.length === 1 ? ipArray[0] : ipArray),
    user: ctx.user,
    process: ctx.processName
      ? {
          name: ctx.processName,
          pid: ctx.processPid,
          ppid: ctx.processPpid,
          cmdline: ctx.processCmd,
          sha256: ctx.processSha
        }
      : undefined,
    file: undefined
  };
}

function initialiseModalities(): NonNullable<NormalizedAlertV2["modalities"]> {
  return { network: [], process: [], identity: [], file: [] };
}

function pushModality(modalities: NonNullable<NormalizedAlertV2["modalities"]>, ctx: DerivedContext, observation: VendorObservation) {
  if (ctx.dstIp || ctx.domain) {
    modalities.network?.push({
      direction: ctx.srcIp && ctx.dstIp ? "egress" : undefined,
      proto: normalizeString(observation.fields?.protocol || observation.fields?.proto),
      src_ip: ctx.srcIp,
      dst_ip: ctx.dstIp,
      domain: ctx.domain,
      url: normalizeString(observation.fields?.url),
      bytes_out: extractNumber(observation.fields?.bytes_out),
      bytes_in: extractNumber(observation.fields?.bytes_in)
    });
  }
  if (ctx.processName) {
    modalities.process?.push({
      name: ctx.processName,
      pid: ctx.processPid,
      ppid: ctx.processPpid,
      cmdline: ctx.processCmd,
      exe: normalizeString(observation.fields?.exe ?? observation.fields?.process_path),
      sha256: ctx.processSha,
      action: ctx.action as any
    });
  }
  if (ctx.user) {
    modalities.identity?.push({
      user: ctx.user,
      action: ctx.action as any,
      provider: normalizeString(observation.fields?.identity_provider),
      result: normalizeString(observation.fields?.result)
    });
  }
  const filePath = normalizeString(
    pick(
      observation.fields?.["file.path"],
      observation.fields?.file_path,
      observation.fields?.file,
      observation.fields?.target_path
    )
  );
  if (filePath) {
    modalities.file?.push({
      path: filePath,
      sha256: ctx.processSha,
      action: ctx.action as any
    });
  }
}

function buildEdge(ctx: DerivedContext, cluster: CorrelationCluster): EvidenceEdge[] {
  const edges: EvidenceEdge[] = [];
  const hostId = ctx.hostname || ctx.endpointId ? `host:${ctx.hostname ?? ctx.endpointId}` : undefined;
  if (ctx.processName && hostId && ctx.processPid) {
    const nodeId = `proc:${ctx.processPid}@${cluster.subject.hostname ?? cluster.subject.endpoint_id ?? "host"}`;
    if (ctx.dstIp) {
      edges.push({
        kind: "network_to",
        src: nodeId,
        dst: `ip:${ctx.dstIp}`,
        at: new Date(cluster.lastObserved).toISOString(),
        vendor_source: cluster.observations.at(-1)?.observation.vendor
      });
    }
    if (ctx.domain) {
      edges.push({
        kind: "dns_query",
        src: nodeId,
        dst: `domain:${ctx.domain}`,
        at: new Date(cluster.lastObserved).toISOString(),
        vendor_source: cluster.observations.at(-1)?.observation.vendor
      });
    }
  }
  return edges;
}

const TACTIC_TO_STAGE: Record<string, StageHypothesis["stage"]> = {
  TA0001: "initial_access",
  TA0002: "execution",
  TA0003: "persistence",
  TA0004: "privilege_escalation",
  TA0005: "defense_evasion",
  TA0006: "credential_access",
  TA0007: "discovery",
  TA0008: "lateral_movement",
  TA0009: "collection",
  TA0010: "exfiltration",
  TA0011: "command_and_control"
};

function tacticToStage(tactic?: string): StageHypothesis["stage"] {
  if (!tactic) return "execution";
  const upper = tactic.toUpperCase();
  if (upper in TACTIC_TO_STAGE) return TACTIC_TO_STAGE[upper];
  return (tactic.replace(/\s+/g, "_").toLowerCase() as StageHypothesis["stage"]);
}

function aggregateTtpEvidence(records: ObservationRecord[]): NormalizedAlertV2["ttp_evidence"] {
  const map = new Map<string, { tactic?: string; technique?: string; subtechnique?: string; sources: Set<string> }>();
  for (const rec of records) {
    const { derived, observation } = rec;
    const key = [derived.tactic, derived.technique, derived.subtechnique].filter(Boolean).join(":");
    if (!key) continue;
    if (!map.has(key)) {
      map.set(key, {
        tactic: derived.tactic,
        technique: derived.technique,
        subtechnique: derived.subtechnique,
        sources: new Set()
      });
    }
    map.get(key)!.sources.add(observation.vendor);
  }
  return Array.from(map.values()).map((entry) => ({
    tactic: entry.tactic,
    technique: entry.technique,
    subtechnique: entry.subtechnique,
    sources: Array.from(entry.sources)
  }));
}

function buildStages(records: ObservationRecord[], edges: EvidenceEdge[]): StageHypothesis[] {
  const stageMap = new Map<string, StageHypothesis>();
  for (const rec of records) {
    const stage = tacticToStage(rec.derived.tactic);
    const key = stage;
    const existing = stageMap.get(key);
    if (!existing) {
      stageMap.set(key, {
        stage,
        start_at: new Date(rec.timestamp).toISOString(),
        end_at: new Date(rec.timestamp).toISOString(),
        ttps: [],
        edges_supported: 0,
        score: 0
      });
    } else {
      if (!existing.start_at || rec.timestamp < Date.parse(existing.start_at)) {
        existing.start_at = new Date(rec.timestamp).toISOString();
      }
      if (!existing.end_at || rec.timestamp > Date.parse(existing.end_at)) {
        existing.end_at = new Date(rec.timestamp).toISOString();
      }
    }
    const stageRef = stageMap.get(key)!;
    stageRef.ttps.push({
      tactic: rec.derived.tactic,
      technique: rec.derived.technique,
      subtechnique: rec.derived.subtechnique,
      sources: [rec.observation.vendor]
    });
  }

  for (const edge of edges) {
    if (!edge.vendor_source) continue;
    const stage = tacticToStage(edge.kind.includes("network") ? "command_and_control" : edge.kind.includes("file") ? "defense_evasion" : undefined);
    const ref = stageMap.get(stage) ?? {
      stage,
      start_at: edge.at,
      end_at: edge.at,
      ttps: [],
      edges_supported: 0,
      score: 0
    };
    ref.edges_supported += 1;
    stageMap.set(stage, ref);
  }

  for (const [, value] of stageMap) {
    const signalCount = value.ttps.length;
    value.score = Math.min(100, 40 + value.edges_supported * 10 + signalCount * 5);
  }

  return Array.from(stageMap.values()).sort((a, b) => {
    const aTime = a.start_at ? Date.parse(a.start_at) : 0;
    const bTime = b.start_at ? Date.parse(b.start_at) : 0;
    return aTime - bTime;
  });
}

function chooseSummary(cluster: CorrelationCluster): string {
  const latest = cluster.observations.at(-1);
  const vendors = new Set(cluster.observations.map((o) => o.observation.vendor));
  const ttp = cluster.ttpEvidence.at(-1);
  const tactic = ttp?.tactic ?? cluster.multistage.at(-1)?.stage;
  const technique = ttp?.technique ?? cluster.multistage.at(-1)?.ttps?.at(0)?.technique;
  const action = latest?.derived.action ?? latest?.derived.processName;
  return [
    vendors.size ? Array.from(vendors).join("+") : undefined,
    tactic ? tactic.replace(/_/g, " ") : undefined,
    technique,
    action ? `via ${action}` : undefined
  ]
    .filter(Boolean)
    .join(" · ") || "Correlated activity";
}

function explainSnippet(cluster: CorrelationCluster): string {
  const vendors = Array.from(new Set(cluster.observations.map((o) => o.observation.vendor)));
  const stage = cluster.multistage.at(-1);
  const stageText = stage ? `${stage.stage} (${stage.score.toFixed(0)})` : "multi-stage";
  return `${vendors.join("+")} corroborate ${stageText} activity on ${cluster.subject.hostname ?? cluster.subject.endpoint_id ?? "host"}`;
}

function computeRisk(cluster: CorrelationCluster): number {
  const severityMean = cluster.observations.reduce((sum, obs) => sum + (obs.derived.severity ?? 2), 0) /
    Math.max(1, cluster.observations.length);
  const vendorFactor = new Set(cluster.observations.map((o) => o.observation.vendor)).size;
  const stageFactor = cluster.multistage.length;
  const raw = 30 + severityMean * 10 + vendorFactor * 8 + stageFactor * 6;
  return Math.min(100, Math.max(0, raw));
}

function computeConfidence(cluster: CorrelationCluster): number {
  const confidences = cluster.observations
    .map((o) => o.derived.confidence ?? (typeof o.observation.confidence === "number" ? o.observation.confidence : undefined))
    .filter((v): v is number => typeof v === "number" && !Number.isNaN(v));
  if (!confidences.length) {
    const vendorCount = new Set(cluster.observations.map((o) => o.observation.vendor)).size;
    return Math.min(1, 0.5 + vendorCount * 0.1);
  }
  const avg = confidences.reduce((sum, v) => sum + v, 0) / confidences.length;
  return Math.min(1, Math.max(0, avg));
}

function extractFeatureHashes(cluster: CorrelationCluster): string[] {
  const features: string[] = [];
  for (const rec of cluster.observations) {
    if (rec.derived.processSha) features.push(`proc:${rec.derived.processSha}`);
    if (rec.derived.domain) features.push(`domain:${rec.derived.domain}`);
    if (rec.derived.technique) features.push(`tech:${rec.derived.technique}`);
  }
  return Array.from(new Set(features));
}

function pruneOldClusters(subject: string) {
  const list = clustersBySubject.get(subject);
  if (!list) return;
  const cutoff = Date.now() - WINDOW_MS;
  const filtered = list.filter((cluster) => cluster.lastObserved >= cutoff);
  clustersBySubject.set(subject, filtered);
}

function pickCluster(candidates: CorrelationCluster[], record: ObservationRecord): CorrelationCluster | undefined {
  const thresholdMs = Number(process.env.CORR_STAGE_THRESHOLD_MS ?? 10 * 60 * 1000);
  return candidates.find((cluster) => {
    if (record.timestamp - cluster.lastObserved > WINDOW_MS) return false;
    const lastObs = cluster.observations.at(-1);
    if (!lastObs) return false;
    const sameUser = !!record.derived.user && record.derived.user === lastObs.derived.user;
    const sameProcess = !!record.derived.processSha && record.derived.processSha === lastObs.derived.processSha;
    const sameDst = !!record.derived.dstIp && record.derived.dstIp === lastObs.derived.dstIp;
    const temporalOk = record.timestamp >= cluster.lastObserved && record.timestamp - cluster.lastObserved <= thresholdMs;
    return temporalOk && (sameUser || sameProcess || sameDst);
  });
}

function ensureCluster(subjectKey: string, record: ObservationRecord): CorrelationCluster {
  pruneOldClusters(subjectKey);
  const list = clustersBySubject.get(subjectKey) ?? [];
  const cluster = pickCluster(list, record);
  if (cluster) {
    return cluster;
  }
  const id = `cgrp-${crypto.randomUUID().slice(0, 8)}`;
  const subject = buildSubject(record.derived);
  const newCluster: CorrelationCluster = {
    id,
    subjectKey,
    subject,
    observations: [],
    evidenceEdges: [],
    modalities: initialiseModalities(),
    ttpEvidence: [],
    multistage: [],
    createdAt: record.timestamp,
    lastObserved: record.timestamp,
    sequence: 0
  };
  list.push(newCluster);
  clustersBySubject.set(subjectKey, list);
  return newCluster;
}

async function buildNormalizedAlert(cluster: CorrelationCluster): Promise<NormalizedAlertV2 | null> {
  cluster.ttpEvidence = aggregateTtpEvidence(cluster.observations);
  cluster.multistage = buildStages(cluster.observations, cluster.evidenceEdges);

  const stageIndex = Math.max(0, cluster.multistage.length - 1);
  const sequenceId = `${cluster.id}:` + String(cluster.sequence).padStart(SEQUENCE_PAD, "0");

  const dedupePayload = `${cluster.id}:${stageIndex}:` +
    cluster.observations
      .map((o) => o.observation.event_id)
      .sort()
      .join(":");
  const dedupeHash = sha1(dedupePayload);
  const dedupeKey = `corr:${cluster.id}:${stageIndex}`;

  const duplicate = await isDuplicate(dedupeKey, dedupeHash);
  if (duplicate) {
    return null;
  }

  const features = extractFeatureHashes(cluster);
  let novelty = 0;
  if (features.length) {
    const scores = await Promise.all(
      features.map((feature) => noveltyScore(cluster.subjectKey, feature))
    );
    novelty = scores.reduce((acc, val) => acc + val, 0) / scores.length;
  }

  return {
    schema_version: 2,
    correlation_group_id: cluster.id,
    sequence_id: sequenceId,
    stage_index: stageIndex,
    campaign_id: undefined,
    subject: cluster.subject,
    summary: chooseSummary(cluster),
    explain_snippet: explainSnippet(cluster),
    observed_at: new Date(cluster.createdAt).toISOString(),
    scoring: {
      risk: computeRisk(cluster),
      confidence: computeConfidence(cluster),
      corroborations: new Set(cluster.observations.map((o) => o.observation.vendor)).size,
      novelty
    },
    graph_ref: { id: cluster.id },
    evidence_edges: cluster.evidenceEdges,
    modalities: cluster.modalities,
    ttp_evidence: cluster.ttpEvidence,
    multistage: cluster.multistage,
    observations: cluster.observations.map((o) => o.observation),
    sinks: {},
    deep_link: `${DEFAULT_DEEP_LINK}?cgrp=${cluster.id}`
  };
}

async function upsertObservation(observation: VendorObservation): Promise<CorrelationCluster> {
  const derived = extractContext(observation);
  const timestamp = toDate(observation.observed_at);
  const record: ObservationRecord = { observation, timestamp, derived };
  const key = subjectKey(derived, observation);
  const cluster = ensureCluster(key, record);
  cluster.observations.push(record);
  cluster.lastObserved = Math.max(cluster.lastObserved, timestamp);
  cluster.subject = {
    ...cluster.subject,
    ...buildSubject(derived)
  };
  pushModality(cluster.modalities, derived, observation);
  const newEdges = buildEdge(derived, cluster);
  cluster.evidenceEdges.push(...newEdges);
  cluster.sequence += 1;
  return cluster;
}

export async function correlate(observation: VendorObservation): Promise<NormalizedAlertV2[]> {
  const cluster = await upsertObservation(observation);
  const alert = await buildNormalizedAlert(cluster);
  return alert ? [alert] : [];
}
