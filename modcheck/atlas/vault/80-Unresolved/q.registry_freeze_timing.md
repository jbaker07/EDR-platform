---
type: "question"
id: "q.registry_freeze_timing"
kind: "unmodeled_behaviour"
status: "open"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# q.registry_freeze_timing

**Question.** Relative to ModInitializer.onInitialize and the DynamicRegistrySetupCallback, when are the built-in registries frozen in 26.3, and which registrations after that point fail?

**Kind.** `unmodeled_behaviour` -- **Status.** open

**Why it matters.** Register-too-late is a classic startup crash. The atlas lists 97 built-in registries and the entrypoints, but no lifecycle_phase edge orders them.

**Affects.** [[10-Workflows/wf.content.block_item|wf.content.block_item]], [[10-Workflows/wf.content.entity_type|wf.content.entity_type]], [[10-Workflows/wf.integration.registry_sync|wf.integration.registry_sync]]

**Evidence already available.**
- [[30-Mechanisms/Registries|Registries]]
- [[40-Interfaces/net.fabricmc.api.ModInitializer|ModInitializer]]
- [[50-Interactions/events/net.fabricmc.fabric.api.event.registry.DynamicRegistrySetupCallback.EVENT|EVENT]]

**Best remaining source.** net.minecraft.core.registries.BuiltInRegistries (freeze call sites) and fabric-loader's entrypoint invocation sites, read with javap -c.

**Procedure.** Extract the freeze call chain and the entrypoint invocation chain; emit orders_before edges between lifecycle phases with static_inference.

**Done when.** A lifecycle note orders entrypoints, registry freeze and dynamic registry load with edge ids.

**Conclusions affected while open.**
- Content workflows say "register in onInitialize" by convention, not by extracted ordering.
