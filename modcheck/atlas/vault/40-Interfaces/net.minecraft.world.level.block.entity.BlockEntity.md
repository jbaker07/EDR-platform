---
type: "interface"
fqcn: "net.minecraft.world.level.block.entity.BlockEntity"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.block.entity.BlockEntity

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`abstract_class` public abstract; extends `java/lang/Object`; implements `net/minecraft/util/debug/DebugValueSource`, `net/minecraft/core/TypedInstance`, `net/fabricmc/fabric/api/blockgetter/v2/RenderDataBlockEntity`, `net/fabricmc/fabric/api/attachment/v1/AttachmentTarget`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getBlockPos` | `()Lnet/minecraft/core/BlockPos;` | exact | invokevirtual@5 in `BlockApiCacheImpl.lambda$static$1` | unknown | [[30-Mechanisms/fabric-api-lookup-api-v1|fabric-api-lookup-api-v1]] | direct_reference |
| calls | `getBlockPos` | `()Lnet/minecraft/core/BlockPos;` | exact | invokevirtual@5 in `BlockApiCacheImpl.lambda$static$0` | unknown | [[30-Mechanisms/fabric-api-lookup-api-v1|fabric-api-lookup-api-v1]] | direct_reference |
| calls | `getBlockPos` | `()Lnet/minecraft/core/BlockPos;` | exact | invokevirtual@42 in `PlayerLookup.tracking` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getBlockPos` | `()Lnet/minecraft/core/BlockPos;` | exact | invokevirtual@67 in `DebugMessages.forInventory` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getBlockState` | `()Lnet/minecraft/world/level/block/state/BlockState;` | exact | invokevirtual@20 in `BlockApiCacheImpl.find` | unknown | [[30-Mechanisms/fabric-api-lookup-api-v1|fabric-api-lookup-api-v1]] | direct_reference |
| calls | `getBlockState` | `()Lnet/minecraft/world/level/block/state/BlockState;` | exact | invokevirtual@52 in `BlockApiLookupImpl.find` | unknown | [[30-Mechanisms/fabric-api-lookup-api-v1|fabric-api-lookup-api-v1]] | direct_reference |
| calls | `getBlockState` | `()Lnet/minecraft/world/level/block/state/BlockState;` | exact | invokevirtual@56 in `DebugMessages.forInventory` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getLevel` | `()Lnet/minecraft/world/level/Level;` | exact | invokevirtual@1 in `BlockDataAccessorMixin.setData` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `getLevel` | `()Lnet/minecraft/world/level/Level;` | exact | invokevirtual@15 in `PlayerLookup.tracking` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getLevel` | `()Lnet/minecraft/world/level/Level;` | exact | invokevirtual@35 in `PlayerLookup.tracking` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getLevel` | `()Lnet/minecraft/world/level/Level;` | exact | invokevirtual@63 in `DebugMessages.forInventory` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getRenderData` | `()Ljava/lang/Object;` | inherited_exact | invokevirtual@20 in `FabricBlockGetter.getBlockEntityRenderData` | unknown | [[30-Mechanisms/fabric-block-getter-api-v2|fabric-block-getter-api-v2]] | direct_reference |
| calls | `getRenderData` | `()Ljava/lang/Object;` | inherited_exact | invokevirtual@198 in `RenderRegionCacheMixin.mapChunk` | unknown | [[30-Mechanisms/fabric-block-getter-api-v2|fabric-block-getter-api-v2]] | direct_reference |
| calls | `getType` | `()Lnet/minecraft/world/level/block/entity/BlockEntityType;` | exact | invokevirtual@7 in `BlockApiLookupImpl.lambda$registerForBlockEntities$0` | unknown | [[30-Mechanisms/fabric-api-lookup-api-v1|fabric-api-lookup-api-v1]] | direct_reference |
| calls | `getUpdatePacket` | `()Lnet/minecraft/network/protocol/Packet;` | exact | invokevirtual@98 in `ServerPlayerGameModeMixin.startBlockBreak` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| calls | `hasLevel` | `()Z` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | declared |
| calls | `hasLevel` | `()Z` | exact | invokevirtual@8 in `PlayerLookup.tracking` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `setChanged` | `()V` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | declared |
| injects_into | `loadWithComponents` | `(Lnet/minecraft/world/level/storage/ValueInput;)V` | name_only | @Inject at ['RETURN'] | both | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| injects_into | `saveWithoutMetadata` | `(Lnet/minecraft/world/level/storage/ValueOutput;)V` | exact | @Inject at ['TAIL'] | both | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| reads | `level` | `Lnet/minecraft/world/level/Level;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | declared |
| reads | `worldPosition` | `Lnet/minecraft/core/BlockPos;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | declared |

## Declared members (8 fields, 51 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final TYPE_CODEC : Lcom/mojang/serialization/Codec;
private static final LOGGER : Lorg/slf4j/Logger;
private final type : Lnet/minecraft/world/level/block/entity/BlockEntityType;
protected level : Lnet/minecraft/world/level/Level;
protected final worldPosition : Lnet/minecraft/core/BlockPos;
protected remove : Z
private blockState : Lnet/minecraft/world/level/block/state/BlockState;
private components : Lnet/minecraft/core/component/DataComponentMap;
public <init>(Lnet/minecraft/world/level/block/entity/BlockEntityType;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)V
private validateBlockState(Lnet/minecraft/world/level/block/state/BlockState;)V
public isValidBlockState(Lnet/minecraft/world/level/block/state/BlockState;)Z
public static getPosFromTag(Lnet/minecraft/world/level/ChunkPos;Lnet/minecraft/nbt/CompoundTag;)Lnet/minecraft/core/BlockPos;
public getLevel()Lnet/minecraft/world/level/Level;
public setLevel(Lnet/minecraft/world/level/Level;)V
public hasLevel()Z
protected loadAdditional(Lnet/minecraft/world/level/storage/ValueInput;)V
public final loadWithComponents(Lnet/minecraft/world/level/storage/ValueInput;)V
public final loadCustomOnly(Lnet/minecraft/world/level/storage/ValueInput;)V
protected saveAdditional(Lnet/minecraft/world/level/storage/ValueOutput;)V
public final saveWithFullMetadata(Lnet/minecraft/core/HolderLookup$Provider;)Lnet/minecraft/nbt/CompoundTag;
public saveWithFullMetadata(Lnet/minecraft/world/level/storage/ValueOutput;)V
public saveWithId(Lnet/minecraft/world/level/storage/ValueOutput;)V
public final saveWithoutMetadata(Lnet/minecraft/core/HolderLookup$Provider;)Lnet/minecraft/nbt/CompoundTag;
public saveWithoutMetadata(Lnet/minecraft/world/level/storage/ValueOutput;)V
public final saveCustomOnly(Lnet/minecraft/core/HolderLookup$Provider;)Lnet/minecraft/nbt/CompoundTag;
public saveCustomOnly(Lnet/minecraft/world/level/storage/ValueOutput;)V
private saveId(Lnet/minecraft/world/level/storage/ValueOutput;)V
public static addEntityType(Lnet/minecraft/world/level/storage/ValueOutput;Lnet/minecraft/world/level/block/entity/BlockEntityType;)V
private saveMetadata(Lnet/minecraft/world/level/storage/ValueOutput;)V
public static loadStatic(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/nbt/CompoundTag;Lnet/minecraft/core/HolderLookup$Provider;)Lnet/minecraft/world/level/block/entity/BlockEntity;
public setChanged()V
protected static setChanged(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)V
public getBlockPos()Lnet/minecraft/core/BlockPos;
public getBlockState()Lnet/minecraft/world/level/block/state/BlockState;
public getUpdatePacket()Lnet/minecraft/network/protocol/Packet;
public getUpdateTag(Lnet/minecraft/core/HolderLookup$Provider;)Lnet/minecraft/nbt/CompoundTag;
public isRemoved()Z
public setRemoved()V
public clearRemoved()V
public preRemoveSideEffects(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)V
public triggerEvent(II)Z
public fillCrashReportCategory(Lnet/minecraft/CrashReportCategory;)V
public getNameForReporting()Ljava/lang/String;
public getType()Lnet/minecraft/world/level/block/entity/BlockEntityType;
public typeHolder()Lnet/minecraft/core/Holder;
public setBlockState(Lnet/minecraft/world/level/block/state/BlockState;)V
protected applyImplicitComponents(Lnet/minecraft/core/component/DataComponentGetter;)V
public final applyComponentsFromItemStack(Lnet/minecraft/world/item/ItemStack;)V
public final applyComponents(Lnet/minecraft/core/component/DataComponentMap;Lnet/minecraft/core/component/DataComponentPatch;)V
protected collectImplicitComponents(Lnet/minecraft/core/component/DataComponentMap$Builder;)V
public removeComponentsFromTag(Lnet/minecraft/world/level/storage/ValueOutput;)V
public final collectComponents()Lnet/minecraft/core/component/DataComponentMap;
public components()Lnet/minecraft/core/component/DataComponentMap;
public setComponents(Lnet/minecraft/core/component/DataComponentMap;)V
public static parseCustomNameSafe(Lnet/minecraft/world/level/storage/ValueInput;Ljava/lang/String;)Lnet/minecraft/network/chat/Component;
public problemPath()Lnet/minecraft/util/ProblemReporter$PathElement;
public registerDebugValues(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/util/debug/DebugValueSource$Registration;)V
private synthetic lambda$fillCrashReportCategory$0()Ljava/lang/String;
static <clinit>()V
```
