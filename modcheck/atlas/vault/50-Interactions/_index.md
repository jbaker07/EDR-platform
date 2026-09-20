---
type: "index"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# Interaction contracts

An edge is a typed relationship with exact endpoints, a resolution state and an evidence class. Totals: {"injects_into": 544, "wraps": 197, "reads": 1431, "calls": 5088, "publishes_event": 242, "callback_of": 189, "writes": 36, "replaces": 1, "registers_into": 97}. Evidence: {"direct_reference": 7094, "declared": 489, "static_inference": 242}. Resolution: {"injects_into": {"exact": 93, "name_only": 441, "ambiguous": 5, "selector_unsupported": 5}, "wraps": {"exact": 33, "name_only": 163, "ambiguous": 1}, "reads": {"exact": 1422, "inherited_exact": 9}, "calls": {"exact": 4373, "inherited_exact": 715}, "writes": {"exact": 36}, "replaces": {"exact": 1}}.

- [[50-Interactions/events/_index|Events]] -- what publishes each, and its callback
- [[50-Interactions/shared_targets|Shared targets]] -- vanilla methods more than one module injects into: potential interactions
- [[50-Interactions/overwrites|Overwrites]] -- methods replaced outright
- [[30-Mechanisms/Transformation_Tests|Composition rules]] -- what the transformer does with a pair of injections
- [[50-Interactions/store_records|Curated interaction records]]
- Analyst contracts: [[_authored/contracts/_index|index]]
