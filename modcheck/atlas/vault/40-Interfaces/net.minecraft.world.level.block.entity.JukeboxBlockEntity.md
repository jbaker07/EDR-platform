---
type: "interface"
fqcn: "net.minecraft.world.level.block.entity.JukeboxBlockEntity"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.block.entity.JukeboxBlockEntity

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` public; extends `net/minecraft/world/level/block/entity/BlockEntity`; implements `net/minecraft/world/ticks/ContainerSingleItem$BlockContainerSingleItem`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `setTheItem` | `(Lnet/minecraft/world/item/ItemStack;)V` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | declared |
| injects_into | `setTheItem` | `(Lnet/minecraft/world/item/ItemStack;)V` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `item` | `Lnet/minecraft/world/item/ItemStack;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | declared |

## Declared members (4 fields, 24 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final SONG_ITEM_TAG_ID : Ljava/lang/String;
public static final TICKS_SINCE_SONG_STARTED_TAG_ID : Ljava/lang/String;
private item : Lnet/minecraft/world/item/ItemStack;
private final jukeboxSongPlayer : Lnet/minecraft/world/item/JukeboxSongPlayer;
public <init>(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)V
public getSongPlayer()Lnet/minecraft/world/item/JukeboxSongPlayer;
public onSongChanged()V
private notifyItemChangedInJukebox(Z)V
public popOutTheItem()V
public static tick(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/block/entity/JukeboxBlockEntity;)V
public getComparatorOutput()I
protected loadAdditional(Lnet/minecraft/world/level/storage/ValueInput;)V
protected saveAdditional(Lnet/minecraft/world/level/storage/ValueOutput;)V
public getTheItem()Lnet/minecraft/world/item/ItemStack;
public splitTheItem(I)Lnet/minecraft/world/item/ItemStack;
public setTheItem(Lnet/minecraft/world/item/ItemStack;)V
public setRemoved()V
public getMaxStackSize()I
public getContainerBlockEntity()Lnet/minecraft/world/level/block/entity/BlockEntity;
public canPlaceItem(ILnet/minecraft/world/item/ItemStack;)Z
public canTakeItem(Lnet/minecraft/world/Container;ILnet/minecraft/world/item/ItemStack;)Z
public preRemoveSideEffects(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)V
public setSongItemWithoutPlaying(Lnet/minecraft/world/item/ItemStack;)V
public tryForcePlaySong()V
private synthetic lambda$tryForcePlaySong$0(Lnet/minecraft/core/Holder;)V
private synthetic lambda$setSongItemWithoutPlaying$0(Lnet/minecraft/core/Holder;)V
private synthetic lambda$loadAdditional$0(Ljava/lang/Long;)V
private synthetic lambda$loadAdditional$1(Ljava/lang/Long;Lnet/minecraft/core/Holder;)V
```
