---
type: "interface"
fqcn: "net.minecraft.world.level.block.state.BlockState"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.block.state.BlockState

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `getBlock()Lnet/minecraft/world/level/block/Block;` | `` | unknown | [[30-Mechanisms/fabric-api-lookup-api-v1|fabric-api-lookup-api-v1]] | direct_reference |
| calls | `getBlock()Lnet/minecraft/world/level/block/Block;` | `` | unknown | [[30-Mechanisms/fabric-api-lookup-api-v1|fabric-api-lookup-api-v1]] | direct_reference |
| calls | `getBlock()Lnet/minecraft/world/level/block/Block;` | `` | both | [[30-Mechanisms/fabric-block-api-v1|fabric-block-api-v1]] | direct_reference |
| calls | `getBlock()Lnet/minecraft/world/level/block/Block;` | `` | both | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `getBlock()Lnet/minecraft/world/level/block/Block;` | `` | both | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| calls | `getBlock()Lnet/minecraft/world/level/block/Block;` | `` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| calls | `getBlock()Lnet/minecraft/world/level/block/Block;` | `` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `getBlock()Lnet/minecraft/world/level/block/Block;` | `` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getBlock()Lnet/minecraft/world/level/block/Block;` | `` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getLightEmission()I` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `getLightEmission()I` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `getOffset(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/phys/Vec` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `getProvidedEnchantmentPower(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/` | `` | both | [[30-Mechanisms/fabric-block-api-v1|fabric-block-api-v1]] | direct_reference |
| calls | `getSeed(Lnet/minecraft/core/BlockPos;)J` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `getSeed(Lnet/minecraft/core/BlockPos;)J` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `getShadeBrightness(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `getValue(Lnet/minecraft/world/level/block/state/properties/Property;` | `` | both | [[30-Mechanisms/fabric-block-api-v1|fabric-block-api-v1]] | direct_reference |
| calls | `getValue(Lnet/minecraft/world/level/block/state/properties/Property;` | `` | both | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `getValue(Lnet/minecraft/world/level/block/state/properties/Property;` | `` | both | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| calls | `getValue(Lnet/minecraft/world/level/block/state/properties/Property;` | `` | both | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| calls | `getValue(Lnet/minecraft/world/level/block/state/properties/Property;` | `` | both | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getValue(Lnet/minecraft/world/level/block/state/properties/Property;` | `` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getValue(Lnet/minecraft/world/level/block/state/properties/Property;` | `` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `hasBlockEntity()Z` | `` | unknown | [[30-Mechanisms/fabric-api-lookup-api-v1|fabric-api-lookup-api-v1]] | direct_reference |
| calls | `hasBlockEntity()Z` | `` | both | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| calls | `hasProperty(Lnet/minecraft/world/level/block/state/properties/Property;` | `` | both | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `hasProperty(Lnet/minecraft/world/level/block/state/properties/Property;` | `` | both | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| calls | `hasProperty(Lnet/minecraft/world/level/block/state/properties/Property;` | `` | both | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| calls | `initCache()V` | `` | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `is(Ljava/lang/Object;)Z` | `` | both | [[30-Mechanisms/fabric-block-api-v1|fabric-block-api-v1]] | direct_reference |
| calls | `is(Ljava/lang/Object;)Z` | `` | both | [[30-Mechanisms/fabric-block-api-v1|fabric-block-api-v1]] | direct_reference |
| calls | `is(Lnet/minecraft/tags/TagKey;)Z` | `` | both | [[30-Mechanisms/fabric-block-api-v1|fabric-block-api-v1]] | direct_reference |
| calls | `isAir()Z` | `` | both | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |
| calls | `isCollisionShapeFullBlock(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `isCollisionShapeFullBlock(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `isLightPermeable()Z` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `isSolidRender()Z` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `setValue(Lnet/minecraft/world/level/block/state/properties/Property;` | `` | both | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| calls | `setValue(Lnet/minecraft/world/level/block/state/properties/Property;` | `` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `setValue(Lnet/minecraft/world/level/block/state/properties/Property;` | `` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `toString()Ljava/lang/String;` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |

## Declared members (9, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.level.block.state.BlockState extends net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase {
    public static final com.mojang.serialization.Codec<net.minecraft.world.level.block.state.BlockState> FULL_CODEC;
    private static final com.mojang.serialization.Codec<com.mojang.datafixers.util.Either<net.minecraft.world.level.block.Block, net.minecraft.world.level.block.state.BlockState>> CONSTANT_OR_DISPATCH_CODEC;
    public static final com.mojang.serialization.Codec<net.minecraft.world.level.block.state.BlockState> CODEC;
    public net.minecraft.world.level.block.state.BlockState(net.minecraft.world.level.block.Block, net.minecraft.world.level.block.state.properties.Property<?>[], java.lang.Comparable<?>[]);
    protected net.minecraft.world.level.block.state.BlockState asState();
    private static com.mojang.datafixers.util.Either lambda$static$2(net.minecraft.world.level.block.state.BlockState);
    private static net.minecraft.world.level.block.state.BlockState lambda$static$0(com.mojang.datafixers.util.Either);
    private static net.minecraft.world.level.block.state.BlockState lambda$static$1(net.minecraft.world.level.block.state.BlockState);
    static {};
}
```
