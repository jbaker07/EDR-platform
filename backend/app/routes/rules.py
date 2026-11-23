from fastapi import APIRouter, HTTPException
from pydantic import BaseModel
from typing import List
import json
import logging
from pathlib import Path

from app.rules.safe_expr import RuleError, compile_rule, evaluate

router = APIRouter()
logger = logging.getLogger(__name__)

# backend/logs/ next to main.py, the same folder as iocs.json, whatever the CWD.
RULES_FILE = Path(__file__).resolve().parents[2] / "logs" / "rules.json"

# The only process fields a rule may read.
RULE_FIELDS = ("cpu_usage", "memory", "cmd", "name")

class Process(BaseModel):
    pid: int
    name: str
    cmd: str
    memory: int
    cpu_usage: float
    status: str

class HostTelemetry(BaseModel):
    hostname: str
    timestamp: int
    processes: List[Process]

@router.post("/rules/eval")
def evaluate_rules(data: HostTelemetry):
    if not RULES_FILE.exists():
        raise HTTPException(status_code=500, detail="Rules file not found")

    rules = json.loads(RULES_FILE.read_text())

    # Check every rule once, before any telemetry touches it. Rules are never
    # passed to eval(): safe_expr parses them and allows only plain
    # comparisons on the fields above. A rule that breaks the allowlist is
    # logged and skipped instead of running.
    compiled = []
    for rule in rules:
        try:
            compiled.append((rule, compile_rule(rule["rule"], RULE_FIELDS)))
        except (RuleError, KeyError, TypeError) as e:
            logger.warning("Rule rejected: %r (%s)", rule.get("rule") if isinstance(rule, dict) else rule, e)

    alerts = []

    for proc in data.processes:
        values = {
            "cpu_usage": proc.cpu_usage,
            "memory": proc.memory,
            "cmd": proc.cmd,
            "name": proc.name,
        }
        for rule, tree in compiled:
            try:
                if evaluate(tree, values):
                    alerts.append({
                        "hostname": data.hostname,
                        "timestamp": data.timestamp,
                        "pid": proc.pid,
                        "process": proc.name,
                        "rule": rule["description"],
                        "severity": rule["severity"]
                    })
            except (RuleError, KeyError) as e:
                logger.warning("Rule failed: %r (%s)", rule["rule"], e)

    return {"alerts": alerts}
