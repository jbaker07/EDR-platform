---
type: "scope"
generated_at: "2026-09-20T08:55:31+00:00"
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

## Cache inventory versus the resolved environment

Two different things, kept apart. The **cache inventory** is every jar in the Gradle caches that this project's tooling downloaded. The **resolved environment** is what the reference project's own Gradle build put on its classpaths (``extracted/resolved_environment.json``, gradle --offline printResolvedClasspaths (init script in atlas/README.md); every entry hashed).

| configuration | entries |
|---|---|
| compileClasspath | 114 |
| runtimeClasspath | 126 |
| testRuntimeClasspath | 134 |

- corpus artifacts on the runtime classpath: 126 / 144
- runtime entries outside the corpus: 0

### The jar a mod compiles against is not Mojang's jar

The compile classpath carries a Loom-processed jar (`minecraft-merged-7e9a32a5b8-26.3.jar`, sha256 `97a090f2e55dbcee`), not the merged cache jar (`minecraft-merged.jar`, `5918174887871ab0`). Same 34225 entries; 363 classes differ: 771 member access changes, 13 class access changes, 88 hierarchy changes (interfaces injected). Details: [[00-Scope/Processed_Jar_Diff]]. Every member resolution in the atlas is against the processed jar, because that is what a mod resolves against; which module widened what is [[80-Unresolved/q.processed_jar_change_provenance|q.processed_jar_change_provenance]].

## Artifacts (144)

