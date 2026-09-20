---
type: "interface"
fqcn: "net.minecraft.world.item.BucketItem"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.item.BucketItem

System: [[20-Systems/net.minecraft.world.item|net.minecraft.world.item]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `playEmptySound` | `@ModifyVariable at STORE` | both | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (10, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.item.BucketItem extends net.minecraft.world.item.Item implements net.minecraft.world.item.DispensibleContainerItem {
    protected final net.minecraft.world.level.material.Fluid content;
    public net.minecraft.world.item.BucketItem(net.minecraft.world.level.material.Fluid, net.minecraft.world.item.Item$Properties);
    public net.minecraft.world.InteractionResult use(net.minecraft.world.level.Level, net.minecraft.world.entity.player.Player, net.minecraft.world.InteractionHand);
    public net.minecraft.world.level.ClipContext$Fluid getFluidContext();
    public static net.minecraft.world.item.ItemStack getEmptySuccessItem(net.minecraft.world.item.ItemStack, net.minecraft.world.entity.player.Player);
    public void checkExtraContent(net.minecraft.world.entity.LivingEntity, net.minecraft.world.level.Level, net.minecraft.world.item.ItemStack, net.minecraft.core.BlockPos);
    public boolean emptyContents(net.minecraft.world.entity.LivingEntity, net.minecraft.world.level.Level, net.minecraft.core.BlockPos, net.minecraft.world.phys.BlockHitResult);
    protected void playEmptySound(net.minecraft.world.entity.LivingEntity, net.minecraft.world.level.LevelAccessor, net.minecraft.core.BlockPos);
    public net.minecraft.world.level.material.Fluid getContent();
    private static void lambda$use$0(net.minecraft.world.entity.player.Player, net.minecraft.sounds.SoundEvent);
}
```
