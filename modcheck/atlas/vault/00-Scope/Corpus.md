---
type: "scope"
generated_at: "2026-09-20T07:32:22+00:00"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# Corpus: what was actually resolved

This is the concrete input set. Every fact in the atlas that is marked `direct_reference` or `static_inference` was read from one of these files, identified by hash. Anything not listed here is outside the corpus, and the atlas says so in [[00-Scope/Branches|Branches]].

**Source of truth:** jars resolved into the Gradle cache by the project's own build (build_workspaces/demo and reference/rainlantern); nothing fetched for this inventory

## Minecraft Java Edition 26.3 -- pinned identity

- version id `26.3`, world version 5023, protocol 777
- Java 25
- resource pack format 97.1, data pack format 121.0
- stable: True

## Artifacts (68)

| group | artifact | version | classes (top/nested) | sha256 |
|---|---|---|---|---|
| build_tooling | dev-launch-injector | 0.2.1+build.8 | 1/0 | `63bebba2a134d891` |
| build_tooling | fabric-log4j-util | 1.0.2 | 2/1 | `538e845a086d5830` |
| build_tooling | fabric-loom-native | 0.2.0 | 5/0 | `590f8966d3136d92` |
| build_tooling | lorenz-tiny | 4.0.2 | 6/3 | `9ea2b49f6149cf29` |
| build_tooling | mapping-io | 0.8.0 | 60/66 | `cf137c7f9049a8d4` |
| build_tooling | mercury | 0.4.3 | 3064/1199 | `25eac582a574329b` |
| build_tooling | mercurymixin | 0.2.2 | 18/1 | `85ec7d9edb40f90f` |
| build_tooling | net.fabricmc.unpick:unpick | 3.0.0-beta.13 | 52/29 | `5baf1cca1af85d25` |
| build_tooling | net.fabricmc.unpick:unpick-format-utils | 3.0.0-beta.13 | 28/26 | `7fb61b82db3f400a` |
| build_tooling | stitch | 0.6.2 | 50/31 | `3209f1fefba59f3d` |
| build_tooling | tiny-mappings-parser | 0.3.0+build.17 | 49/37 | `ab91038c4b1bff9b` |
| build_tooling | tiny-remapper | 0.14.0 | 83/61 | `0a86f606ca086bd7` |
| fabric_api_module | fabric-advancement-api-v1 | 1.0.0+38c7a2d55d | 12/3 | `89e2094ca63a5e3e` |
| fabric_api_module | fabric-api | 0.161.0+26.3 | 0/0 | `86f16178a3cecc88` |
| fabric_api_module | fabric-api-base | 2.0.6+fcdff87f5d | 16/1 | `88485b1edbcb642f` |
| fabric_api_module | fabric-api-lookup-api-v1 | 2.0.24+3434d6d95d | 27/7 | `4ff3be674760c602` |
| fabric_api_module | fabric-biome-api-v1 | 20.0.9+74ed1ea55d | 31/14 | `ce9698da6dd365c6` |
| fabric_api_module | fabric-block-api-v1 | 3.1.0+0ae6f9115d | 16/2 | `12df8ce066403f03` |
| fabric_api_module | fabric-block-getter-api-v2 | 2.0.9+3434d6d95d | 13/0 | `0f819114e0eb2d7e` |
| fabric_api_module | fabric-client-gametest-api-v1 | 6.0.7+4be74c3f5d | 79/9 | `09d7d48475eef47a` |
| fabric_api_module | fabric-command-api-v2 | 3.1.2+fcdff87f5d | 22/1 | `71e0ce2931b34674` |
| fabric_api_module | fabric-content-registries-v0 | 15.0.4+74ed1ea55d | 46/10 | `e83273ce3a8d06e0` |
| fabric_api_module | fabric-convention-tags-v2 | 4.10.3+6b5b47fd5d | 18/1 | `96fa76cd74df23aa` |
| fabric_api_module | fabric-crash-report-info-v1 | 1.0.7+fcdff87f5d | 5/1 | `8c89611954926210` |
| fabric_api_module | fabric-creative-tab-api-v1 | 5.0.21+fcdff87f5d | 18/6 | `415e659be69014ed` |
| fabric_api_module | fabric-data-attachment-api-v1 | 2.2.30+3434d6d95d | 50/12 | `916e1b1046113d52` |
| fabric_api_module | fabric-data-generation-api-v1 | 27.2.4+427eab975d | 76/27 | `2c22049cd3a75ddd` |
| fabric_api_module | fabric-debug-api-v1 | 1.0.4+3434d6d95d | 20/2 | `1ef63bc24f6319cc` |
| fabric_api_module | fabric-dimensions-v1 | 5.1.19+47f74c985d | 16/1 | `1b2d1d92f0e32719` |
| fabric_api_module | fabric-entity-events-v1 | 6.0.4+3434d6d95d | 30/32 | `a5a9e382e4f9875f` |
| fabric_api_module | fabric-events-interaction-v0 | 5.3.6+3434d6d95d | 34/15 | `f57dd8df1d78cbca` |
| fabric_api_module | fabric-game-rule-api-v1 | 4.0.10+3434d6d95d | 24/6 | `58266b2e28d20444` |
| fabric_api_module | fabric-gametest-api-v1 | 4.0.32+3434d6d95d | 15/2 | `1bd8282a95da3822` |
| fabric_api_module | fabric-item-api-v1 | 14.7.0+cf6bc2db5d | 52/13 | `aff8cffc3d6da547` |
| fabric_api_module | fabric-key-mapping-api-v1 | 2.0.8+3434d6d95d | 9/0 | `6ec1d5560d333f44` |
| fabric_api_module | fabric-lifecycle-events-v1 | 4.1.9+ffef5f675d | 50/39 | `b9ba49109cf21f96` |
| fabric_api_module | fabric-loot-api-v3 | 4.0.8+4068fd645d | 18/4 | `569540023c6d19e4` |
| fabric_api_module | fabric-menu-api-v1 | 2.0.27+ed66f0a35d | 11/2 | `2d406ae5995d2249` |
| fabric_api_module | fabric-message-api-v1 | 7.0.10+3434d6d95d | 12/21 | `967f819dc19e4e50` |
| fabric_api_module | fabric-model-loading-api-v1 | 8.0.35+fcdff87f5d | 35/26 | `4889e5947bb2f9d8` |
| fabric_api_module | fabric-networking-api-v1 | 6.3.8+fcdff87f5d | 114/55 | `dfff56a878bba654` |
| fabric_api_module | fabric-object-builder-api-v1 | 24.1.9+3434d6d95d | 35/11 | `3a5f0ccef4405528` |
| fabric_api_module | fabric-particles-v1 | 5.0.24+3434d6d95d | 32/9 | `0bf0c29bd7f1803e` |
| fabric_api_module | fabric-permission-api-v1 | 1.0.8+fcdff87f5d | 17/6 | `a3a82771b3fd9f2e` |
| fabric_api_module | fabric-recipe-api-v1 | 10.0.8+fcdff87f5d | 56/6 | `7d63b44a449ddd84` |
| fabric_api_module | fabric-registry-sync-v0 | 8.0.1+fcdff87f5d | 69/10 | `039a5c3dee042ff1` |
| fabric_api_module | fabric-renderer-api-v1 | 17.0.15+79385d0b5d | 79/13 | `2e4aaeb20f8615e8` |
| fabric_api_module | fabric-renderer-indigo | 9.1.23+b9d63e335d | 34/16 | `1e71ac3ddf0dd1d1` |
| fabric_api_module | fabric-rendering-fluids-v1 | 6.0.6+3434d6d95d | 10/2 | `ce4c943f676d1052` |
| fabric_api_module | fabric-rendering-v1 | 27.0.14+901a437c5d | 131/36 | `749427999b4845b1` |
| fabric_api_module | fabric-resource-conditions-api-v1 | 6.1.5+3434d6d95d | 28/2 | `1d7d9bea7e90eacf` |
| fabric_api_module | fabric-resource-loader-v0 | 3.3.26+4fc5413f5d | 10/1 | `18afa6466d69ff68` |
| fabric_api_module | fabric-resource-loader-v1 | 3.0.4+fcdff87f5d | 62/15 | `2d2fb907728c8956` |
| fabric_api_module | fabric-screen-api-v1 | 5.2.4+48607d035d | 19/30 | `6d0660544189cee8` |
| fabric_api_module | fabric-serialization-api-v1 | 2.0.7+74ed1ea55d | 12/2 | `eb0799d38e825cb6` |
| fabric_api_module | fabric-sound-api-v1 | 2.0.7+3434d6d95d | 5/0 | `d2c61f4bc0440251` |
| fabric_api_module | fabric-tag-api-v1 | 2.1.10+fcdff87f5d | 24/4 | `6c2fa7a4d870ee33` |
| fabric_api_module | fabric-transfer-api-v1 | 8.0.25+fcdff87f5d | 101/31 | `599f69de9e7e693b` |
| fabric_api_module | fabric-transitive-access-wideners-v1 | 8.1.16+52268a0e5d | 0/0 | `8bb488c84c37af37` |
| fabric_loader | fabric-loader | 0.19.5 | 464/304 | `93044e4dd46de5d8` |
| loader_runtime_dep | class-tweaker | 0.3.0-beta.2 | 28/21 | `47c5101083aa8d73` |
| minecraft | fabric-loom-1.17.21 | 26.3 | 460/304 | `89c08938d8656211` |
| minecraft | minecraft-client | 26.3 | 7301/4082 | `4508d006323f24fa` |
| minecraft | minecraft-extracted_server | 26.3 | 5037/2725 | `a362163eec5d1612` |
| minecraft | minecraft-merged | 26.3 | 7301/4082 | `5918174887871ab0` |
| minecraft | minecraft-merged-deobf-26.3 | 26.3 | 7301/4082 | `5918174887871ab0` |
| minecraft | minecraft-server | 26.3 | 1/3 | `d052f14d7a173734` |
| mixin_runtime | sponge-mixin | 0.17.4+mixin.0.8.7 | 574/453 | `1f0ae44db7295f86` |

## Groups

- **build_tooling**: 12
- **fabric_api_module**: 47
- **fabric_loader**: 1
- **loader_runtime_dep**: 1
- **minecraft**: 6
- **mixin_runtime**: 1
