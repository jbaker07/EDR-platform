---
type: "index"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# Known failures and resolutions (store records)

## java_runtime_older_than_the_mod_requires

**Mods for current Minecraft versions do not load on an older Java runtime**

- symptom: The mod fails to load, or the game refuses to start, on a Java runtime older than the one the mod's class files target.
- mechanism: A mod's class files are compiled to a specific Java class-file version and its mixins declare a compatibilityLevel. The Minecraft 26.x toolchain targets Java 25: the upstream Fabric example mod for 26.3 sets the mixin compatibility level to JAVA_25, and Fabric API 0.161.0+26.3 declares a dependency on java >=25. An older runtime cannot load those classes. This is a real break point for players and creators who updated the game but not the Java runtime.
- detectable: partial via `dependency.version_mismatch`

## missing_required_dependency_prevents_launch

**A declared required dependency that is not installed stops the game launching**

- symptom: The game does not reach the main menu. Fabric Loader reports the mod and the dependency it required, and refuses to continue.
- mechanism: fabric.mod.json distinguishes `depends` from `recommends`. `depends` is enforced by the loader at load time: the declared id must resolve to a loaded mod whose version satisfies the declared range. `recommends` is advisory and does not stop loading. A dependency can be satisfied by a module embedded in another mod's jar under META-INF/jars/, so a dependency that looks absent from the mods folder may in fact be present.
- detectable: yes via `dependency.missing`

## mojang_mappings_in_non_obfuscated_environment

**Build fails after updating to a non-obfuscated Minecraft version**

- symptom: Gradle fails while evaluating build.gradle with "Cannot use Mojang mappings in a non-obfuscated environment". No compilation happens.
- mechanism: Minecraft shipped obfuscated for most of its history, so Fabric builds declared a mappings dependency (loom.officialMojangMappings()) and used the remapping configuration modImplementation for mod dependencies. From the 26.x line the game is no longer obfuscated, so there is nothing to remap: Loom rejects a mappings declaration, and mod dependencies use plain implementation. A build script carried forward from a 1.21.x project therefore fails at configuration time. The upstream example mod's build script differs in exactly these two ways between its 1.21 and 26.3 branches.
- detectable: no via `None`
- resolution: remove_mappings_for_non_obfuscated_minecraft

## Resolutions

- `remove_mappings_for_non_obfuscated_minecraft` -- Drop the mappings declaration and use plain implementation (method: configuration_change)
