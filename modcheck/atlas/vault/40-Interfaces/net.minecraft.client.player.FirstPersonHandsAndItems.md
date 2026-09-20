---
type: "interface"
fqcn: "net.minecraft.client.player.FirstPersonHandsAndItems"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.player.FirstPersonHandsAndItems

System: [[20-Systems/net.minecraft.client.player|net.minecraft.client.player]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `tick` | `@Inject at HEAD` | client | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |

## Declared members (14, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.player.FirstPersonHandsAndItems {
    private net.minecraft.world.item.ItemStack mainHandItem;
    private net.minecraft.world.item.ItemStack offHandItem;
    private float mainHandHeight;
    private float oMainHandHeight;
    private float offHandHeight;
    private float oOffHandHeight;
    public net.minecraft.client.player.FirstPersonHandsAndItems();
    public void tick(net.minecraft.client.player.LocalPlayer);
    public void itemUsed(net.minecraft.world.InteractionHand);
    public void extractRenderState(net.minecraft.client.player.LocalPlayer, float, net.minecraft.client.renderer.state.level.FirstPersonHandsAndItemsRenderState);
    private boolean shouldInstantlyReplaceVisibleItem(net.minecraft.world.item.ItemStack, net.minecraft.world.item.ItemStack, net.minecraft.client.player.LocalPlayer);
    private boolean extractMapRenderState(net.minecraft.client.player.LocalPlayer, net.minecraft.world.item.ItemStack, net.minecraft.client.renderer.state.MapRenderState);
    public static net.minecraft.client.renderer.state.level.FirstPersonHandsAndItemsRenderState$HandRenderSelection evaluateWhichHandsToRender(net.minecraft.client.player.LocalPlayer);
    private static boolean isChargedCrossbow(net.minecraft.world.item.ItemStack);
}
```
