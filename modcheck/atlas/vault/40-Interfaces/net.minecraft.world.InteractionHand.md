---
type: "interface"
fqcn: "net.minecraft.world.InteractionHand"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.InteractionHand

System: [[20-Systems/net.minecraft.world|net.minecraft.world]]

`enum` public final; extends `java/lang/Enum`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `values` | `()[Lnet/minecraft/world/InteractionHand;` | exact | invokestatic@22 in `PlayerInventoryStorageImpl.offer` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `MAIN_HAND` | `Lnet/minecraft/world/InteractionHand;` | exact | getstatic@31 in `PlayerMixin.onPlayerInteractEntity` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| reads | `MAIN_HAND` | `Lnet/minecraft/world/InteractionHand;` | exact | getstatic@25 in `ServerPlayerGameModeMixin.startBlockBreak` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| reads | `MAIN_HAND` | `Lnet/minecraft/world/InteractionHand;` | exact | getstatic@23 in `MultiPlayerGameModeMixin.fabric_fireAttackBlockCallback` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| reads | `MAIN_HAND` | `Lnet/minecraft/world/InteractionHand;` | exact | getstatic@14 in `MultiPlayerGameModeMixin.attackEntity` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| reads | `MAIN_HAND` | `Lnet/minecraft/world/InteractionHand;` | exact | getstatic@27 in `FirstPersonHandsAndItemsMixin.modifyProgressAnimation` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| reads | `MAIN_HAND` | `Lnet/minecraft/world/InteractionHand;` | exact | getstatic@4 in `PlayerInventoryStorageImpl.getHandSlot` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `OFF_HAND` | `Lnet/minecraft/world/InteractionHand;` | exact | getstatic@75 in `FirstPersonHandsAndItemsMixin.modifyProgressAnimation` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| reads | `OFF_HAND` | `Lnet/minecraft/world/InteractionHand;` | exact | getstatic@56 in `PlayerInventoryStorageImpl.getHandSlot` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (6 fields, 9 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final MAIN_HAND : Lnet/minecraft/world/InteractionHand;
public static final OFF_HAND : Lnet/minecraft/world/InteractionHand;
private static final BY_ID : Ljava/util/function/IntFunction;
public static final STREAM_CODEC : Lnet/minecraft/network/codec/StreamCodec;
private final id : I
private static final synthetic $VALUES : [Lnet/minecraft/world/InteractionHand;
public static values()[Lnet/minecraft/world/InteractionHand;
public static valueOf(Ljava/lang/String;)Lnet/minecraft/world/InteractionHand;
private <init>(Ljava/lang/String;II)V
public asArm(Lnet/minecraft/world/entity/HumanoidArm;)Lnet/minecraft/world/entity/HumanoidArm;
public asEquipmentSlot()Lnet/minecraft/world/entity/EquipmentSlot;
private static synthetic $values()[Lnet/minecraft/world/InteractionHand;
private static synthetic lambda$static$1(Lnet/minecraft/world/InteractionHand;)I
private static synthetic lambda$static$0(Lnet/minecraft/world/InteractionHand;)I
static <clinit>()V
```
