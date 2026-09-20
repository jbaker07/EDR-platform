---
type: "interface"
fqcn: "net.minecraft.network.RegistryFriendlyByteBuf"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.RegistryFriendlyByteBuf

System: [[20-Systems/net.minecraft.network|net.minecraft.network]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `readBoolean()Z` | `` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `readIdentifier()Lnet/minecraft/resources/Identifier;` | `` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `readIdentifier()Lnet/minecraft/resources/Identifier;` | `` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `readVarInt()I` | `` | unknown | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |
| calls | `readVarInt()I` | `` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `readVarInt()I` | `` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `readableBytes()I` | `` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `readerIndex()I` | `` | unknown | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |
| calls | `readerIndex(I)Lnet/minecraft/network/FriendlyByteBuf;` | `` | unknown | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |
| calls | `readerIndex()I` | `` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `readerIndex(I)Lnet/minecraft/network/FriendlyByteBuf;` | `` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `readerIndex()I` | `` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `readerIndex(I)Lnet/minecraft/network/FriendlyByteBuf;` | `` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `writeBoolean(Z)Lnet/minecraft/network/FriendlyByteBuf;` | `` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `writeIdentifier(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/network` | `` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `writeIdentifier(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/network` | `` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `writeVarInt(I)Lnet/minecraft/network/FriendlyByteBuf;` | `` | unknown | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |
| calls | `writeVarInt(I)Lnet/minecraft/network/FriendlyByteBuf;` | `` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `writeVarInt(I)Lnet/minecraft/network/FriendlyByteBuf;` | `` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |

## Declared members (5, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.network.RegistryFriendlyByteBuf extends net.minecraft.network.FriendlyByteBuf {
    private final net.minecraft.core.RegistryAccess registryAccess;
    public net.minecraft.network.RegistryFriendlyByteBuf(io.netty.buffer.ByteBuf, net.minecraft.core.RegistryAccess);
    public net.minecraft.core.RegistryAccess registryAccess();
    public static java.util.function.Function<io.netty.buffer.ByteBuf, net.minecraft.network.RegistryFriendlyByteBuf> decorator(net.minecraft.core.RegistryAccess);
    private static net.minecraft.network.RegistryFriendlyByteBuf lambda$decorator$0(net.minecraft.core.RegistryAccess, io.netty.buffer.ByteBuf);
}
```
