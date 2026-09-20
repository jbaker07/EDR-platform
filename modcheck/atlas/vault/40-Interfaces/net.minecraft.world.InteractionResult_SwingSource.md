---
type: "interface"
fqcn: "net.minecraft.world.InteractionResult$SwingSource"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.InteractionResult$SwingSource

System: [[20-Systems/net.minecraft.world|net.minecraft.world]]

`enum` public final; extends `java/lang/Enum`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `PREDICTED` | `Lnet/minecraft/world/InteractionResult$SwingSource;` | exact | getstatic@124 in `MinecraftMixin.injectUseEntityCallback` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |

## Declared members (4 fields, 5 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final NONE : Lnet/minecraft/world/InteractionResult$SwingSource;
public static final PREDICTED : Lnet/minecraft/world/InteractionResult$SwingSource;
public static final SERVER_ONLY : Lnet/minecraft/world/InteractionResult$SwingSource;
private static final synthetic $VALUES : [Lnet/minecraft/world/InteractionResult$SwingSource;
public static values()[Lnet/minecraft/world/InteractionResult$SwingSource;
public static valueOf(Ljava/lang/String;)Lnet/minecraft/world/InteractionResult$SwingSource;
private <init>(Ljava/lang/String;I)V
private static synthetic $values()[Lnet/minecraft/world/InteractionResult$SwingSource;
static <clinit>()V
```
