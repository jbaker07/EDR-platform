---
type: "interface"
fqcn: "net.minecraft.world.InteractionResult$Success"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.InteractionResult$Success

System: [[20-Systems/net.minecraft.world|net.minecraft.world]]

`record` public final; extends `java/lang/Record`; implements `net/minecraft/world/InteractionResult`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `swingSource` | `()Lnet/minecraft/world/InteractionResult$SwingSource;` | exact | invokevirtual@121 in `MinecraftMixin.injectUseEntityCallback` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |

## Declared members (2 fields, 12 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final swingSource : Lnet/minecraft/world/InteractionResult$SwingSource;
private final itemContext : Lnet/minecraft/world/InteractionResult$ItemContext;
public <init>(Lnet/minecraft/world/InteractionResult$SwingSource;Lnet/minecraft/world/InteractionResult$ItemContext;)V
public shouldSwing()Z
public consumesAction()Z
public heldItemTransformedTo(Lnet/minecraft/world/item/ItemStack;)Lnet/minecraft/world/InteractionResult$Success;
public withoutItem()Lnet/minecraft/world/InteractionResult$Success;
public wasItemInteraction()Z
public heldItemTransformedTo()Lnet/minecraft/world/item/ItemStack;
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public swingSource()Lnet/minecraft/world/InteractionResult$SwingSource;
public itemContext()Lnet/minecraft/world/InteractionResult$ItemContext;
```
