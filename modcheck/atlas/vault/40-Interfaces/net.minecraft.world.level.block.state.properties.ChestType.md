---
type: "interface"
fqcn: "net.minecraft.world.level.block.state.properties.ChestType"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.block.state.properties.ChestType

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`enum` public final; extends `java/lang/Enum`; implements `net/minecraft/util/StringRepresentable`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `SINGLE` | `Lnet/minecraft/world/level/block/state/properties/ChestType;` | exact | getstatic@46 in `ContainerSlotWrapper.updateSnapshots` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (6 fields, 7 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final SINGLE : Lnet/minecraft/world/level/block/state/properties/ChestType;
public static final LEFT : Lnet/minecraft/world/level/block/state/properties/ChestType;
public static final RIGHT : Lnet/minecraft/world/level/block/state/properties/ChestType;
public static final CODEC : Lcom/mojang/serialization/Codec;
private final name : Ljava/lang/String;
private static final synthetic $VALUES : [Lnet/minecraft/world/level/block/state/properties/ChestType;
public static values()[Lnet/minecraft/world/level/block/state/properties/ChestType;
public static valueOf(Ljava/lang/String;)Lnet/minecraft/world/level/block/state/properties/ChestType;
private <init>(Ljava/lang/String;ILjava/lang/String;)V
public getSerializedName()Ljava/lang/String;
public getOpposite()Lnet/minecraft/world/level/block/state/properties/ChestType;
private static synthetic $values()[Lnet/minecraft/world/level/block/state/properties/ChestType;
static <clinit>()V
```
