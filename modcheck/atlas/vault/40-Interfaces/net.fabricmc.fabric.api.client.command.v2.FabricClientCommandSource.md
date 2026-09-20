---
type: "interface"
fqcn: "net.fabricmc.fabric.api.client.command.v2.FabricClientCommandSource"
module: "fabric-command-api-v2"
sha256: "71e0ce2931b3467422b17fd49b181698a7ed79ca8cb92d112fc50ad2fced105e"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.client.command.v2.FabricClientCommandSource

Module: [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] -- kind: interface

```java
public abstract void sendFeedback(net.minecraft.network.chat.Component)
public abstract void sendError(net.minecraft.network.chat.Component)
public abstract net.minecraft.client.Minecraft getClient()
public abstract net.minecraft.client.player.LocalPlayer getPlayer()
public net.minecraft.world.entity.Entity getEntity()
public net.minecraft.world.phys.Vec3 getPosition()
public net.minecraft.world.phys.Vec2 getRotation()
public abstract net.minecraft.client.multiplayer.ClientLevel getLevel()
public java.lang.Object getMeta(java.lang.String)
public abstract boolean attended()
```
