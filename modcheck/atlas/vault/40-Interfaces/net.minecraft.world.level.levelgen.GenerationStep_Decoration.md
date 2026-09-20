---
type: "interface"
fqcn: "net.minecraft.world.level.levelgen.GenerationStep$Decoration"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.levelgen.GenerationStep$Decoration

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`enum` public final; extends `java/lang/Enum`; implements `net/minecraft/util/StringRepresentable`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `ordinal` | `()I` | inherited_exact | invokevirtual@16 in `BiomeModificationContextImpl$GenerationSettingsContextImpl.removeFeat | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `ordinal` | `()I` | inherited_exact | invokevirtual@9 in `BiomeModificationContextImpl$GenerationSettingsContextImpl.addFeature` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `values` | `()[Lnet/minecraft/world/level/levelgen/GenerationStep$Decoration;` | exact | invokestatic@2 in `BiomeModificationContext$GenerationSettingsContext.removeFeature` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |

## Declared members (14 fields, 7 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final RAW_GENERATION : Lnet/minecraft/world/level/levelgen/GenerationStep$Decoration;
public static final LAKES : Lnet/minecraft/world/level/levelgen/GenerationStep$Decoration;
public static final LOCAL_MODIFICATIONS : Lnet/minecraft/world/level/levelgen/GenerationStep$Decoration;
public static final UNDERGROUND_STRUCTURES : Lnet/minecraft/world/level/levelgen/GenerationStep$Decoration;
public static final SURFACE_STRUCTURES : Lnet/minecraft/world/level/levelgen/GenerationStep$Decoration;
public static final STRONGHOLDS : Lnet/minecraft/world/level/levelgen/GenerationStep$Decoration;
public static final UNDERGROUND_ORES : Lnet/minecraft/world/level/levelgen/GenerationStep$Decoration;
public static final UNDERGROUND_DECORATION : Lnet/minecraft/world/level/levelgen/GenerationStep$Decoration;
public static final FLUID_SPRINGS : Lnet/minecraft/world/level/levelgen/GenerationStep$Decoration;
public static final VEGETAL_DECORATION : Lnet/minecraft/world/level/levelgen/GenerationStep$Decoration;
public static final TOP_LAYER_MODIFICATION : Lnet/minecraft/world/level/levelgen/GenerationStep$Decoration;
public static final CODEC : Lcom/mojang/serialization/Codec;
private final name : Ljava/lang/String;
private static final synthetic $VALUES : [Lnet/minecraft/world/level/levelgen/GenerationStep$Decoration;
public static values()[Lnet/minecraft/world/level/levelgen/GenerationStep$Decoration;
public static valueOf(Ljava/lang/String;)Lnet/minecraft/world/level/levelgen/GenerationStep$Decoration;
private <init>(Ljava/lang/String;ILjava/lang/String;)V
public getName()Ljava/lang/String;
public getSerializedName()Ljava/lang/String;
private static synthetic $values()[Lnet/minecraft/world/level/levelgen/GenerationStep$Decoration;
static <clinit>()V
```
