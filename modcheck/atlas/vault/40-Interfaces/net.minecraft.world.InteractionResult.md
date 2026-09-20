---
type: "interface"
fqcn: "net.minecraft.world.InteractionResult"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.InteractionResult

System: [[20-Systems/net.minecraft.world|net.minecraft.world]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `consumesAction()Z` | `` | client | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| calls | `consumesAction()Z` | `` | client | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| reads | `PASSLnet/minecraft/world/InteractionResult$Pass;` | `` | both | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| reads | `PASSLnet/minecraft/world/InteractionResult$Pass;` | `` | both | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| reads | `PASSLnet/minecraft/world/InteractionResult$Pass;` | `` | both | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| reads | `PASSLnet/minecraft/world/InteractionResult$Pass;` | `` | client | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| reads | `PASSLnet/minecraft/world/InteractionResult$Pass;` | `` | client | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| reads | `SUCCESSLnet/minecraft/world/InteractionResult$Success;` | `` | client | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |

## Declared members (8, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.world.InteractionResult {
    public static final net.minecraft.world.InteractionResult$Success SUCCESS;
    public static final net.minecraft.world.InteractionResult$Success SUCCESS_SERVER;
    public static final net.minecraft.world.InteractionResult$Success CONSUME;
    public static final net.minecraft.world.InteractionResult$Fail FAIL;
    public static final net.minecraft.world.InteractionResult$Pass PASS;
    public static final net.minecraft.world.InteractionResult$TryEmptyHandInteraction TRY_WITH_EMPTY_HAND;
    public default boolean consumesAction();
    static {};
}
```
