---
type: "interface"
fqcn: "net.minecraft.server.packs.resources.SimpleJsonResourceReloadListener"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.packs.resources.SimpleJsonResourceReloadListener

System: [[20-Systems/net.minecraft.server.packs|net.minecraft.server.packs]]

`abstract_class` public abstract; extends `net/minecraft/server/packs/resources/SimplePreparableReloadListener`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `lambda$prepare$0` | `(Ljava/util/Map;Lnet/minecraft/resources/Identifier;Ljava/lang/Object;` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| reads | `lister` | `Lnet/minecraft/resources/FileToIdConverter;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | declared |
| wraps | `prepare` | `?` | ambiguous | @WrapOperation at ['INVOKE'] | both | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |

## Declared members (4 fields, 7 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
private final ops : Lcom/mojang/serialization/DynamicOps;
private final codec : Lcom/mojang/serialization/Codec;
private final lister : Lnet/minecraft/resources/FileToIdConverter;
protected <init>(Lcom/mojang/serialization/Codec;Lnet/minecraft/resources/FileToIdConverter;)V
private <init>(Lcom/mojang/serialization/DynamicOps;Lcom/mojang/serialization/Codec;Lnet/minecraft/resources/FileToIdConverter;)V
protected prepare(Lnet/minecraft/server/packs/resources/ResourceManager;Lnet/minecraft/util/profiling/ProfilerFiller;)Ljava/util/Map;
protected synthetic prepare(Lnet/minecraft/server/packs/resources/ResourceManager;Lnet/minecraft/util/profiling/ProfilerFiller;)Ljava/lang/Object;
private static synthetic lambda$prepare$1(Lnet/minecraft/resources/Identifier;Lnet/minecraft/resources/Identifier;Lcom/mojang/serialization/DataResult$Error;)V
private static synthetic lambda$prepare$0(Ljava/util/Map;Lnet/minecraft/resources/Identifier;Ljava/lang/Object;)V
static <clinit>()V
```
