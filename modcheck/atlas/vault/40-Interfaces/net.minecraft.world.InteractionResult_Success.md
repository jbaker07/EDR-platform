---
type: "interface"
fqcn: "net.minecraft.world.InteractionResult$Success"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.InteractionResult$Success

System: [[20-Systems/net.minecraft.world|net.minecraft.world]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `swingSource()Lnet/minecraft/world/InteractionResult$SwingSource;` | `` | client | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |

## Declared members (14, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.world.InteractionResult$Success extends java.lang.Record implements net.minecraft.world.InteractionResult {
    private final net.minecraft.world.InteractionResult$SwingSource swingSource;
    private final net.minecraft.world.InteractionResult$ItemContext itemContext;
    public net.minecraft.world.InteractionResult$Success(net.minecraft.world.InteractionResult$SwingSource, net.minecraft.world.InteractionResult$ItemContext);
    public boolean shouldSwing();
    public boolean consumesAction();
    public net.minecraft.world.InteractionResult$Success heldItemTransformedTo(net.minecraft.world.item.ItemStack);
    public net.minecraft.world.InteractionResult$Success withoutItem();
    public boolean wasItemInteraction();
    public net.minecraft.world.item.ItemStack heldItemTransformedTo();
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public net.minecraft.world.InteractionResult$SwingSource swingSource();
    public net.minecraft.world.InteractionResult$ItemContext itemContext();
}
```
