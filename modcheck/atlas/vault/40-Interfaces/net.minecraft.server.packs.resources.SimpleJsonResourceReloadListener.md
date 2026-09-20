---
type: "interface"
fqcn: "net.minecraft.server.packs.resources.SimpleJsonResourceReloadListener"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.packs.resources.SimpleJsonResourceReloadListener

System: [[20-Systems/net.minecraft.server.packs|net.minecraft.server.packs]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `lambda$prepare$0` | `@Inject at HEAD` | both | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |

## Declared members (11, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public abstract class net.minecraft.server.packs.resources.SimpleJsonResourceReloadListener<T> extends net.minecraft.server.packs.resources.SimplePreparableReloadListener<java.util.Map<net.minecraft.resources.Identifier, T>> {
    private static final org.slf4j.Logger LOGGER;
    private final com.mojang.serialization.DynamicOps<com.google.gson.JsonElement> ops;
    private final com.mojang.serialization.Codec<T> codec;
    private final net.minecraft.resources.FileToIdConverter lister;
    protected net.minecraft.server.packs.resources.SimpleJsonResourceReloadListener(com.mojang.serialization.Codec<T>, net.minecraft.resources.FileToIdConverter);
    private net.minecraft.server.packs.resources.SimpleJsonResourceReloadListener(com.mojang.serialization.DynamicOps<com.google.gson.JsonElement>, com.mojang.serialization.Codec<T>, net.minecraft.resources.FileToIdConverter);
    protected java.util.Map<net.minecraft.resources.Identifier, T> prepare(net.minecraft.server.packs.resources.ResourceManager, net.minecraft.util.profiling.ProfilerFiller);
    protected java.lang.Object prepare(net.minecraft.server.packs.resources.ResourceManager, net.minecraft.util.profiling.ProfilerFiller);
    private static void lambda$prepare$1(net.minecraft.resources.Identifier, net.minecraft.resources.Identifier, com.mojang.serialization.DataResult$Error);
    private static void lambda$prepare$0(java.util.Map, net.minecraft.resources.Identifier, java.lang.Object);
    static {};
}
```
