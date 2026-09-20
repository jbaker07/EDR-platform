---
type: "interface"
fqcn: "net.fabricmc.fabric.api.block.BlockAttackInteractionAware"
module: "fabric-events-interaction-v0"
sha256: "f57dd8df1d78cbcaf6ee1073aebd64e5c959cc7224002098e653832f0ecd7de8"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.block.BlockAttackInteractionAware

Module: [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] -- kind: interface

```java
public abstract boolean onAttackInteraction(net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.Level, net.minecraft.core.BlockPos, net.minecraft.world.entity.player.Player, net.minecraft.world.InteractionHand, net.minecraft.core.Direction)
```
