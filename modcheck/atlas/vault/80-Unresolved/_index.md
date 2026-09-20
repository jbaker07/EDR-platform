---
type: "index"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# Unresolved questions and engineering requirements

| id | kind | status | affects |
|---|---|---|---|
| [[80-Unresolved/q.lantern_rain_semantics|q.lantern_rain_semantics]] | ambiguous_creator_intent | open | request:request.rain_lantern |
| [[80-Unresolved/q.team_counter_scope|q.team_counter_scope]] | ambiguous_creator_intent | open | request:request.team_counter |
| [[80-Unresolved/q.contested_method_policy|q.contested_method_policy]] | conflicting_requirements | open | workflow:wf.behaviour.vanilla_modification, workflow:wf.integration.mixin_coexistence, request:request.villager_fear |
| [[80-Unresolved/q.pack_format_old_packs|q.pack_format_old_packs]] | conflicting_requirements | open | workflow:wf.presentation.models_animation, workflow:wf.engineering.version_migration |
| [[80-Unresolved/q.edge_targets_unresolved|q.edge_targets_unresolved]] | incomplete_extraction | open | workflow:wf.behaviour.vanilla_modification, workflow:wf.integration.mixin_coexistence |
| [[80-Unresolved/q.event_phase_ordering|q.event_phase_ordering]] | incomplete_extraction | open | workflow:wf.behaviour.lifecycle_events, workflow:wf.integration.mixin_coexistence |
| [[80-Unresolved/q.event_publishers_missing|q.event_publishers_missing]] | incomplete_extraction | open | workflow:wf.behaviour.lifecycle_events |
| [[80-Unresolved/q.field_read_write_direction|q.field_read_write_direction]] | incomplete_extraction | open | workflow:wf.integration.mixin_coexistence, workflow:wf.state.persistence |
| [[80-Unresolved/q.injection_points_not_resolved|q.injection_points_not_resolved]] | incomplete_extraction | open | workflow:wf.integration.mixin_coexistence, workflow:wf.behaviour.vanilla_modification, request:request.villager_fear |
| [[80-Unresolved/q.unhooked_vanilla_members|q.unhooked_vanilla_members]] | incomplete_extraction | open | request:request.villager_fear, request:request.crystal_caves, request:request.lantern_moth |
| [[80-Unresolved/q.bedrock_artifacts|q.bedrock_artifacts]] | missing_artifact_access | open | workflow:wf.content.data_driven_content, workflow:wf.behaviour.entity_ai |
| [[80-Unresolved/q.neoforge_forge_quilt_artifacts|q.neoforge_forge_quilt_artifacts]] | missing_artifact_access | open | workflow:wf.integration.dependencies_conditions, workflow:wf.engineering.project_setup |
| [[80-Unresolved/q.older_minecraft_versions|q.older_minecraft_versions]] | missing_artifact_access | open | workflow:wf.engineering.version_migration, request:request.port_1_21_mod, workflow:wf.state.migration |
| [[80-Unresolved/q.paper_plugin_api|q.paper_plugin_api]] | missing_artifact_access | open | workflow:wf.multiplayer.side_separation, workflow:wf.behaviour.lifecycle_events |
| [[80-Unresolved/q.external_asset_tools|q.external_asset_tools]] | missing_documentation | open | workflow:wf.presentation.models_animation, workflow:wf.presentation.particles_sounds, request:request.lantern_moth |
| [[80-Unresolved/q.fabric_docs_module_pages|q.fabric_docs_module_pages]] | missing_documentation | open | workflow:wf.multiplayer.networking, workflow:wf.presentation.hud_screens, workflow:wf.content.data_driven_content |
| [[80-Unresolved/q.fabric_javadoc_sources|q.fabric_javadoc_sources]] | missing_documentation | open | workflow:wf.behaviour.lifecycle_events, workflow:wf.multiplayer.networking, workflow:wf.behaviour.entity_ai |
| [[80-Unresolved/q.mixin_docs_application_order|q.mixin_docs_application_order]] | missing_documentation | open | workflow:wf.integration.mixin_coexistence |
| [[80-Unresolved/q.mod_data_migration_practice|q.mod_data_migration_practice]] | missing_documentation | open | workflow:wf.state.migration, request:request.port_1_21_mod, request:request.team_counter |
| [[80-Unresolved/q.mojang_changelog_26_3|q.mojang_changelog_26_3]] | missing_documentation | open | workflow:wf.presentation.models_animation, workflow:wf.content.data_driven_content, workflow:wf.engineering.version_migration |
| [[80-Unresolved/q.runtime_event_delivery|q.runtime_event_delivery]] | unavailable_runtime_observation | open | workflow:wf.behaviour.lifecycle_events, request:request.rain_lantern, request:request.team_counter |
| [[80-Unresolved/q.runtime_mixin_application|q.runtime_mixin_application]] | unavailable_runtime_observation | open | workflow:wf.integration.mixin_coexistence, workflow:wf.engineering.testing, request:request.rain_lantern |
| [[80-Unresolved/q.runtime_performance|q.runtime_performance]] | unavailable_runtime_observation | open | request:request.rain_lantern, request:request.crystal_caves, workflow:wf.world.features_biomes |
| [[80-Unresolved/q.contract_checker|q.contract_checker]] | unimplemented_automation | open | workflow:wf.behaviour.lifecycle_events, workflow:wf.multiplayer.side_separation |
| [[80-Unresolved/q.inspector_mixin_targets|q.inspector_mixin_targets]] | unimplemented_automation | open | workflow:wf.integration.mixin_coexistence, workflow:wf.integration.dependencies_conditions |
| [[80-Unresolved/q.mixin_collision_analyser|q.mixin_collision_analyser]] | unimplemented_automation | open | workflow:wf.integration.mixin_coexistence, workflow:wf.engineering.testing |
| [[80-Unresolved/q.version_diff_tool|q.version_diff_tool]] | unimplemented_automation | open | workflow:wf.engineering.version_migration, request:request.port_1_21_mod |
| [[80-Unresolved/q.datapack_load_order|q.datapack_load_order]] | unmodeled_behaviour | open | workflow:wf.world.features_biomes, workflow:wf.content.data_driven_content, request:request.crystal_caves |
| [[80-Unresolved/q.payload_receiver_thread|q.payload_receiver_thread]] | unmodeled_behaviour | open | workflow:wf.multiplayer.networking, request:request.team_counter, request:request.rain_lantern |
| [[80-Unresolved/q.registry_freeze_timing|q.registry_freeze_timing]] | unmodeled_behaviour | open | workflow:wf.content.block_item, workflow:wf.content.entity_type, workflow:wf.integration.registry_sync |
| [[80-Unresolved/q.villager_ai_architecture|q.villager_ai_architecture]] | unmodeled_behaviour | open | request:request.villager_fear, workflow:wf.behaviour.entity_ai |
| [[80-Unresolved/q.worldgen_threading|q.worldgen_threading]] | unmodeled_behaviour | open | workflow:wf.world.features_biomes, request:request.crystal_caves, workflow:wf.behaviour.lifecycle_events |
