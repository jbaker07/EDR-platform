---
type: "interface"
fqcn: "net.minecraft.world.level.levelgen.WorldGenSettings"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.levelgen.WorldGenSettings

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` public final; extends `net/minecraft/world/level/saveddata/SavedData`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `TYPE` | `Lnet/minecraft/world/level/saveddata/SavedDataType;` | exact | getstatic@88 in `CreateWorldScreenMixin.createLevelDataForServers` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (4 fields, 9 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final CODEC : Lcom/mojang/serialization/Codec;
public static final TYPE : Lnet/minecraft/world/level/saveddata/SavedDataType;
private final options : Lnet/minecraft/world/level/levelgen/WorldOptions;
private final dimensions : Lnet/minecraft/world/level/levelgen/WorldDimensions;
public <init>(Lnet/minecraft/world/level/levelgen/WorldOptions;Lnet/minecraft/world/level/levelgen/WorldDimensions;)V
public static of(Lnet/minecraft/world/level/levelgen/WorldOptions;Lnet/minecraft/core/RegistryAccess;)Lnet/minecraft/world/level/levelgen/WorldGenSettings;
public options()Lnet/minecraft/world/level/levelgen/WorldOptions;
public dimensions()Lnet/minecraft/world/level/levelgen/WorldDimensions;
public hashCode()I
public toString()Ljava/lang/String;
private static synthetic lambda$static$1()Lnet/minecraft/world/level/levelgen/WorldGenSettings;
private static synthetic lambda$static$0(Lcom/mojang/serialization/codecs/RecordCodecBuilder$Instance;)Lcom/mojang/datafixers/kinds/App;
static <clinit>()V
```
