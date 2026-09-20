---
type: "interface"
fqcn: "net.minecraft.world.level.block.entity.BannerBlockEntity"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.block.entity.BannerBlockEntity

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` public; extends `net/minecraft/world/level/block/entity/BlockEntity`; implements `net/minecraft/world/Nameable`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `getUpdateTag` | `(Lnet/minecraft/core/HolderLookup$Provider;)Lnet/minecraft/nbt/Compoun` | name_only | @ModifyExpressionValue at ['INVOKE'] | both | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |

## Declared members (6 fields, 16 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final MAX_PATTERNS : I
private static final TAG_PATTERNS : Ljava/lang/String;
private static final DEFAULT_NAME : Lnet/minecraft/network/chat/Component;
private name : Lnet/minecraft/network/chat/Component;
private final baseColor : Lnet/minecraft/world/item/DyeColor;
private patterns : Lnet/minecraft/world/level/block/entity/BannerPatternLayers;
public <init>(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)V
public <init>(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/item/DyeColor;)V
public getName()Lnet/minecraft/network/chat/Component;
public getCustomName()Lnet/minecraft/network/chat/Component;
protected saveAdditional(Lnet/minecraft/world/level/storage/ValueOutput;)V
protected loadAdditional(Lnet/minecraft/world/level/storage/ValueInput;)V
public getUpdatePacket()Lnet/minecraft/network/protocol/game/ClientboundBlockEntityDataPacket;
public getUpdateTag(Lnet/minecraft/core/HolderLookup$Provider;)Lnet/minecraft/nbt/CompoundTag;
public getPatterns()Lnet/minecraft/world/level/block/entity/BannerPatternLayers;
public getItem()Lnet/minecraft/world/item/ItemStack;
public getBaseColor()Lnet/minecraft/world/item/DyeColor;
protected applyImplicitComponents(Lnet/minecraft/core/component/DataComponentGetter;)V
protected collectImplicitComponents(Lnet/minecraft/core/component/DataComponentMap$Builder;)V
public removeComponentsFromTag(Lnet/minecraft/world/level/storage/ValueOutput;)V
public synthetic getUpdatePacket()Lnet/minecraft/network/protocol/Packet;
static <clinit>()V
```
