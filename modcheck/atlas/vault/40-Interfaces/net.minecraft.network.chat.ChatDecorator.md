---
type: "interface"
fqcn: "net.minecraft.network.chat.ChatDecorator"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.chat.ChatDecorator

System: [[20-Systems/net.minecraft.network.chat|net.minecraft.network.chat]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `decorate(Lnet/minecraft/server/level/ServerPlayer;Lnet/minecraft/net` | `` | both | [[30-Mechanisms/fabric-message-api-v1|fabric-message-api-v1]] | direct_reference |

## Declared members (4, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.network.chat.ChatDecorator {
    public static final net.minecraft.network.chat.ChatDecorator PLAIN;
    public abstract net.minecraft.network.chat.Component decorate(net.minecraft.server.level.ServerPlayer, net.minecraft.network.chat.Component);
    private static net.minecraft.network.chat.Component lambda$static$0(net.minecraft.server.level.ServerPlayer, net.minecraft.network.chat.Component);
    static {};
}
```
