---
type: "scope"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# Branches of "Minecraft modding", and which one this corpus is

"Minecraft" is not one modification surface. The corpus resolved for this project
(see `extracted:corpus.json`, hashed) is one branch of a tree. This note names the
others so that nothing outside the corpus is silently folded in, and so that each
missing branch has an explicit gap record instead of an implied "unsupported".

Legend: **in corpus** means resolved artifacts with sha256 exist under
`extracted/`; **documented only** means a cached web source exists but no
artifact; **absent** means neither.

## 1. Editions

| branch | state | what exists | what is missing | gap record |
|---|---|---|---|---|
| Java Edition 26.3 (this corpus) | **in corpus** | `minecraft-merged`, `-client`, `-server`, `-extracted_server` jars; `version.json` (id 26.3, Java 25, protocol 777, world version 5023, resource pack 97.1, data pack 121.0) -- `source:minecraft_26_3_merged_jar`, `extracted:corpus.json` | any other Java version (no 1.21.x jar was resolved, so version diffs are not mechanically possible) | question:q.older_minecraft_versions |
| Java Edition, other versions | absent | the Mojang manifest listing versions (`source:mojang_version_manifest`) | the jars themselves | question:q.older_minecraft_versions |
| Bedrock Edition (behaviour packs, resource packs, scripting API) | absent | nothing retrieved | the Bedrock client or server, the `@minecraft/server` script modules, pack manifests, documentation | question:q.bedrock_artifacts |

Bedrock is a different program with a different language surface (JSON packs and
JavaScript), not a variant of the Java surface. Nothing in this atlas transfers to
it except the workflow *intents* in `10-Workflows`.

## 2. Loaders and runtimes (Java Edition)

| branch | state | what exists | what is missing | gap record |
|---|---|---|---|---|
| Fabric (loader 0.19.5, API 0.161.0+26.3) | **in corpus** | fabric-loader, 47 Fabric API modules, sponge-mixin 0.17.4+mixin.0.8.7, class-tweaker 0.3.0-beta.2 -- `extracted:corpus.json`, `extracted:fabric_api.json` | Fabric API *sources* (javadoc), so documented contracts are not available from the artifacts | question:q.fabric_javadoc_sources |
| Quilt | absent | the pack manifest names it as a Fabric-compatible loader (`packs/minecraft/pack.yaml`) | quilt-loader and QSL artifacts | question:q.neoforge_forge_quilt_artifacts |
| NeoForge | documented only | `source:neoforge_docs_getting_started` | the NeoForge jar, its event bus, its mod descriptor format | question:q.neoforge_forge_quilt_artifacts |
| Forge (legacy) | absent | nothing retrieved | everything | question:q.neoforge_forge_quilt_artifacts |
| Vanilla, no loader (data packs and resource packs only) | **in corpus** | the pack formats and every data/asset entry type the jar itself ships (`extracted:corpus.json`: 44 data entry types, 15 asset entry types) | the pack *format specification* prose; only the jar's own examples are present | question:q.mojang_changelog_26_3 |

The mixin runtime is shared by Fabric, Quilt and NeoForge in principle, so the
`mechanism:Mixin` note is the part of this atlas most likely to transfer -- but
that transfer is not established by anything in the corpus.

## 3. Server-side plugin platforms

| branch | state | what exists | what is missing | gap record |
|---|---|---|---|---|
| Paper / Spigot / Bukkit plugins | absent | nothing retrieved | the server jar, the plugin API, `plugin.yml` format | question:q.paper_plugin_api |
| Velocity / BungeeCord proxies | absent | nothing retrieved | everything | question:q.paper_plugin_api |

Plugins do not modify game bytecode and do not run on clients; they are a
separate surface with separate compatibility rules. Fabric server-side mods are
the corpus's nearest equivalent, and they are *not* interchangeable with plugins.

## 4. Execution contexts inside Java Edition (all in corpus at the artifact level)

| context | artifacts | how the atlas distinguishes it |
|---|---|---|
| client (with integrated server for single-player) | `minecraft-client` jar; `net.minecraft.client.*` packages; `net.minecraft.client.server` package present in the jar (`extracted:corpus.json`) | Fabric API modules and mixin configs marked `environment: client`; edges carry `applies_to.environment` |
| dedicated server | `minecraft-server` / `minecraft-extracted_server` jars; `net.minecraft.server.dedicated` package | mixin configs' `server` lists; `net.fabricmc.api.DedicatedServerModInitializer` entrypoint (`type:net.fabricmc.api.DedicatedServerModInitializer`) |
| merged (what Loom compiles against) | `minecraft-merged` jar | the default for every `direct_reference` edge |

What is **not** established for these contexts: which thread any hooked method
runs on (see question:q.worldgen_threading and question:q.payload_receiver_thread),
and whether the integrated server behaves identically to the dedicated one for a
given hook. Both need a running game (question:q.runtime_event_delivery).

## 5. Distribution channels (documented only)

`source:modrinth_fabric_api` (Modrinth project metadata) and `source:nexusmods_web`
(Nexus Mods home page) were retrieved. CurseForge was not. Distribution terms are
recorded per source in `packs/minecraft/sources.yaml`; nothing in the corpus is
redistributed by ModCheck.
