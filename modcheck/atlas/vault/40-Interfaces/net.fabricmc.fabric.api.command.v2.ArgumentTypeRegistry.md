---
type: "interface"
fqcn: "net.fabricmc.fabric.api.command.v2.ArgumentTypeRegistry"
module: "fabric-command-api-v2"
sha256: "71e0ce2931b3467422b17fd49b181698a7ed79ca8cb92d112fc50ad2fced105e"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.command.v2.ArgumentTypeRegistry

Module: [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] -- kind: class

```java
public static <A extends com.mojang.brigadier.arguments.ArgumentType<?>, T extends net.minecraft.commands.synchronization.ArgumentTypeInfo$Template<A>> void registerArgumentType(net.minecraft.resources.Identifier, java.lang.Class<? extends A>, net.minecraft.commands.synchronization.ArgumentTypeInfo<A, T>)
```
