"""Rendering the prediction/observation loop as a table a reader can audit."""
from __future__ import annotations

from .prediction import Comparison, Prediction

_MARK = {"matched": "MATCH", "contradicted": "WRONG", "unobserved": "  ?  "}


def render_predictions(predictions: list[Prediction]) -> str:
    lines = ["Prediction records", ""]
    lines.append(f"  {'id':26} {'case':22} {'claims':>6}  observation")
    lines.append(f"  {'-' * 26} {'-' * 22} {'-' * 6}  {'-' * 24}")
    for prediction in predictions:
        note = prediction.observation_status
        if prediction.observation_status == "blocked" and prediction.observation_blocked_reason:
            note = f"blocked: {prediction.observation_blocked_reason}"
        lines.append(f"  {prediction.id:26} {prediction.case:22} {len(prediction.claims):>6}  {note}")
    lines.append("")
    lines.append("  A prediction is only a prediction while its observation reads")
    lines.append("  not_yet_observed. Once a transcript is filed the record is closed:")
    lines.append("  it cannot be edited to fit what the transcript said.")
    return "\n".join(lines)


def render_prediction(prediction: Prediction) -> str:
    lines = [f"{prediction.id}  ({prediction.case})", ""]
    lines.append(f"  question: {prediction.question.strip()}")
    lines.append(f"  recorded: {prediction.recorded_at}")
    lines.append(f"  observation: {prediction.observation_status}")
    if prediction.observation_blocked_reason:
        lines.append(f"  blocked because: {prediction.observation_blocked_reason}")
    lines.append("")
    lines.append("  derived from:")
    lines.append(f"    {prediction.derived_from.strip()}")
    lines.append("")
    lines.append("  inputs (a prediction is about these exact bytes):")
    for item in prediction.inputs:
        lines.append(f"    {item.sha256[:16]}...  {item.bytes:>6}  {item.path}")
        if item.note:
            lines.append(f"      {item.note}")
    lines.append("")
    lines.append("  what must be run to judge it:")
    for command in prediction.commands:
        lines.append(f"    {command}")
    lines.append("")
    lines.append("  requires:")
    for key, value in prediction.requires.items():
        lines.append(f"    {key}: {str(value).strip()}")
    lines.append("")
    lines.append("  claims:")
    for claim in prediction.claims:
        lines.append(f"    - {claim.describe()}")
        if claim.because:
            lines.append(f"      because {claim.because.strip()}")
    return "\n".join(lines)


def render_comparison(comparison: Comparison) -> str:
    lines = [f"{comparison.prediction.id}: {comparison.verdict.upper()}", ""]
    for result in comparison.results:
        lines.append(f"  [{_MARK[result.verdict]}] {result.claim.describe()}")
        lines.append(f"          observed: {result.observed}")
    lines.append("")
    if comparison.verdict == "incomplete":
        lines.append("  Claims marked ? were not answered by the transcripts supplied.")
        lines.append("  An unanswered claim is not a passed one: run the commands the")
        lines.append("  prediction names, or the result stays incomplete.")
    return "\n".join(lines)
