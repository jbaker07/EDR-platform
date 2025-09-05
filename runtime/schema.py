from __future__ import annotations
from dataclasses import dataclass, field, asdict
from typing import Dict, List, Optional, Any

@dataclass
class Event:
    # Core normalized fields (superset across sensors)
    ts_ns: int
    host: str
    event_type: str                      # e.g., "file_open", "exec", "net_send"
    pid: Optional[int] = None
    ppid: Optional[int] = None
    uid: Optional[int] = None
    gid: Optional[int] = None
    tgid: Optional[int] = None
    comm: Optional[str] = None
    exe: Optional[str] = None
    path: Optional[str] = None
    fd: Optional[int] = None
    ret: Optional[int] = None
    aux_u32: Optional[int] = None

    # Network-ish
    src_ip: Optional[str] = None
    src_port: Optional[int] = None
    dst_ip: Optional[str] = None
    dst_port: Optional[int] = None
    proto: Optional[str] = None

    # Container / identity
    container_id: Optional[str] = None
    user: Optional[str] = None
    session_id: Optional[str] = None

    # Enrichment
    tags: List[str] = field(default_factory=list)
    meta: Dict[str, Any] = field(default_factory=dict)

    def to_dict(self) -> Dict[str, Any]:
        return asdict(self)


@dataclass
class Alert:
    rule_id: str
    title: str
    severity: str
    ts_ns: int
    host: str
    event: Dict[str, Any]
    reason: str
    tags: List[str] = field(default_factory=list)
    score: Optional[float] = None     # for anomaly signals etc.
