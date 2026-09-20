---
type: "interface"
fqcn: "net.minecraft.world.level.block.entity.BlockEntityType"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.block.entity.BlockEntityType

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Lnet/minecraft/world/level/block/entity/BlockEntityType$Blo` | `` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| calls | `create(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/blo` | `` | unknown | [[30-Mechanisms/fabric-api-lookup-api-v1|fabric-api-lookup-api-v1]] | direct_reference |
| calls | `onlyOpCanSetNbt()Z` | `` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| injects_into | `<init>` | `@Inject at RETURN` | both | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |

## Declared members (9, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.level.block.entity.BlockEntityType<T extends net.minecraft.world.level.block.entity.BlockEntity> {
    private final net.minecraft.world.level.block.entity.BlockEntityType$BlockEntitySupplier<? extends T> factory;
    private final java.util.Set<net.minecraft.world.level.block.Block> validBlocks;
    private final net.minecraft.core.Holder$Reference<net.minecraft.world.level.block.entity.BlockEntityType<?>> builtInRegistryHolder;
    public net.minecraft.world.level.block.entity.BlockEntityType(net.minecraft.world.level.block.entity.BlockEntityType$BlockEntitySupplier<? extends T>, java.util.Set<net.minecraft.world.level.block.Block>);
    public T create(net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState);
    public boolean isValid(net.minecraft.world.level.block.state.BlockState);
    public net.minecraft.core.Holder$Reference<net.minecraft.world.level.block.entity.BlockEntityType<?>> builtInRegistryHolder();
    public T getBlockEntity(net.minecraft.world.level.BlockGetter, net.minecraft.core.BlockPos);
    public boolean onlyOpCanSetNbt();
}
```
