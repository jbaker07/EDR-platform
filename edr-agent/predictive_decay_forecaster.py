import json
import numpy as np
import pandas as pd
from datetime import datetime, timedelta
from pathlib import Path
from typing import List, Dict
from sklearn.linear_model import LinearRegression

# File locations (adjust as needed)
TRUST_HISTORY_PATH = Path("trust_store/trust_history.json")
FORECAST_OUTPUT_PATH = Path("trust_store/trust_forecast.json")


def load_trust_history() -> List[Dict]:
    with open(TRUST_HISTORY_PATH, "r") as f:
        return json.load(f)


def forecast_decay(trust_data: List[Dict], forecast_hours: int = 6) -> Dict[str, List[Dict]]:
    forecasts = {}

    for entry in trust_data:
        entity_id = entry["entity_id"]
        history = entry["history"]  # List of {timestamp, trust_score}

        if len(history) < 3:
            continue  # not enough data to forecast

        timestamps = [datetime.fromtimestamp(h["timestamp"]) for h in history]
        scores = [h["trust_score"] for h in history]

        time_deltas = np.array([(t - timestamps[0]).total_seconds() / 3600 for t in timestamps]).reshape(-1, 1)
        score_series = np.array(scores).reshape(-1, 1)

        model = LinearRegression()
        model.fit(time_deltas, score_series)

        forecast = []
        last_time = timestamps[-1]
        for i in range(1, forecast_hours + 1):
            future_time = last_time + timedelta(hours=i)
            delta = (future_time - timestamps[0]).total_seconds() / 3600
            predicted_score = model.predict([[delta]])[0][0]
            forecast.append({
                "timestamp": int(future_time.timestamp()),
                "predicted_trust_score": round(float(predicted_score), 2)
            })

        forecasts[entity_id] = forecast

    return forecasts


def run_forecast():
    trust_data = load_trust_history()
    forecast_data = forecast_decay(trust_data)

    with open(FORECAST_OUTPUT_PATH, "w") as f:
        json.dump(forecast_data, f, indent=4)

    print(f"[✓] Forecasted trust decay for {len(forecast_data)} entities. Saved to {FORECAST_OUTPUT_PATH}")


if __name__ == "__main__":
    run_forecast()
