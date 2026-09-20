# Engineering backlog derived from the atlas

Ordered by dependency: an item lists what it needs. Each item names the
unresolved question it closes, so completion is checkable in `80-Unresolved`.
Sizes are relative (S/M/L), not hours; no confidence is attached.

## Tier 0 -- no dependencies

1. **Merge the mixin extractor into the inspector** (question:q.inspector_mixin_targets, S).
   Move `atlas/extract/jvm.py` `mixin_facts` into `src/modcheck/inspect/`, have the
   atlas import it, add a `mixin_targets` fact key with tests. Removes the second
   code path for one fact.
2. **Walk interfaces and interface injections in the member cross-check**
   (question:q.edge_targets_unresolved, S). Extend `vanilla_members.py`; the
   unresolved list should fall to zero or carry categories.
3. **Separate field reads from writes** (question:q.field_read_write_direction, S).
   Keep the opcode in `class_refs`; emit `writes` edges.
4. **Extract extra vanilla types on request** (question:q.unhooked_vanilla_members, S).
   `vanilla_members.py --extra` from request notes' affected systems; villager AI,
   levelgen features and client animation first (they block three request analyses).
5. **Resolve the two creator-intent questions** (question:q.lantern_rain_semantics,
   question:q.team_counter_scope, S). Two questions to the founder; answers go
   into the request notes' approved_behaviour.

## Tier 1 -- needs tier 0

6. **Mixin collision analyser over arbitrary jars** (question:q.mixin_collision_analyser, M;
   needs 1). `modcheck inspect --mixins`: pairwise same-method, same-call-site
   redirect and overwrite detection with a failure record and detector.
7. **Injection-point resolution** (question:q.injection_points_not_resolved, M; needs 1).
   Match `@At` targets against `javap -c` of the target method; attach instruction
   locators to edges; `contested_methods` then distinguishes same-method from
   same-point.
8. **Event phase ordering extraction** (question:q.event_phase_ordering, S; needs 3's
   opcode plumbing). `orders_before` edges from `addPhaseOrdering` call sites.
9. **Widen event-publisher scanning to every module class**
   (question:q.event_publishers_missing, S). Closes the 7 events without a publisher.
10. **Villager AI and worldgen candidate verification** (question:q.villager_ai_architecture,
    question:q.worldgen_threading, M; needs 4). Read the extracted classes; record
    `runs_on_thread` edges as static_inference; update the two request notes.

## Tier 2 -- needs evidence acquisition

11. **Resolve Fabric API sources and extract javadoc** (question:q.fabric_javadoc_sources, M).
    Upgrades contract claims from analyst_inference to documented; needs Loom's
    sources resolution and a javadoc extractor.
12. **Cache the documentation pages the workflows cite by need**
    (question:q.fabric_docs_module_pages, question:q.mojang_changelog_26_3,
    question:q.mixin_docs_application_order, question:q.mod_data_migration_practice,
    question:q.external_asset_tools, S each). `modcheck sources add` with reuse terms.
13. **Second Minecraft corpus and the version diff** (question:q.older_minecraft_versions
    then question:q.version_diff_tool, L). A scratch Loom project for 1.21.x; diff
    member sets per hooked type; generated migration note.
14. **Registry freeze and data pack order** (question:q.registry_freeze_timing,
    question:q.datapack_load_order, M; needs 4). Extract the freeze and pack-ordering
    call chains; `orders_before` edges between lifecycle phases.

## Tier 3 -- needs a runtime

15. **Headless dedicated-server run in a sandbox** (question:q.runtime_mixin_application,
    question:q.runtime_event_delivery, L). A probe mod generated from the event list;
    Mixin debug flags; `modcheck runtime observe` ingests the log and promotes edges
    to `observed` with run provenance. Never touches a user install.
16. **Performance measurement** (question:q.runtime_performance, M; needs 15).
    Profiler around the lantern handler and a chunk-generation listener.
17. **Payload receiver thread** (question:q.payload_receiver_thread, S static + needs 15
    to confirm). Trace the dispatch path in fabric-networking-api-v1 impl classes.

## Tier 4 -- new branches (each is its own corpus)

18. NeoForge / Quilt / Forge artifacts (question:q.neoforge_forge_quilt_artifacts, L).
19. Paper plugin API (question:q.paper_plugin_api, M).
20. Bedrock (question:q.bedrock_artifacts, L; needs a TypeScript-declaration extractor).

## Policy items (founder decisions, not code)

- Contested-method policy (question:q.contested_method_policy).
- Supported pack-format range policy (question:q.pack_format_old_packs).
- Contract checker scope (question:q.contract_checker), after 8 and 11.
