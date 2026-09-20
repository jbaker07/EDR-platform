# Engineering backlog derived from the atlas

Ordered by dependency: an item lists what it needs. Each item names the
unresolved question it closes. Sizes are relative (S/M/L). No confidence is
attached. Items closed since the first version are listed at the end so the
change is visible, not silent.

## Tier 0 -- no dependencies

1. **Merge the class-file extractor into the inspector** (question:q.inspector_mixin_targets, S).
   `atlas/extract/classfile.py`, `jvm.py` and `resolve.py` read annotations,
   opcodes and members exactly; the inspector still lists mixin classes by name.
   One implementation, used by both, with the fixture tests moved alongside.
2. **Handler-signature matching for name-only selectors** (question:q.selector_ambiguity, S).
   Six ambiguous and five quantified selectors remain; Mixin's own rule is the
   handler's parameter types.
3. **Provenance of processed-jar changes** (question:q.processed_jar_change_provenance, S).
   Attach the widening module to each of the 771 member and 88 hierarchy changes
   from the modules' access-widener and interface-injection declarations.
4. **Extract extra vanilla types on request** (question:q.unhooked_vanilla_members, S).
   The full surface now exists (`extracted:minecraft_surface.json.gz`, 11383
   classes); what is missing is per-class notes for the classes the exercises
   name (villager AI, levelgen features, client animation).
5. **Publisher status for the two publisher-less events** (question:q.event_publishers_missing, S).

## Tier 1 -- needs tier 0

6. **Injection-point offsets** (question:q.injection_points_not_resolved, M; needs 2).
   Locate each resolved point inside the target method's code; then
   `shared_targets` can say same-point versus same-method.
7. **Collision analysis, in the established order** (question:q.mixin_collision_analyser, M; needs 1, 2, 6).
   (1) shared-target index over arbitrary jars -- potential interactions only;
   (2) exact resolution of both sides; (3) applicability by environment;
   (4) the composition rule for the injector pair and priorities from
   `extracted:mixin_transformation_tests.json`. A conflict is reported at step 4
   only. Never convert an overlap into a conflict.
8. **Event phase ordering extraction** (question:q.event_phase_ordering, S).
9. **Villager AI and worldgen candidate verification** (question:q.villager_ai_architecture,
   question:q.worldgen_threading, M; needs 4).
10. **Contested-method policy** (question:q.contested_method_policy; a founder decision,
    now decidable against the composition table rather than a guess).

## Tier 2 -- needs evidence acquisition

11. **Fabric API sources and javadoc** (question:q.fabric_javadoc_sources, M).
12. **Documentation pages the workflows cite by need** (question:q.fabric_docs_module_pages,
    question:q.mojang_changelog_26_3, question:q.mixin_docs_application_order,
    question:q.mod_data_migration_practice, question:q.external_asset_tools, S each).
13. **Second Minecraft corpus and the version diff** (question:q.older_minecraft_versions
    then question:q.version_diff_tool, L). The surface extractor already
    produces the per-version input.
14. **Registry freeze and data pack order** (question:q.registry_freeze_timing,
    question:q.datapack_load_order, M; needs 4).

## Tier 3 -- needs a runtime

15. **Headless dedicated-server run in a sandbox** (question:q.runtime_mixin_application,
    question:q.runtime_event_delivery, question:q.lantern_runtime_gate, L).
    Gametests for the lantern's four server-side claims; a probe mod for the
    event list; `modcheck runtime observe` promotes edges to `observed`.
    The transformation harness is not this: it establishes transformer
    behaviour, not game behaviour.
16. **Performance measurement** (question:q.runtime_performance, M; needs 15).
17. **Payload receiver thread** (question:q.payload_receiver_thread, S static + 15 to confirm).

## Tier 4 -- new branches (each its own corpus)

18. NeoForge / Quilt / Forge artifacts (question:q.neoforge_forge_quilt_artifacts, L).
19. Paper plugin API (question:q.paper_plugin_api, M).
20. Bedrock (question:q.bedrock_artifacts, L).

## Closed since the first version

- question:q.edge_targets_unresolved -- exact resolution against the processed jar, libraries and JDK; 0 unresolved calls or reads.
- question:q.field_read_write_direction -- opcodes read from the Code attribute; reads and writes are separate relations.
- question:q.lantern_rain_semantics -- taken from the canonical request's proposed contract, recorded as a proposal.
- question:q.team_counter_scope -- will not resolve; both definitions carried explicitly in the exercise.
- The "overwrite removes events" claim -- withdrawn; replaced by the executed composition table (mechanism:Mixin).
