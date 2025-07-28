
import os
import logging

def quarantine_agent(hostname: str, reason: str = "Trust score threshold breached") -> bool:
    try:
        quarantine_path = "/var/edr/quarantine"
        os.makedirs(quarantine_path, exist_ok=True)
        quarantine_marker = os.path.join(quarantine_path, f"{hostname}.q")
        with open(quarantine_marker, "w") as f:
            f.write(f"Quarantined: {hostname}\nReason: {reason}")
        logging.warning(f"[QUARANTINE] Host {hostname} has been quarantined. Reason: {reason}")
        return True
    except Exception as e:
        logging.error(f"Failed to quarantine host {hostname}: {str(e)}")
        return False
