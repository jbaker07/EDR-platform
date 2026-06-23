"""End-to-end serving tests for POST /api/predict.

These assert the endpoint returns a real prediction out-of-the-box (the
committed DEMO model loads) and that the synthetic contract behaves sanely:
a clearly-benign sample and a clearly-malicious-looking sample both return a
valid label, and the demo model separates them.

Feature contract (see backend/ml/train.py FEATURE_ORDER):
    [cpu_percent, memory_bytes, pid]
"""

from __future__ import annotations

VALID_LABELS = {"benign", "malicious"}

# Low CPU + low memory -> benign per the synthetic rule.
BENIGN_SAMPLE = {"features": [2.0, 80_000_000, 501]}
# High CPU + high memory -> malicious per the synthetic rule.
MALICIOUS_SAMPLE = {"features": [92.0, 3_200_000_000, 731]}


def _predict(client, payload):
    resp = client.post("/api/predict", json=payload)
    assert resp.status_code == 200, resp.text
    body = resp.json()
    assert body["label"] in VALID_LABELS, body
    assert body["prediction"] in VALID_LABELS, body
    assert body["prediction"] == body["label"]
    return body


def test_predict_benign_sample(client):
    body = _predict(client, BENIGN_SAMPLE)
    assert body["label"] == "benign", body


def test_predict_malicious_sample(client):
    body = _predict(client, MALICIOUS_SAMPLE)
    assert body["label"] == "malicious", body


def test_two_inputs_classified_differently(client):
    benign = _predict(client, BENIGN_SAMPLE)
    malicious = _predict(client, MALICIOUS_SAMPLE)
    # The two inputs are handled and the demo model separates them.
    assert benign["label"] != malicious["label"], (benign, malicious)


def test_confidence_present_and_bounded(client):
    body = _predict(client, MALICIOUS_SAMPLE)
    # LogisticRegression exposes predict_proba, so confidence should surface.
    assert "confidence" in body, body
    assert 0.0 <= body["confidence"] <= 1.0, body


def test_wrong_feature_count_rejected(client):
    resp = client.post("/api/predict", json={"features": [1.0, 2.0]})
    assert resp.status_code == 422, resp.text
