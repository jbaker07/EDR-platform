
import base64
import re

def normalize_command(cmd):
    """Lowercase, strip, collapse whitespace, remove nulls and escape chars."""
    cmd = cmd.lower()
    cmd = re.sub(r'\x[0-9a-fA-F]{2}', '', cmd)  # Remove hex escapes
    cmd = cmd.replace('\x00', '').replace('\0', '')
    cmd = re.sub(r'\\+', '\\', cmd)
    cmd = re.sub(r'\s+', ' ', cmd.strip())
    return cmd

def decode_base64_segment(cmd):
    """Try to decode base64-encoded substrings if possible."""
    tokens = cmd.split()
    decoded_segments = []
    for tok in tokens:
        try:
            if len(tok) > 16 and re.fullmatch(r'[A-Za-z0-9+/=]+', tok):
                decoded = base64.b64decode(tok + "===", validate=True).decode('utf-8', errors='ignore')
                decoded_segments.append(decoded)
        except Exception:
            continue
    return decoded_segments

def extract_chain_features(cmd):
    features = {
        "uses_cmd": "cmd" in cmd,
        "uses_powershell": "powershell" in cmd,
        "uses_curl_or_certutil": any(x in cmd for x in ["curl", "certutil", "bitsadmin", "wget"]),
        "writes_file": any(x in cmd for x in ["echo", ">", "outfile", "set-content"]),
        "executes_payload": any(x in cmd for x in ["start", "invoke", "iex", "run", "schtasks"]),
        "obfuscation": any(x in cmd for x in ["-enc", "frombase64", "stringbuilder", "join"])
    }
    return features

def detect_suspicious_conjunctions(cmd):
    return any(
        x in cmd for x in [
            "powershell -enc", "cmd /c certutil", "curl | bash", "iex(", "invoke-expression",
            "bitsadmin && start", "wget && chmod +x"
        ]
    )
