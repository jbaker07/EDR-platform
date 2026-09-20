---
type: "interface"
fqcn: "net.minecraft.network.syncher.EntityDataSerializers"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.syncher.EntityDataSerializers

System: [[20-Systems/net.minecraft.network.syncher|net.minecraft.network.syncher]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `<clinit>` | `()V` | exact | @Inject at ['TAIL'] | both | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| injects_into | `registerSerializer` | `(Lnet/minecraft/network/syncher/EntityDataSerializer;)V` | exact | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |

## Declared members (47 fields, 5 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final SERIALIZERS : Lnet/minecraft/util/CrudeIncrementalIntIdentityHashBiMap;
public static final BYTE : Lnet/minecraft/network/syncher/EntityDataSerializer;
public static final INT : Lnet/minecraft/network/syncher/EntityDataSerializer;
public static final LONG : Lnet/minecraft/network/syncher/EntityDataSerializer;
public static final FLOAT : Lnet/minecraft/network/syncher/EntityDataSerializer;
public static final STRING : Lnet/minecraft/network/syncher/EntityDataSerializer;
public static final COMPONENT : Lnet/minecraft/network/syncher/EntityDataSerializer;
public static final OPTIONAL_COMPONENT : Lnet/minecraft/network/syncher/EntityDataSerializer;
public static final ITEM_STACK : Lnet/minecraft/network/syncher/EntityDataSerializer;
public static final BLOCK_STATE : Lnet/minecraft/network/syncher/EntityDataSerializer;
private static final OPTIONAL_BLOCK_STATE_CODEC : Lnet/minecraft/network/codec/StreamCodec;
public static final OPTIONAL_BLOCK_STATE : Lnet/minecraft/network/syncher/EntityDataSerializer;
public static final BOOLEAN : Lnet/minecraft/network/syncher/EntityDataSerializer;
public static final PARTICLE : Lnet/minecraft/network/syncher/EntityDataSerializer;
public static final PARTICLES : Lnet/minecraft/network/syncher/EntityDataSerializer;
public static final ROTATIONS : Lnet/minecraft/network/syncher/EntityDataSerializer;
public static final BLOCK_POS : Lnet/minecraft/network/syncher/EntityDataSerializer;
public static final OPTIONAL_BLOCK_POS : Lnet/minecraft/network/syncher/EntityDataSerializer;
public static final DIRECTION : Lnet/minecraft/network/syncher/EntityDataSerializer;
public static final OPTIONAL_LIVING_ENTITY_REFERENCE : Lnet/minecraft/network/syncher/EntityDataSerializer;
public static final OPTIONAL_GLOBAL_POS : Lnet/minecraft/network/syncher/EntityDataSerializer;
public static final VILLAGER_DATA : Lnet/minecraft/network/syncher/EntityDataSerializer;
private static final OPTIONAL_UNSIGNED_INT_CODEC : Lnet/minecraft/network/codec/StreamCodec;
public static final OPTIONAL_UNSIGNED_INT : Lnet/minecraft/network/syncher/EntityDataSerializer;
public static final POSE : Lnet/minecraft/network/syncher/EntityDataSerializer;
public static final CAT_VARIANT : Lnet/minecraft/network/syncher/EntityDataSerializer;
public static final CAT_SOUND_VARIANT : Lnet/minecraft/network/syncher/EntityDataSerializer;
public static final CHICKEN_VARIANT : Lnet/minecraft/network/syncher/EntityDataSerializer;
public static final CHICKEN_SOUND_VARIANT : Lnet/minecraft/network/syncher/EntityDataSerializer;
public static final COW_VARIANT : Lnet/minecraft/network/syncher/EntityDataSerializer;
public static final COW_SOUND_VARIANT : Lnet/minecraft/network/syncher/EntityDataSerializer;
public static final WOLF_VARIANT : Lnet/minecraft/network/syncher/EntityDataSerializer;
public static final WOLF_SOUND_VARIANT : Lnet/minecraft/network/syncher/EntityDataSerializer;
public static final FROG_VARIANT : Lnet/minecraft/network/syncher/EntityDataSerializer;
public static final PIG_VARIANT : Lnet/minecraft/network/syncher/EntityDataSerializer;
public static final PIG_SOUND_VARIANT : Lnet/minecraft/network/syncher/EntityDataSerializer;
public static final ZOMBIE_NAUTILUS_VARIANT : Lnet/minecraft/network/syncher/EntityDataSerializer;
public static final PAINTING_VARIANT : Lnet/minecraft/network/syncher/EntityDataSerializer;
public static final ARMADILLO_STATE : Lnet/minecraft/network/syncher/EntityDataSerializer;
public static final SNIFFER_STATE : Lnet/minecraft/network/syncher/EntityDataSerializer;
public static final WEATHERING_COPPER_STATE : Lnet/minecraft/network/syncher/EntityDataSerializer;
public static final COPPER_GOLEM_STATE : Lnet/minecraft/network/syncher/EntityDataSerializer;
public static final VECTOR3 : Lnet/minecraft/network/syncher/EntityDataSerializer;
public static final QUATERNION : Lnet/minecraft/network/syncher/EntityDataSerializer;
public static final RESOLVABLE_PROFILE : Lnet/minecraft/network/syncher/EntityDataSerializer;
public static final HUMANOID_ARM : Lnet/minecraft/network/syncher/EntityDataSerializer;
public static final DYE_COLOR : Lnet/minecraft/network/syncher/EntityDataSerializer;
public static registerSerializer(Lnet/minecraft/network/syncher/EntityDataSerializer;)V
public static getSerializer(I)Lnet/minecraft/network/syncher/EntityDataSerializer;
public static getSerializedId(Lnet/minecraft/network/syncher/EntityDataSerializer;)I
private <init>()V
static <clinit>()V
```
