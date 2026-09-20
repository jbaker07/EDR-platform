---
type: "mechanism"
module: "fabric-transfer-api-v1"
version: "8.0.25+fcdff87f5d"
sha256: "599f69de9e7e693b4b8ca2f2792f129d8bd2e17fced9ae7b66f7e20b5a674db6"
lifecycle: "stable"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# fabric-transfer-api-v1

**Version** `8.0.25+fcdff87f5d` -- **artifact sha256** `599f69de9e7e693b4b8ca2f2792f129d8bd2e17fced9ae7b66f7e20b5a674db6`

## Declared (fabric.mod.json)

- environment: `*`
- depends: `{"fabricloader": ">=0.19.3", "fabric-api-lookup-api-v1": "*", "fabric-rendering-fluids-v1": "*"}`
- entrypoints: `null`
- mixin configs: `["fabric-transfer-api-v1.mixins.json"]`

## Events this module publishes

- [[50-Interactions/events/net.fabricmc.fabric.api.transfer.v1.fluid.FluidStorage.GENERAL_COMBINED_PROVIDER|FluidStorage.GENERAL_COMBINED_PROVIDER]]

## Vanilla types this module modifies (mixins)

| vanilla type | method | how | environment | mixin |
|---|---|---|---|---|
| [[40-Interfaces/net.minecraft.world.SimpleContainer|SimpleContainer]] | `setItem(ILnet/minecraft/world/item/ItemStack;)V` | wraps `@Redirect at INVOKE Lnet/minecraft/world/SimpleContainer;setChanged()V` | both | `SimpleContainerMixin.fabric_redirectChanged` |
| [[40-Interfaces/net.minecraft.world.item.BucketItem|BucketItem]] | `playEmptySound` | injects_into `@ModifyVariable at STORE` | both | `BucketItemMixin.hookEmptyingSound` |
| [[40-Interfaces/net.minecraft.world.level.block.CrafterBlock|CrafterBlock]] | `dispenseItem` | injects_into `@Inject at INVOKE Lnet/minecraft/world/item/ItemStack;isEmpty()Z` | both | `CrafterBlockMixin.transferOrSpawnStack` |
| [[40-Interfaces/net.minecraft.world.level.block.DropperBlock|DropperBlock]] | `dispenseFrom` | injects_into `@Inject at INVOKE Lnet/minecraft/core/dispenser/DispenseItemBehavior;dispense(Lnet/minecraft/core/dispenser/BlockSource;Lnet/minecraft/world/item/ItemStack;)Lnet/minecraft/world/item/ItemStack;` | both | `DropperBlockMixin.hookDispense` |
| [[40-Interfaces/net.minecraft.world.level.block.entity.AbstractFurnaceBlockEntity|AbstractFurnaceBlockEntity]] | `setItem` | injects_into `@Inject at HEAD` | both | `AbstractFurnaceBlockEntityMixin.setStackSuppressUpdate` |
| [[40-Interfaces/net.minecraft.world.level.block.entity.ChiseledBookShelfBlockEntity|ChiseledBookShelfBlockEntity]] | `setItem` | injects_into `@Inject at HEAD` | both | `ChiseledBookShelfBlockEntityMixin.setStackBypass` |
| [[40-Interfaces/net.minecraft.world.level.block.entity.HopperBlockEntity|HopperBlockEntity]] | `ejectItems` | injects_into `@Inject at INVOKE_ASSIGN Lnet/minecraft/world/level/block/entity/HopperBlockEntity;getAttachedContainer(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/entity/HopperBlockEntity;)Lnet/minecraft/world/Container;` | both | `HopperBlockEntityMixin.hookInsert` |
| [[40-Interfaces/net.minecraft.world.level.block.entity.HopperBlockEntity|HopperBlockEntity]] | `suckInItems(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/level/block/entity/Hopper;)Z` | injects_into `@Inject at INVOKE_ASSIGN Lnet/minecraft/world/level/block/entity/HopperBlockEntity;getSourceContainer(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/level/block/entity/Hopper;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/world/Container;` | both | `HopperBlockEntityMixin.hookExtract` |
| [[40-Interfaces/net.minecraft.world.level.block.entity.JukeboxBlockEntity|JukeboxBlockEntity]] | `setTheItem` | injects_into `@Inject at HEAD` | both | `JukeboxBlockEntityMixin.setStackBypass` |
| [[40-Interfaces/net.minecraft.world.level.material.Fluid|Fluid]] | `getPickupSound` | injects_into `@Inject at HEAD` | both | `FluidMixin.hookGetBucketFillSound` |

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.transfer.v1.client.fluid.FluidVariantRenderHandler|FluidVariantRenderHandler]] (interface, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.transfer.v1.client.fluid.FluidVariantRendering|FluidVariantRendering]] (class, 10 members)
- [[40-Interfaces/net.fabricmc.fabric.api.transfer.v1.context.ContainerItemContext|ContainerItemContext]] (interface, 17 members)
- [[40-Interfaces/net.fabricmc.fabric.api.transfer.v1.fluid.CauldronFluidContent|CauldronFluidContent]] (class, 10 members)
- [[40-Interfaces/net.fabricmc.fabric.api.transfer.v1.fluid.FluidConstants|FluidConstants]] (class, 14 members)
- [[40-Interfaces/net.fabricmc.fabric.api.transfer.v1.fluid.FluidStorage|FluidStorage]] (class, 5 members)
- [[40-Interfaces/net.fabricmc.fabric.api.transfer.v1.fluid.FluidStorageUtil|FluidStorageUtil]] (class, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.transfer.v1.fluid.FluidVariant|FluidVariant]] (interface, 10 members)
- [[40-Interfaces/net.fabricmc.fabric.api.transfer.v1.fluid.FluidVariantAttributeHandler|FluidVariantAttributeHandler]] (interface, 9 members)
- [[40-Interfaces/net.fabricmc.fabric.api.transfer.v1.fluid.FluidVariantAttributes|FluidVariantAttributes]] (class, 14 members)
- [[40-Interfaces/net.fabricmc.fabric.api.transfer.v1.fluid.base.EmptyItemFluidStorage|EmptyItemFluidStorage]] (class, 6 members)
- [[40-Interfaces/net.fabricmc.fabric.api.transfer.v1.fluid.base.FullItemFluidStorage|FullItemFluidStorage]] (class, 10 members)
- [[40-Interfaces/net.fabricmc.fabric.api.transfer.v1.fluid.base.SingleFluidStorage|SingleFluidStorage]] (abstract_class, 6 members)
- [[40-Interfaces/net.fabricmc.fabric.api.transfer.v1.item.ContainerStorage|ContainerStorage]] (interface, 4 members)
- [[40-Interfaces/net.fabricmc.fabric.api.transfer.v1.item.ItemStorage|ItemStorage]] (class, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.transfer.v1.item.ItemVariant|ItemVariant]] (interface, 14 members)
- [[40-Interfaces/net.fabricmc.fabric.api.transfer.v1.item.PlayerInventoryStorage|PlayerInventoryStorage]] (interface, 11 members)
- [[40-Interfaces/net.fabricmc.fabric.api.transfer.v1.item.base.SingleItemStorage|SingleItemStorage]] (abstract_class, 5 members)
- [[40-Interfaces/net.fabricmc.fabric.api.transfer.v1.item.base.SingleStackStorage|SingleStackStorage]] (abstract_class, 20 members)
- [[40-Interfaces/net.fabricmc.fabric.api.transfer.v1.storage.SlottedStorage|SlottedStorage]] (interface, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.transfer.v1.storage.Storage|Storage]] (interface, 10 members)
- [[40-Interfaces/net.fabricmc.fabric.api.transfer.v1.storage.StoragePreconditions|StoragePreconditions]] (class, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.transfer.v1.storage.StorageUtil|StorageUtil]] (class, 15 members)
- [[40-Interfaces/net.fabricmc.fabric.api.transfer.v1.storage.StorageView|StorageView]] (interface, 6 members)
- [[40-Interfaces/net.fabricmc.fabric.api.transfer.v1.storage.TransferVariant|TransferVariant]] (interface, 8 members)
- [[40-Interfaces/net.fabricmc.fabric.api.transfer.v1.storage.base.BlankVariantView|BlankVariantView]] (class, 8 members)
- [[40-Interfaces/net.fabricmc.fabric.api.transfer.v1.storage.base.CombinedSlottedStorage|CombinedSlottedStorage]] (class, 4 members)
- [[40-Interfaces/net.fabricmc.fabric.api.transfer.v1.storage.base.CombinedStorage|CombinedStorage]] (class, 8 members)
- [[40-Interfaces/net.fabricmc.fabric.api.transfer.v1.storage.base.ExtractionOnlyStorage|ExtractionOnlyStorage]] (interface, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.transfer.v1.storage.base.FilteringStorage|FilteringStorage]] (abstract_class, 16 members)
- [[40-Interfaces/net.fabricmc.fabric.api.transfer.v1.storage.base.InsertionOnlyStorage|InsertionOnlyStorage]] (interface, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.transfer.v1.storage.base.ResourceAmount|ResourceAmount]] (record, 6 members)
- [[40-Interfaces/net.fabricmc.fabric.api.transfer.v1.storage.base.SidedStorageBlockEntity|SidedStorageBlockEntity]] (interface, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.transfer.v1.storage.base.SingleSlotStorage|SingleSlotStorage]] (interface, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.transfer.v1.storage.base.SingleVariantItemStorage|SingleVariantItemStorage]] (abstract_class, 20 members)
- [[40-Interfaces/net.fabricmc.fabric.api.transfer.v1.storage.base.SingleVariantStorage|SingleVariantStorage]] (abstract_class, 23 members)
- [[40-Interfaces/net.fabricmc.fabric.api.transfer.v1.transaction.Transaction|Transaction]] (interface, 8 members)
- [[40-Interfaces/net.fabricmc.fabric.api.transfer.v1.transaction.TransactionContext|TransactionContext]] (interface, 5 members)
- [[40-Interfaces/net.fabricmc.fabric.api.transfer.v1.transaction.base.SnapshotParticipant|SnapshotParticipant]] (abstract_class, 8 members)

## What this establishes, and does not

- Injection targets and API signatures are `direct_reference`: read from the jar.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- Nothing here is `observed`. No game ran.
