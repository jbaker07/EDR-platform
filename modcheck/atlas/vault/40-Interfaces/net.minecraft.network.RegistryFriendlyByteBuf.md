---
type: "interface"
fqcn: "net.minecraft.network.RegistryFriendlyByteBuf"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.RegistryFriendlyByteBuf

System: [[20-Systems/net.minecraft.network|net.minecraft.network]]

`class` public; extends `net/minecraft/network/FriendlyByteBuf`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `readBoolean` | `()Z` | inherited_exact | invokevirtual@40 in `AttachmentChange.decodePacket` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `readByte` | `()B` | inherited_exact | invokevirtual@24 in `Networking$OpenScreenPayload.fromBuf` | unknown | [[30-Mechanisms/fabric-menu-api-v1|fabric-menu-api-v1]] | direct_reference |
| calls | `readIdentifier` | `()Lnet/minecraft/resources/Identifier;` | inherited_exact | invokevirtual@1 in `Networking$OpenScreenPayload.fromBuf` | unknown | [[30-Mechanisms/fabric-menu-api-v1|fabric-menu-api-v1]] | direct_reference |
| calls | `readIdentifier` | `()Lnet/minecraft/resources/Identifier;` | inherited_exact | invokevirtual@34 in `CustomIngredientStreamCodec.decode` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `readIdentifier` | `()Lnet/minecraft/resources/Identifier;` | inherited_exact | invokevirtual@34 in `OptionalCustomIngredientStreamCodec.decode` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `readIdentifier` | `()Lnet/minecraft/resources/Identifier;` | inherited_exact | invokevirtual@1 in `ClientboundRecipeSyncPayload$Entry.read` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `readResourceKey` | `(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/resources/Resour` | inherited_exact | invokevirtual@73 in `ClientboundRecipeSyncPayload$Entry.read` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `readVarInt` | `()I` | inherited_exact | invokevirtual@6 in `ExtendedBlockParticleOptionStreamCodec.decode` | unknown | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |
| calls | `readVarInt` | `()I` | inherited_exact | invokevirtual@6 in `CustomIngredientStreamCodec.decode` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `readVarInt` | `()I` | inherited_exact | invokevirtual@6 in `OptionalCustomIngredientStreamCodec.decode` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `readVarInt` | `()I` | inherited_exact | invokevirtual@47 in `ClientboundRecipeSyncPayload$Entry.read` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `readableBytes` | `()I` | inherited_exact | invokevirtual@78 in `AttachmentChange.encodePacket` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `readableBytes` | `()I` | inherited_exact | invokevirtual@96 in `AttachmentChange.encodePacket` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `readableBytes` | `()I` | inherited_exact | invokevirtual@58 in `AttachmentChange.decodePacket` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `readerIndex` | `()I` | inherited_exact | invokevirtual@1 in `ExtendedBlockParticleOptionStreamCodec.decode` | unknown | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |
| calls | `readerIndex` | `()I` | inherited_exact | invokevirtual@1 in `CustomIngredientStreamCodec.decode` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `readerIndex` | `()I` | inherited_exact | invokevirtual@1 in `OptionalCustomIngredientStreamCodec.decode` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `readerIndex` | `(I)Lnet/minecraft/network/FriendlyByteBuf;` | inherited_exact | invokevirtual@15 in `ExtendedBlockParticleOptionStreamCodec.decode` | unknown | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |
| calls | `readerIndex` | `(I)Lnet/minecraft/network/FriendlyByteBuf;` | inherited_exact | invokevirtual@15 in `CustomIngredientStreamCodec.decode` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `readerIndex` | `(I)Lnet/minecraft/network/FriendlyByteBuf;` | inherited_exact | invokevirtual@15 in `OptionalCustomIngredientStreamCodec.decode` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `writeBoolean` | `(Z)Lnet/minecraft/network/FriendlyByteBuf;` | inherited_exact | invokevirtual@54 in `AttachmentChange.encodePacket` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `writeBoolean` | `(Z)Lnet/minecraft/network/FriendlyByteBuf;` | inherited_exact | invokevirtual@61 in `AttachmentChange.encodePacket` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `writeByte` | `(I)Lnet/minecraft/network/FriendlyByteBuf;` | inherited_exact | invokevirtual@14 in `Networking$OpenScreenPayload.write` | unknown | [[30-Mechanisms/fabric-menu-api-v1|fabric-menu-api-v1]] | direct_reference |
| calls | `writeIdentifier` | `(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/network/FriendlyB` | inherited_exact | invokevirtual@5 in `Networking$OpenScreenPayload.write` | unknown | [[30-Mechanisms/fabric-menu-api-v1|fabric-menu-api-v1]] | direct_reference |
| calls | `writeIdentifier` | `(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/network/FriendlyB` | inherited_exact | invokevirtual@42 in `CustomIngredientStreamCodec.encode` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `writeIdentifier` | `(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/network/FriendlyB` | inherited_exact | invokevirtual@67 in `OptionalCustomIngredientStreamCodec.encode` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `writeIdentifier` | `(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/network/FriendlyB` | inherited_exact | invokevirtual@13 in `ClientboundRecipeSyncPayload$Entry.write` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `writeResourceKey` | `(Lnet/minecraft/resources/ResourceKey;)V` | inherited_exact | invokevirtual@75 in `ClientboundRecipeSyncPayload$Entry.write` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `writeVarInt` | `(I)Lnet/minecraft/network/FriendlyByteBuf;` | inherited_exact | invokevirtual@29 in `ExtendedBlockParticleOptionStreamCodec.encode` | unknown | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |
| calls | `writeVarInt` | `(I)Lnet/minecraft/network/FriendlyByteBuf;` | inherited_exact | invokevirtual@26 in `CustomIngredientStreamCodec.encode` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `writeVarInt` | `(I)Lnet/minecraft/network/FriendlyByteBuf;` | inherited_exact | invokevirtual@51 in `OptionalCustomIngredientStreamCodec.encode` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `writeVarInt` | `(I)Lnet/minecraft/network/FriendlyByteBuf;` | inherited_exact | invokevirtual@27 in `ClientboundRecipeSyncPayload$Entry.write` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |

## Declared members (1 fields, 4 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final registryAccess : Lnet/minecraft/core/RegistryAccess;
public <init>(Lio/netty/buffer/ByteBuf;Lnet/minecraft/core/RegistryAccess;)V
public registryAccess()Lnet/minecraft/core/RegistryAccess;
public static decorator(Lnet/minecraft/core/RegistryAccess;)Ljava/util/function/Function;
private static synthetic lambda$decorator$0(Lnet/minecraft/core/RegistryAccess;Lio/netty/buffer/ByteBuf;)Lnet/minecraft/network/RegistryFriendlyByteBuf;
```
