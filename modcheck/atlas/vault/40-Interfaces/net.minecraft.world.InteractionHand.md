---
type: "interface"
fqcn: "net.minecraft.world.InteractionHand"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.InteractionHand

System: [[20-Systems/net.minecraft.world|net.minecraft.world]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `values()[Lnet/minecraft/world/InteractionHand;` | `` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `MAIN_HANDLnet/minecraft/world/InteractionHand;` | `` | both | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| reads | `MAIN_HANDLnet/minecraft/world/InteractionHand;` | `` | both | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| reads | `MAIN_HANDLnet/minecraft/world/InteractionHand;` | `` | client | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| reads | `MAIN_HANDLnet/minecraft/world/InteractionHand;` | `` | client | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| reads | `OFF_HANDLnet/minecraft/world/InteractionHand;` | `` | client | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |

## Declared members (15, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.world.InteractionHand extends java.lang.Enum<net.minecraft.world.InteractionHand> {
    public static final net.minecraft.world.InteractionHand MAIN_HAND;
    public static final net.minecraft.world.InteractionHand OFF_HAND;
    private static final java.util.function.IntFunction<net.minecraft.world.InteractionHand> BY_ID;
    public static final net.minecraft.network.codec.StreamCodec<io.netty.buffer.ByteBuf, net.minecraft.world.InteractionHand> STREAM_CODEC;
    private final int id;
    private static final net.minecraft.world.InteractionHand[] $VALUES;
    public static net.minecraft.world.InteractionHand[] values();
    public static net.minecraft.world.InteractionHand valueOf(java.lang.String);
    private net.minecraft.world.InteractionHand(int);
    public net.minecraft.world.entity.HumanoidArm asArm(net.minecraft.world.entity.HumanoidArm);
    public net.minecraft.world.entity.EquipmentSlot asEquipmentSlot();
    private static net.minecraft.world.InteractionHand[] $values();
    private static int lambda$static$1(net.minecraft.world.InteractionHand);
    private static int lambda$static$0(net.minecraft.world.InteractionHand);
    static {};
}
```
