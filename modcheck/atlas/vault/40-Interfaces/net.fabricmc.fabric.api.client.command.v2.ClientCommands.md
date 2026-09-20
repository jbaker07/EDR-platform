---
type: "interface"
fqcn: "net.fabricmc.fabric.api.client.command.v2.ClientCommands"
module: "fabric-command-api-v2"
sha256: "71e0ce2931b3467422b17fd49b181698a7ed79ca8cb92d112fc50ad2fced105e"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.client.command.v2.ClientCommands

Module: [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] -- kind: class

```java
public static com.mojang.brigadier.CommandDispatcher<net.fabricmc.fabric.api.client.command.v2.FabricClientCommandSource> getActiveDispatcher()
public static void refreshCommandCompletions()
public static com.mojang.brigadier.builder.LiteralArgumentBuilder<net.fabricmc.fabric.api.client.command.v2.FabricClientCommandSource> literal(java.lang.String)
public static <T> com.mojang.brigadier.builder.RequiredArgumentBuilder<net.fabricmc.fabric.api.client.command.v2.FabricClientCommandSource, T> argument(java.lang.String, com.mojang.brigadier.arguments.ArgumentType<T>)
```
