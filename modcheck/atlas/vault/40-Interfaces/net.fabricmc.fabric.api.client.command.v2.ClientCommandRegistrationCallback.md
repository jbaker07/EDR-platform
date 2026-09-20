---
type: "interface"
fqcn: "net.fabricmc.fabric.api.client.command.v2.ClientCommandRegistrationCallback"
module: "fabric-command-api-v2"
sha256: "71e0ce2931b3467422b17fd49b181698a7ed79ca8cb92d112fc50ad2fced105e"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.client.command.v2.ClientCommandRegistrationCallback

Module: [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] -- kind: interface

```java
public static final net.fabricmc.fabric.api.event.Event<net.fabricmc.fabric.api.client.command.v2.ClientCommandRegistrationCallback> EVENT
public abstract void register(com.mojang.brigadier.CommandDispatcher<net.fabricmc.fabric.api.client.command.v2.FabricClientCommandSource>, net.minecraft.commands.CommandBuildContext)
static {}
```
