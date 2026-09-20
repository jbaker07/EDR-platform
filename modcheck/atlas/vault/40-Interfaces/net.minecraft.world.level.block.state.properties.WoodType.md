---
type: "interface"
fqcn: "net.minecraft.world.level.block.state.properties.WoodType"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.block.state.properties.WoodType

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `name()Ljava/lang/String;` | `` | client | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| calls | `name()Ljava/lang/String;` | `` | client | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |

## Declared members (35, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.world.level.block.state.properties.WoodType extends java.lang.Record {
    private final java.lang.String name;
    private final net.minecraft.world.level.block.state.properties.BlockSetType setType;
    private final net.minecraft.world.level.block.SoundType soundType;
    private final net.minecraft.world.level.block.SoundType hangingSignSoundType;
    private final net.minecraft.sounds.SoundEvent fenceGateClose;
    private final net.minecraft.sounds.SoundEvent fenceGateOpen;
    private static final java.util.Map<java.lang.String, net.minecraft.world.level.block.state.properties.WoodType> TYPES;
    public static final com.mojang.serialization.Codec<net.minecraft.world.level.block.state.properties.WoodType> CODEC;
    public static final net.minecraft.world.level.block.state.properties.WoodType OAK;
    public static final net.minecraft.world.level.block.state.properties.WoodType SPRUCE;
    public static final net.minecraft.world.level.block.state.properties.WoodType BIRCH;
    public static final net.minecraft.world.level.block.state.properties.WoodType ACACIA;
    public static final net.minecraft.world.level.block.state.properties.WoodType CHERRY;
    public static final net.minecraft.world.level.block.state.properties.WoodType JUNGLE;
    public static final net.minecraft.world.level.block.state.properties.WoodType DARK_OAK;
    public static final net.minecraft.world.level.block.state.properties.WoodType PALE_OAK;
    public static final net.minecraft.world.level.block.state.properties.WoodType POPLAR;
    public static final net.minecraft.world.level.block.state.properties.WoodType CRIMSON;
    public static final net.minecraft.world.level.block.state.properties.WoodType WARPED;
    public static final net.minecraft.world.level.block.state.properties.WoodType MANGROVE;
    public static final net.minecraft.world.level.block.state.properties.WoodType BAMBOO;
    public net.minecraft.world.level.block.state.properties.WoodType(java.lang.String, net.minecraft.world.level.block.state.properties.BlockSetType);
    public net.minecraft.world.level.block.state.properties.WoodType(java.lang.String, net.minecraft.world.level.block.state.properties.BlockSetType, net.minecraft.world.level.block.SoundType, net.minecraft.world.level.block.SoundType, net.minecraft.sounds.SoundEvent, net.minecraft.sounds.SoundEvent);
    private static net.minecraft.world.level.block.state.properties.WoodType register(net.minecraft.world.level.block.state.properties.WoodType);
    public static java.util.stream.Stream<net.minecraft.world.level.block.state.properties.WoodType> values();
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public java.lang.String name();
    public net.minecraft.world.level.block.state.properties.BlockSetType setType();
    public net.minecraft.world.level.block.SoundType soundType();
    public net.minecraft.world.level.block.SoundType hangingSignSoundType();
    public net.minecraft.sounds.SoundEvent fenceGateClose();
    public net.minecraft.sounds.SoundEvent fenceGateOpen();
    static {};
}
```
