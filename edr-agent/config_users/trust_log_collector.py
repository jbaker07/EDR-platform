from flask import Flask, request, jsonify
import os, json

app = Flask(__name__)
os.makedirs("received_trust_logs", exist_ok=True)

@app.route("/submit_log", methods=["POST"])
def submit_log():
    data = request.json
    endpoint_id = data.get("endpoint_id", "unknown")
    with open(f"received_trust_logs/{endpoint_id}_{data['timestamp']}.json", "w") as f:
        json.dump(data, f, indent=2)
    return jsonify({"status": "received"}), 200

if __name__ == "__main__":
    app.run(host="0.0.0.0", port=8080)

@app.route("/submit_log", methods=["POST"])
def submit_log():
    data = request.json
    print(f"[RECEIVED] {data['endpoint_id']} trust score: {data['trust_score']}")

    # Optional trigger
    if data["trust_score"] < 30:
        print(f"⚠️  REPLAY triggered for {data['endpoint_id']} due to low trust.")

    with open(f"received_trust_logs/{data['endpoint_id']}_{data['timestamp']}.json", "w") as f:
        json.dump(data, f, indent=2)

    return jsonify({"status": "received"}), 200
