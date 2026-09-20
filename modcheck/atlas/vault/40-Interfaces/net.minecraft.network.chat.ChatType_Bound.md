---
type: "interface"
fqcn: "net.minecraft.network.chat.ChatType$Bound"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.chat.ChatType$Bound

System: [[20-Systems/net.minecraft.network.chat|net.minecraft.network.chat]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `decorate(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/netwo` | `` | client | [[30-Mechanisms/fabric-message-api-v1|fabric-message-api-v1]] | direct_reference |

## Declared members (16, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.network.chat.ChatType$Bound extends java.lang.Record {
    private final net.minecraft.core.Holder<net.minecraft.network.chat.ChatType> chatType;
    private final net.minecraft.network.chat.Component name;
    private final java.util.Optional<net.minecraft.network.chat.Component> targetName;
    public static final net.minecraft.network.codec.StreamCodec<net.minecraft.network.RegistryFriendlyByteBuf, net.minecraft.network.chat.ChatType$Bound> STREAM_CODEC;
    private net.minecraft.network.chat.ChatType$Bound(net.minecraft.core.Holder<net.minecraft.network.chat.ChatType>, net.minecraft.network.chat.Component);
    public net.minecraft.network.chat.ChatType$Bound(net.minecraft.core.Holder<net.minecraft.network.chat.ChatType>, net.minecraft.network.chat.Component, java.util.Optional<net.minecraft.network.chat.Component>);
    public net.minecraft.network.chat.Component decorate(net.minecraft.network.chat.Component);
    public net.minecraft.network.chat.Component decorateNarration(net.minecraft.network.chat.Component);
    public net.minecraft.network.chat.ChatType$Bound withTargetName(net.minecraft.network.chat.Component);
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public net.minecraft.core.Holder<net.minecraft.network.chat.ChatType> chatType();
    public net.minecraft.network.chat.Component name();
    public java.util.Optional<net.minecraft.network.chat.Component> targetName();
    static {};
}
```
