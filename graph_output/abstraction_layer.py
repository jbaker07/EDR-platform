
import re
import json

class AbstractionLayer:
    def __init__(self, ontology_path, semantic_tags_path, threat_patterns_path):
        with open(ontology_path) as f:
            self.ontology = json.load(f)
        with open(semantic_tags_path) as f:
            self.semantic_tags = json.load(f)
        with open(threat_patterns_path) as f:
            self.threat_patterns = json.load(f)

    def infer_platform(self, telemetry):
        exe = telemetry.get("exe", "").lower()
        path = telemetry.get("file_path", "").lower()
        if "launchd" in exe or "osascript" in exe or path.startswith("/users/"):
            return "macos"
        elif "cmd.exe" in exe or "powershell" in exe or path.startswith("c:\\"):
            return "windows"
        return "unknown"

    def validate_telemetry(self, telemetry):
        required_fields = ["exe", "cmdline"]
        return all(field in telemetry for field in required_fields)

    def semi_supervised_tag_prediction(self, telemetry):
        possible_tags = []
        cmd = telemetry.get("cmdline", "")
        for tag, data in self.semantic_tags.items():
            keywords = data.get("keywords", [])
            if any(word in cmd for word in keywords[:2]):  # partial hit for inference
                possible_tags.append(tag)
        return possible_tags

    def normalize(self, telemetry):
        if not self.validate_telemetry(telemetry):
            return {"error": "invalid_telemetry", "original": telemetry}

        norm = {}
        raw_cmd = telemetry.get("cmdline", "")
        exe = telemetry.get("exe", "")
        file_path = telemetry.get("file_path", "")
        norm["platform"] = self.infer_platform(telemetry)

        # Normalize execution context
        if 'osascript' in exe:
            norm['abstracted_type'] = 'script_exec'
        elif 'bash' in exe or 'zsh' in exe:
            norm['abstracted_type'] = 'shell_exec'
        elif 'launchctl' in exe:
            norm['abstracted_type'] = 'persistence_loader'
        elif 'curl' in raw_cmd or 'wget' in raw_cmd:
            norm['abstracted_type'] = 'external_fetch'
        elif 'log erase' in raw_cmd or 'log' in exe:
            norm['abstracted_type'] = 'log_clear'
        elif re.search(r'base64\s+[-]{0,2}decode', raw_cmd):
            norm['abstracted_type'] = 'encoded_payload_decode'
        elif file_path.startswith("/tmp"):
            norm['abstracted_type'] = 'tmp_exec'
        else:
            norm['abstracted_type'] = 'unknown'

        # Tag matching
        norm['tags'] = []
        for tag, pattern in self.semantic_tags.items():
            if any(p in raw_cmd or p in exe for p in pattern.get('keywords', [])):
                norm['tags'].append(tag)

        # Use semi-supervised inference if tags are empty
        if not norm['tags']:
            norm['tags'] = self.semi_supervised_tag_prediction(telemetry)

        for pattern in self.threat_patterns.get(norm["platform"], []):
            if all(k in raw_cmd for k in pattern.get('indicators', [])):
                norm['tags'].append(pattern['tag'])

        norm['original'] = telemetry
        return norm

    def abstract_stream(self, telemetry_stream):
        return [self.normalize(entry) for entry in telemetry_stream]
