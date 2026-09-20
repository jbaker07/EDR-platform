---
type: "interface"
fqcn: "net.minecraft.world.level.DataPackConfig"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.DataPackConfig

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Ljava/util/List;Ljava/util/List;)V` | exact | invokespecial@258 in `ModPackResourcesUtil.createDefaultDataConfiguration` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `<init>` | `(Ljava/util/List;Ljava/util/List;)V` | exact | invokespecial@110 in `ModPackResourcesUtil.createTestServerSettings` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `getDisabled` | `()Ljava/util/List;` | exact | invokevirtual@55 in `ModPackResourcesUtil.createDefaultDataConfiguration` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `getEnabled` | `()Ljava/util/List;` | exact | invokevirtual@41 in `ModPackResourcesUtil.createDefaultDataConfiguration` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| reads | `DEFAULT` | `Lnet/minecraft/world/level/DataPackConfig;` | exact | getstatic@38 in `ModPackResourcesUtil.createDefaultDataConfiguration` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| reads | `DEFAULT` | `Lnet/minecraft/world/level/DataPackConfig;` | exact | getstatic@52 in `ModPackResourcesUtil.createDefaultDataConfiguration` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (4 fields, 7 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final DEFAULT : Lnet/minecraft/world/level/DataPackConfig;
public static final CODEC : Lcom/mojang/serialization/Codec;
private final enabled : Ljava/util/List;
private final disabled : Ljava/util/List;
public <init>(Ljava/util/List;Ljava/util/List;)V
public getEnabled()Ljava/util/List;
public getDisabled()Ljava/util/List;
private static synthetic lambda$static$0(Lcom/mojang/serialization/codecs/RecordCodecBuilder$Instance;)Lcom/mojang/datafixers/kinds/App;
private static synthetic lambda$static$2(Lnet/minecraft/world/level/DataPackConfig;)Ljava/util/List;
private static synthetic lambda$static$1(Lnet/minecraft/world/level/DataPackConfig;)Ljava/util/List;
static <clinit>()V
```
