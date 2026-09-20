---
type: "interface"
fqcn: "net.minecraft.world.level.block.entity.JukeboxBlockEntity"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.block.entity.JukeboxBlockEntity

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `setTheItem` | `@Inject at HEAD` | both | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (28, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.level.block.entity.JukeboxBlockEntity extends net.minecraft.world.level.block.entity.BlockEntity implements net.minecraft.world.ticks.ContainerSingleItem$BlockContainerSingleItem {
    public static final java.lang.String SONG_ITEM_TAG_ID;
    public static final java.lang.String TICKS_SINCE_SONG_STARTED_TAG_ID;
    private net.minecraft.world.item.ItemStack item;
    private final net.minecraft.world.item.JukeboxSongPlayer jukeboxSongPlayer;
    public net.minecraft.world.level.block.entity.JukeboxBlockEntity(net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState);
    public net.minecraft.world.item.JukeboxSongPlayer getSongPlayer();
    public void onSongChanged();
    private void notifyItemChangedInJukebox(boolean);
    public void popOutTheItem();
    public static void tick(net.minecraft.world.level.Level, net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.block.entity.JukeboxBlockEntity);
    public int getComparatorOutput();
    protected void loadAdditional(net.minecraft.world.level.storage.ValueInput);
    protected void saveAdditional(net.minecraft.world.level.storage.ValueOutput);
    public net.minecraft.world.item.ItemStack getTheItem();
    public net.minecraft.world.item.ItemStack splitTheItem(int);
    public void setTheItem(net.minecraft.world.item.ItemStack);
    public void setRemoved();
    public int getMaxStackSize();
    public net.minecraft.world.level.block.entity.BlockEntity getContainerBlockEntity();
    public boolean canPlaceItem(int, net.minecraft.world.item.ItemStack);
    public boolean canTakeItem(net.minecraft.world.Container, int, net.minecraft.world.item.ItemStack);
    public void preRemoveSideEffects(net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState);
    public void setSongItemWithoutPlaying(net.minecraft.world.item.ItemStack);
    public void tryForcePlaySong();
    private void lambda$tryForcePlaySong$0(net.minecraft.core.Holder);
    private void lambda$setSongItemWithoutPlaying$0(net.minecraft.core.Holder);
    private void lambda$loadAdditional$0(java.lang.Long);
    private void lambda$loadAdditional$1(java.lang.Long, net.minecraft.core.Holder);
}
```