| group | artifact | version | classes (top/nested) | compile | runtime | sha256 |
|---|---|---|---|---|---|---|
| build_tooling | dev-launch-injector | 0.2.1+build.8 | 1/0 |  | yes | `63bebba2a134d891` |
| build_tooling | fabric-log4j-util | 1.0.2 | 2/1 |  | yes | `538e845a086d5830` |
| build_tooling | fabric-loom-native | 0.2.0 | 5/0 |  |  | `590f8966d3136d92` |
| build_tooling | lorenz-tiny | 4.0.2 | 6/3 |  |  | `9ea2b49f6149cf29` |
| build_tooling | mapping-io | 0.8.0 | 60/66 |  |  | `cf137c7f9049a8d4` |
| build_tooling | mercury | 0.4.3 | 3064/1199 |  |  | `25eac582a574329b` |
| build_tooling | mercurymixin | 0.2.2 | 18/1 |  |  | `85ec7d9edb40f90f` |
| build_tooling | net.fabricmc.unpick:unpick | 3.0.0-beta.13 | 52/29 |  |  | `5baf1cca1af85d25` |
| build_tooling | net.fabricmc.unpick:unpick-format-utils | 3.0.0-beta.13 | 28/26 |  |  | `7fb61b82db3f400a` |
| build_tooling | stitch | 0.6.2 | 50/31 |  |  | `3209f1fefba59f3d` |
| build_tooling | tiny-mappings-parser | 0.3.0+build.17 | 49/37 |  |  | `ab91038c4b1bff9b` |
| build_tooling | tiny-remapper | 0.14.0 | 83/61 |  |  | `0a86f606ca086bd7` |
| fabric_api_module | fabric-advancement-api-v1 | 1.0.0+38c7a2d55d | 12/3 | yes | yes | `89e2094ca63a5e3e` |
| fabric_api_module | fabric-api | 0.161.0+26.3 | 0/0 | yes | yes | `86f16178a3cecc88` |
| fabric_api_module | fabric-api-base | 2.0.6+fcdff87f5d | 16/1 | yes | yes | `88485b1edbcb642f` |
| fabric_api_module | fabric-api-lookup-api-v1 | 2.0.24+3434d6d95d | 27/7 | yes | yes | `4ff3be674760c602` |
| fabric_api_module | fabric-biome-api-v1 | 20.0.9+74ed1ea55d | 31/14 | yes | yes | `ce9698da6dd365c6` |
| fabric_api_module | fabric-block-api-v1 | 3.1.0+0ae6f9115d | 16/2 | yes | yes | `12df8ce066403f03` |
| fabric_api_module | fabric-block-getter-api-v2 | 2.0.9+3434d6d95d | 13/0 | yes | yes | `0f819114e0eb2d7e` |
| fabric_api_module | fabric-client-gametest-api-v1 | 6.0.7+4be74c3f5d | 79/9 | yes | yes | `09d7d48475eef47a` |
| fabric_api_module | fabric-command-api-v2 | 3.1.2+fcdff87f5d | 22/1 | yes | yes | `71e0ce2931b34674` |
| fabric_api_module | fabric-content-registries-v0 | 15.0.4+74ed1ea55d | 46/10 | yes | yes | `e83273ce3a8d06e0` |
| fabric_api_module | fabric-convention-tags-v2 | 4.10.3+6b5b47fd5d | 18/1 | yes | yes | `96fa76cd74df23aa` |
| fabric_api_module | fabric-crash-report-info-v1 | 1.0.7+fcdff87f5d | 5/1 | yes | yes | `8c89611954926210` |
| fabric_api_module | fabric-creative-tab-api-v1 | 5.0.21+fcdff87f5d | 18/6 | yes | yes | `415e659be69014ed` |
| fabric_api_module | fabric-data-attachment-api-v1 | 2.2.30+3434d6d95d | 50/12 | yes | yes | `916e1b1046113d52` |
| fabric_api_module | fabric-data-generation-api-v1 | 27.2.4+427eab975d | 76/27 | yes | yes | `2c22049cd3a75ddd` |
| fabric_api_module | fabric-debug-api-v1 | 1.0.4+3434d6d95d | 20/2 | yes | yes | `1ef63bc24f6319cc` |
| fabric_api_module | fabric-dimensions-v1 | 5.1.19+47f74c985d | 16/1 | yes | yes | `1b2d1d92f0e32719` |
| fabric_api_module | fabric-entity-events-v1 | 6.0.4+3434d6d95d | 30/32 | yes | yes | `a5a9e382e4f9875f` |
| fabric_api_module | fabric-events-interaction-v0 | 5.3.6+3434d6d95d | 34/15 | yes | yes | `f57dd8df1d78cbca` |
| fabric_api_module | fabric-game-rule-api-v1 | 4.0.10+3434d6d95d | 24/6 | yes | yes | `58266b2e28d20444` |
| fabric_api_module | fabric-gametest-api-v1 | 4.0.32+3434d6d95d | 15/2 | yes | yes | `1bd8282a95da3822` |
| fabric_api_module | fabric-item-api-v1 | 14.7.0+cf6bc2db5d | 52/13 | yes | yes | `aff8cffc3d6da547` |
| fabric_api_module | fabric-key-mapping-api-v1 | 2.0.8+3434d6d95d | 9/0 | yes | yes | `6ec1d5560d333f44` |
| fabric_api_module | fabric-lifecycle-events-v1 | 4.1.9+ffef5f675d | 50/39 | yes | yes | `b9ba49109cf21f96` |
| fabric_api_module | fabric-loot-api-v3 | 4.0.8+4068fd645d | 18/4 | yes | yes | `569540023c6d19e4` |
| fabric_api_module | fabric-menu-api-v1 | 2.0.27+ed66f0a35d | 11/2 | yes | yes | `2d406ae5995d2249` |
| fabric_api_module | fabric-message-api-v1 | 7.0.10+3434d6d95d | 12/21 | yes | yes | `967f819dc19e4e50` |
| fabric_api_module | fabric-model-loading-api-v1 | 8.0.35+fcdff87f5d | 35/26 | yes | yes | `4889e5947bb2f9d8` |
| fabric_api_module | fabric-networking-api-v1 | 6.3.8+fcdff87f5d | 114/55 | yes | yes | `dfff56a878bba654` |
| fabric_api_module | fabric-object-builder-api-v1 | 24.1.9+3434d6d95d | 35/11 | yes | yes | `3a5f0ccef4405528` |
| fabric_api_module | fabric-particles-v1 | 5.0.24+3434d6d95d | 32/9 | yes | yes | `0bf0c29bd7f1803e` |
| fabric_api_module | fabric-permission-api-v1 | 1.0.8+fcdff87f5d | 17/6 | yes | yes | `a3a82771b3fd9f2e` |
| fabric_api_module | fabric-recipe-api-v1 | 10.0.8+fcdff87f5d | 56/6 | yes | yes | `7d63b44a449ddd84` |
| fabric_api_module | fabric-registry-sync-v0 | 8.0.1+fcdff87f5d | 69/10 | yes | yes | `039a5c3dee042ff1` |
| fabric_api_module | fabric-renderer-api-v1 | 17.0.15+79385d0b5d | 79/13 | yes | yes | `2e4aaeb20f8615e8` |
| fabric_api_module | fabric-renderer-indigo | 9.1.23+b9d63e335d | 34/16 | yes | yes | `1e71ac3ddf0dd1d1` |
| fabric_api_module | fabric-rendering-fluids-v1 | 6.0.6+3434d6d95d | 10/2 | yes | yes | `ce4c943f676d1052` |
| fabric_api_module | fabric-rendering-v1 | 27.0.14+901a437c5d | 131/36 | yes | yes | `749427999b4845b1` |
| fabric_api_module | fabric-resource-conditions-api-v1 | 6.1.5+3434d6d95d | 28/2 | yes | yes | `1d7d9bea7e90eacf` |
| fabric_api_module | fabric-resource-loader-v0 | 3.3.26+4fc5413f5d | 10/1 | yes | yes | `18afa6466d69ff68` |
| fabric_api_module | fabric-resource-loader-v1 | 3.0.4+fcdff87f5d | 62/15 | yes | yes | `2d2fb907728c8956` |
| fabric_api_module | fabric-screen-api-v1 | 5.2.4+48607d035d | 19/30 | yes | yes | `6d0660544189cee8` |
| fabric_api_module | fabric-serialization-api-v1 | 2.0.7+74ed1ea55d | 12/2 | yes | yes | `eb0799d38e825cb6` |
| fabric_api_module | fabric-sound-api-v1 | 2.0.7+3434d6d95d | 5/0 | yes | yes | `d2c61f4bc0440251` |
| fabric_api_module | fabric-tag-api-v1 | 2.1.10+fcdff87f5d | 24/4 | yes | yes | `6c2fa7a4d870ee33` |
| fabric_api_module | fabric-transfer-api-v1 | 8.0.25+fcdff87f5d | 101/31 | yes | yes | `599f69de9e7e693b` |
| fabric_api_module | fabric-transitive-access-wideners-v1 | 8.1.16+52268a0e5d | 0/0 | yes | yes | `8bb488c84c37af37` |
| fabric_loader | fabric-loader | 0.19.5 | 464/304 | yes | yes | `93044e4dd46de5d8` |
| loader_runtime_dep | class-tweaker | 0.3.0-beta.2 | 28/21 |  |  | `47c5101083aa8d73` |
| minecraft | fabric-loom-1.17.21 | 26.3 | 460/304 |  |  | `89c08938d8656211` |
| minecraft | minecraft-client | 26.3 | 7301/4082 |  |  | `4508d006323f24fa` |
| minecraft | minecraft-extracted_server | 26.3 | 5037/2725 |  |  | `a362163eec5d1612` |
| minecraft | minecraft-merged | 26.3 | 7301/4082 |  |  | `5918174887871ab0` |
| minecraft | minecraft-merged-deobf-26.3 | 26.3 | 7301/4082 |  |  | `5918174887871ab0` |
| minecraft | minecraft-server | 26.3 | 1/3 |  |  | `d052f14d7a173734` |
| minecraft_library | at.yawk.lz4:lz4-java | 1.10.1 | 57/25 | yes | yes | `a58a84c4271e50df` |
| minecraft_library | com.azure:azure-json | 1.4.0 | 107/15 | yes | yes | `c50bc998cd1a6c68` |
| minecraft_library | com.github.oshi:oshi-core | 6.9.0 | 402/186 | yes | yes | `b29ba9bce041e6ea` |
| minecraft_library | com.google.code.gson:gson | 2.14.0 | 87/135 | yes | yes | `2cbd119bf1961c28` |
| minecraft_library | com.google.guava:failureaccess | 1.0.3 | 3/0 | yes | yes | `cbfc3906b19b8f55` |
| minecraft_library | com.google.guava:guava | 33.6.0-jre | 609/1361 | yes | yes | `dc573e1fca4fd545` |
| minecraft_library | com.ibm.icu:icu4j | 78.3 | 629/1041 | yes | yes | `e962c1758d9659ea` |
| minecraft_library | com.microsoft.azure:msal4j | 1.24.1 | 196/55 | yes | yes | `12e62f79d1a71d17` |
| minecraft_library | com.mojang:authlib | 10.0.77 | 87/36 | yes | yes | `521672866582633a` |
| minecraft_library | com.mojang:blocklist | 1.0.10 | 1/0 | yes | yes | `830bfd639c8db492` |
| minecraft_library | com.mojang:brigadier | 1.3.11 | 47/7 | yes | yes | `9d09cf4819de83df` |
| minecraft_library | com.mojang:datafixerupper | 10.0.21 | 164/320 | yes | yes | `3547cb1da8993fb7` |
| minecraft_library | com.mojang:jtracy | 1.14.38 | 12/1 | yes | yes | `721bff502a9664d6` |
| minecraft_library | com.mojang:jtracy | 1.14.38 | 0/0 |  | yes | `6c64f455ebaa5f66` |
| minecraft_library | com.mojang:logging | 1.7.12 | 8/4 | yes | yes | `3c7ac9742fa98c77` |
| minecraft_library | com.mojang:patchy | 2.2.10 | 2/0 | yes | yes | `16d70e7968b45caf` |
| minecraft_library | com.mojang:text2speech | 1.19.12 | 5/6 | yes | yes | `63769a6f662ad894` |
| minecraft_library | commons-codec:commons-codec | 1.22.0 | 88/63 | yes | yes | `d164fe79f262c32d` |
| minecraft_library | commons-io:commons-io | 2.20.0 | 266/122 | yes | yes | `df90bba0fe3cb586` |
| minecraft_library | io.netty:netty-buffer | 4.2.16.Final | 92/84 | yes | yes | `cc36ae9fbd0b03fe` |
| minecraft_library | io.netty:netty-codec-base | 4.2.16.Final | 67/30 | yes | yes | `feb410225938d997` |
| minecraft_library | io.netty:netty-codec-compression | 4.2.16.Final | 61/38 | yes | yes | `13c8565414dd2115` |
| minecraft_library | io.netty:netty-codec-http | 4.2.16.Final | 254/127 | yes | yes | `0d16f981de8d0d30` |
| minecraft_library | io.netty:netty-common | 4.2.16.Final | 314/286 | yes | yes | `9825ee68a0dc4cd2` |
| minecraft_library | io.netty:netty-handler | 4.2.16.Final | 166/173 | yes | yes | `a259ca496da05ac1` |
| minecraft_library | io.netty:netty-resolver | 4.2.16.Final | 20/3 | yes | yes | `c9eca6a99036485c` |
| minecraft_library | io.netty:netty-transport | 4.2.16.Final | 197/192 | yes | yes | `cfa3f654caff9066` |
| minecraft_library | io.netty:netty-transport-classes-epoll | 4.2.16.Final | 38/38 | yes | yes | `9f2b53210db9c8d2` |
| minecraft_library | io.netty:netty-transport-classes-kqueue | 4.2.16.Final | 33/22 | yes | yes | `6599b495f69420cb` |
| minecraft_library | io.netty:netty-transport-native-epoll | 4.2.16.Final | 1/0 | yes | yes | `2e00548e73194888` |
| minecraft_library | io.netty:netty-transport-native-epoll | 4.2.16.Final | 1/0 | yes | yes | `212cc7d81390e31b` |
| minecraft_library | io.netty:netty-transport-native-kqueue | 4.2.16.Final | 1/0 | yes | yes | `05a09a4ab51f488f` |
| minecraft_library | io.netty:netty-transport-native-kqueue | 4.2.16.Final | 1/0 | yes | yes | `796c503f7a802171` |
| minecraft_library | io.netty:netty-transport-native-unix-common | 4.2.16.Final | 31/2 | yes | yes | `41ca8fe192083d17` |
| minecraft_library | it.unimi.dsi:fastutil | 8.5.18 | 2411/10554 | yes | yes | `9094ae67d01d0ad2` |
| minecraft_library | net.java.dev.jna:jna | 5.17.0 | 65/59 | yes | yes | `b3a9408e7c51e08e` |
| minecraft_library | net.java.dev.jna:jna-platform | 5.17.0 | 241/1044 | yes | yes | `b7e3d46c87bad2eb` |
| minecraft_library | net.sf.jopt-simple:jopt-simple | 5.0.4 | 50/9 | yes | yes | `df26cc58f235f477` |
| minecraft_library | org.apache.commons:commons-compress | 1.28.0 | 409/181 | yes | yes | `e1522945218456f3` |
| minecraft_library | org.apache.commons:commons-lang3 | 3.20.0 | 260/162 | yes | yes | `69e5c9fa35da7a51` |
| minecraft_library | org.apache.logging.log4j:log4j-api | 2.26.0 | 155/59 | yes | yes | `820de7bbaf430eaf` |
| minecraft_library | org.apache.logging.log4j:log4j-core | 2.26.0 | 747/512 | yes | yes | `406bb3132c47a5d3` |
| minecraft_library | org.apache.logging.log4j:log4j-slf4j2-impl | 2.26.0 | 12/2 | yes | yes | `cfc1bddbd39e684b` |
| minecraft_library | org.jcraft:jorbis | 0.0.17 | 33/13 | yes | yes | `e067f29a3701426b` |
| minecraft_library | org.jetbrains:annotations | 26.0.2 | 33/28 | yes |  | `2037be378980d3ba` |
| minecraft_library | org.joml:joml | 1.10.9 | 111/32 | yes | yes | `feca4db853371704` |
| minecraft_library | org.jspecify:jspecify | 1.0.0 | 5/0 | yes | yes | `1fad6e6be7557781` |
| minecraft_library | org.lwjgl:lwjgl | 3.4.3 | 1/0 |  | yes | `d719e545a6db4ea8` |
| minecraft_library | org.lwjgl:lwjgl | 3.4.3 | 285/255 | yes | yes | `46eeca5471833c3c` |
| minecraft_library | org.lwjgl:lwjgl-freetype | 3.4.3 | 1/0 |  | yes | `cbc702aa1d9c8fec` |
| minecraft_library | org.lwjgl:lwjgl-freetype | 3.4.3 | 149/120 | yes | yes | `b331b9d2a29c1b33` |
| minecraft_library | org.lwjgl:lwjgl-jemalloc | 3.4.3 | 22/11 | yes | yes | `f34062b6489454f7` |
| minecraft_library | org.lwjgl:lwjgl-jemalloc | 3.4.3 | 1/0 |  | yes | `1c7fc271f132e3d6` |
| minecraft_library | org.lwjgl:lwjgl-openal | 3.4.3 | 82/13 | yes | yes | `4a0049c8554d6890` |
| minecraft_library | org.lwjgl:lwjgl-openal | 3.4.3 | 1/0 |  | yes | `433cbf19b16b177d` |
| minecraft_library | org.lwjgl:lwjgl-opengl | 3.4.3 | 480/13 | yes | yes | `a92cf30f25b6fb60` |
| minecraft_library | org.lwjgl:lwjgl-opengl | 3.4.3 | 1/0 |  | yes | `4056e81c1174201f` |
| minecraft_library | org.lwjgl:lwjgl-sdl | 3.4.3 | 1/0 |  | yes | `e1880683a4af3329` |
| minecraft_library | org.lwjgl:lwjgl-sdl | 3.4.3 | 288/232 | yes | yes | `dcb33abcbd946553` |
| minecraft_library | org.lwjgl:lwjgl-shaderc | 3.4.3 | 1/0 |  | yes | `0dced6d3362cbcd6` |
| minecraft_library | org.lwjgl:lwjgl-shaderc | 3.4.3 | 38/28 | yes | yes | `82c7b5270d2c7845` |
| minecraft_library | org.lwjgl:lwjgl-spvc | 3.4.3 | 1/0 |  | yes | `8898165a90911bfd` |
| minecraft_library | org.lwjgl:lwjgl-spvc | 3.4.3 | 24/20 | yes | yes | `d737eba8512caa2f` |
| minecraft_library | org.lwjgl:lwjgl-stb | 3.4.3 | 1/0 |  | yes | `5c66a2f11b18062b` |
| minecraft_library | org.lwjgl:lwjgl-stb | 3.4.3 | 47/26 | yes | yes | `544753f5e49c5358` |
| minecraft_library | org.lwjgl:lwjgl-vma | 3.4.3 | 1/0 |  | yes | `f8690711c842de5a` |
| minecraft_library | org.lwjgl:lwjgl-vma | 3.4.3 | 29/20 | yes | yes | `254162ef94bd2136` |
| minecraft_library | org.lwjgl:lwjgl-vulkan | 3.4.3 | 2281/1791 | yes | yes | `acdb40767775ed12` |
| minecraft_library | org.ow2.asm:asm | 9.10.1 | 36/3 | yes | yes | `ed825d10ab1399c8` |
| minecraft_library | org.ow2.asm:asm-analysis | 9.10.1 | 14/1 | yes | yes | `dede75a21306b659` |
| minecraft_library | org.ow2.asm:asm-commons | 9.10.1 | 25/3 | yes | yes | `6d0abefb7cbf972e` |
| minecraft_library | org.ow2.asm:asm-tree | 9.10.1 | 37/2 | yes | yes | `3dfb0d5b6a106cd4` |
| minecraft_library | org.ow2.asm:asm-util | 9.10.1 | 21/7 | yes | yes | `1bb99d091fba2597` |
| minecraft_library | org.slf4j:slf4j-api | 2.0.17 | 48/8 | yes | yes | `7b751d952061954d` |
| minecraft_processed | minecraft-merged-7e9a32a5b8-26.3 | 26.3 | 7301/4082 | yes | yes | `97a090f2e55dbcee` |
| mixin_runtime | sponge-mixin | 0.17.4+mixin.0.8.7 | 574/453 | yes | yes | `1f0ae44db7295f86` |
| mixin_runtime_extras | mixinextras-fabric | 0.5.5 | 353/153 | yes | yes | `5da883dc4bfb16e4` |

## Groups

- **build_tooling**: 12
- **fabric_api_module**: 47
- **fabric_loader**: 1
- **loader_runtime_dep**: 1
- **minecraft**: 6
- **minecraft_library**: 74
- **minecraft_processed**: 1
- **mixin_runtime**: 1
- **mixin_runtime_extras**: 1
