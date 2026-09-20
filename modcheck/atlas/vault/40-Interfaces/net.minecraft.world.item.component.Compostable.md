---
type: "interface"
fqcn: "net.minecraft.world.item.component.Compostable"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.item.component.Compostable

System: [[20-Systems/net.minecraft.world.item|net.minecraft.world.item]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `layers` | `()Lnet/minecraft/world/level/storage/loot/providers/number/ints/Resolv` | exact | invokevirtual@114 in `ComposterWrapper$TopStorage.insert` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (3 fields, 8 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final layers : Lnet/minecraft/world/level/storage/loot/providers/number/ints/ResolvableInt;
public static final CODEC : Lcom/mojang/serialization/Codec;
public static final STREAM_CODEC : Lnet/minecraft/network/codec/StreamCodec;
public <init>(Lnet/minecraft/resources/ResourceKey;)V
public <init>(Lnet/minecraft/world/level/storage/loot/providers/number/ints/ResolvableInt;)V
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public layers()Lnet/minecraft/world/level/storage/loot/providers/number/ints/ResolvableInt;
private static synthetic lambda$static$0(Lcom/mojang/serialization/codecs/RecordCodecBuilder$Instance;)Lcom/mojang/datafixers/kinds/App;
static <clinit>()V
```
