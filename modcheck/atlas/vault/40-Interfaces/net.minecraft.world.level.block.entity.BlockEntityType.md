---
type: "interface"
fqcn: "net.minecraft.world.level.block.entity.BlockEntityType"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.block.entity.BlockEntityType

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` public; extends `java/lang/Object`; implements `net/fabricmc/fabric/api/object/builder/v1/block/entity/FabricBlockEntityType`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/world/level/block/entity/BlockEntityType$BlockEntitySu` | exact | invokespecial@3 in `ExtendedBlockEntityType.<init>` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| calls | `create` | `(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/B` | exact | invokevirtual@63 in `BlockApiLookupImpl.registerSelf` | unknown | [[30-Mechanisms/fabric-api-lookup-api-v1|fabric-api-lookup-api-v1]] | direct_reference |
| calls | `onlyOpCanSetNbt` | `()Z` | exact | invokespecial@16 in `ExtendedBlockEntityType.onlyOpCanSetNbt` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| injects_into | `<init>` | `(Lnet/minecraft/world/level/block/entity/BlockEntityType$BlockEntitySu` | name_only | @Inject at ['RETURN'] | both | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| reads | `validBlocks` | `Ljava/util/Set;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | declared |

## Declared members (3 fields, 6 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final factory : Lnet/minecraft/world/level/block/entity/BlockEntityType$BlockEntitySupplier;
private final validBlocks : Ljava/util/Set;
private final builtInRegistryHolder : Lnet/minecraft/core/Holder$Reference;
public <init>(Lnet/minecraft/world/level/block/entity/BlockEntityType$BlockEntitySupplier;Ljava/util/Set;)V
public create(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/world/level/block/entity/BlockEntity;
public isValid(Lnet/minecraft/world/level/block/state/BlockState;)Z
public builtInRegistryHolder()Lnet/minecraft/core/Holder$Reference;
public getBlockEntity(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/block/entity/BlockEntity;
public onlyOpCanSetNbt()Z
```
