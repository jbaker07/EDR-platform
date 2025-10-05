export interface VendorObservation {
  vendor: "suricata" | "zeek" | "locint-edr" | "sentinel" | "splunk" | string;
  product?: string;
  event_id: string;
  source_index?: string;
  observed_at: string;
  severity?: string | number;
  confidence?: number;
  fields: Record<string, any>;
  raw_ref?: {
    type: "opensearch" | "splunk" | "sentinel" | "s3" | "file" | string;
    pointer: string;
  };
}

export interface EvidenceEdge {
  kind:
    | "caused_by"
    | "parent_of"
    | "same_actor"
    | "same_asset"
    | "network_to"
    | "file_write"
    | "dns_query"
    | string;
  src: string;
  dst: string;
  at?: string;
  vendor_source?: string;
}

export interface StageHypothesis {
  stage:
    | "initial_access"
    | "execution"
    | "persistence"
    | "privilege_escalation"
    | "defense_evasion"
    | "credential_access"
    | "discovery"
    | "lateral_movement"
    | "collection"
    | "exfiltration"
    | "command_and_control"
    | string;
  start_at?: string;
  end_at?: string;
  ttps: Array<{ tactic?: string; technique?: string; subtechnique?: string; sources: string[] }>;
  edges_supported: number;
  score: number;
}

export interface NormalizedAlertV2 {
  schema_version: 2;
  correlation_group_id: string;
  sequence_id?: string;
  stage_index?: number;
  campaign_id?: string;
  subject: {
    endpoint_id?: string;
    hostname?: string;
    ip?: string | string[];
    user?: string;
    process?: { pid?: number; name?: string; ppid?: number; cmdline?: string; sha256?: string };
    file?: { path?: string; sha256?: string };
  };
  summary: string;
  explain_snippet: string;
  observed_at: string;
  scoring: {
    risk: number;
    confidence: number;
    corroborations: number;
    novelty?: number;
  };
  graph_ref?: { id: string };
  evidence_edges?: EvidenceEdge[];
  modalities?: {
    network?: Array<{
      direction?: "ingress" | "egress";
      proto?: string;
      src_ip?: string;
      dst_ip?: string;
      src_port?: number;
      dst_port?: number;
      domain?: string;
      url?: string;
      bytes_out?: number;
      bytes_in?: number;
    }>;
    process?: Array<{
      name?: string;
      pid?: number;
      ppid?: number;
      cmdline?: string;
      exe?: string;
      sha256?: string;
      action?: "spawn" | "inject" | "rename" | "delete" | "write" | "connect";
    }>;
    identity?: Array<{
      user?: string;
      action?: "logon" | "token_use" | "role_change";
      provider?: "aad" | "okta" | "local" | string;
      result?: "success" | "fail" | string;
    }>;
    file?: Array<{
      path?: string;
      sha256?: string;
      action?: "create" | "modify" | "rename" | "delete";
    }>;
  };
  ttp_evidence: Array<{ tactic?: string; technique?: string; subtechnique?: string; sources: string[] }>;
  multistage: StageHypothesis[];
  observations: VendorObservation[];
  sinks?: { opensearch?: boolean; splunk?: boolean; sentinel?: boolean; thehive?: boolean; [k: string]: boolean | undefined };
  deep_link: string;
}

// -- Legacy compatibility ---------------------------------------------------

export interface NormalizedAlertV1 {
  schema_version?: 1;
  id?: string;
  summary?: string;
  severity?: string | number;
  observed_at?: string;
  [key: string]: unknown;
}

export type NormalizedAlert = NormalizedAlertV1 | NormalizedAlertV2;
