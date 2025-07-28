import requests
import json
from datetime import datetime

# Example: Collect telemetry data from an agent endpoint
def fetch_telemetry():
    try:
        response = requests.get("http://127.0.0.1:8000/api/telemetry")
        data = response.json()

        filename = f"telemetry_{datetime.now().strftime('%Y%m%d_%H%M%S')}.json"
        with open(f"app/ingestion/data/{filename}", "w") as f:
            json.dump(data, f, indent=2)

        print(f"Saved telemetry to {filename}")
    except Exception as e:
        print(f"[ERROR] Could not fetch telemetry: {e}")

if __name__ == "__main__":
    fetch_telemetry()
