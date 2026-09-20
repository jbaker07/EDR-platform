---
type: "interface"
fqcn: "net.minecraft.client.renderer.state.level.FirstPersonHandsAndItemsRenderState"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.state.level.FirstPersonHandsAndItemsRenderState

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `clearExtraData()V` | `` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (27, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.renderer.state.level.FirstPersonHandsAndItemsRenderState {
    public net.minecraft.world.InteractionHand attackHand;
    public float viewXRot;
    public float viewYRot;
    public float xBob;
    public float yBob;
    public boolean isScoping;
    public int useItemRemainingTicks;
    public int mainHandUseDuration;
    public int offHandUseDuration;
    public int mainHandChargeDuration;
    public int offHandChargeDuration;
    public float mainHandSwapScale;
    public float offHandSwapScale;
    public net.minecraft.client.renderer.state.level.FirstPersonHandsAndItemsRenderState$HandRenderSelection handRenderSelection;
    public net.minecraft.world.item.ItemStack mainHandItem;
    public net.minecraft.world.item.ItemStack offHandItem;
    public float mainHandHeight;
    public float oldMainHandHeight;
    public float offHandHeight;
    public float oldOffHandHeight;
    public final net.minecraft.client.renderer.item.ItemStackRenderState mainHandRenderState;
    public final net.minecraft.client.renderer.item.ItemStackRenderState offHandRenderState;
    public final net.minecraft.client.renderer.state.MapRenderState mainHandMapRenderState;
    public final net.minecraft.client.renderer.state.MapRenderState offHandMapRenderState;
    public boolean hasMainHandMapData;
    public boolean hasOffHandMapData;
    public net.minecraft.client.renderer.state.level.FirstPersonHandsAndItemsRenderState();
}
```
