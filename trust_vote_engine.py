# trust_vote_engine.py

from typing import Optional, Dict

def vote_trust_score(
    gnn_score: Optional[float],
    rule_score: Optional[float],
    digest_score: Optional[float],
    weights: Dict[str, float] = {
        "gnn": 0.5,
        "rule": 0.3,
        "digest": 0.2
    },
    fallback: float = 50.0
) -> Dict[str, float]:
    """
    Calculates a final trust score using weighted voting among three engines.
    Handles missing data and fallbacks.

    Args:
        gnn_score (float | None): GNN trust score (0–100)
        rule_score (float | None): Rule-based trust score (0–100)
        digest_score (float | None): Historical digest-based score (0–100)
        weights (dict): Voting weights per engine
        fallback (float): Default trust score if all are missing

    Returns:
        Dict with:
            - final_score
            - missing_inputs
            - explanation
    """

    inputs = {
        "gnn": gnn_score,
        "rule": rule_score,
        "digest": digest_score
    }

    total_weight = 0.0
    weighted_sum = 0.0
    missing = []

    for source, score in inputs.items():
        if score is not None:
            weight = weights.get(source, 0)
            weighted_sum += score * weight
            total_weight += weight
        else:
            missing.append(source)

    if total_weight == 0:
        return {
            "final_score": fallback,
            "missing_inputs": missing,
            "explanation": f"Fallback score used due to missing inputs: {', '.join(missing)}"
        }

    final = round(weighted_sum / total_weight, 2)
    return {
        "final_score": final,
        "missing_inputs": missing,
        "explanation": f"Weighted trust score computed with available engines: {', '.join([k for k in inputs if inputs[k] is not None])}"
    }
