---
type: "interface"
fqcn: "net.fabricmc.fabric.api.networking.v1.PayloadTypeRegistry"
module: "fabric-networking-api-v1"
sha256: "dfff56a878bba654646e986d90cf05913d7a914ad6c1292874de1ad505474544"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.networking.v1.PayloadTypeRegistry

Module: [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] -- kind: interface

```java
public abstract <T extends net.minecraft.network.protocol.common.custom.CustomPacketPayload> net.minecraft.network.protocol.common.custom.CustomPacketPayload$TypeAndCodec<? super B, T> register(net.minecraft.network.protocol.common.custom.CustomPacketPayload$Type<T>, net.minecraft.network.codec.StreamCodec<? super B, T>)
public abstract <T extends net.minecraft.network.protocol.common.custom.CustomPacketPayload> net.minecraft.network.protocol.common.custom.CustomPacketPayload$TypeAndCodec<? super B, T> registerLarge(net.minecraft.network.protocol.common.custom.CustomPacketPayload$Type<T>, net.minecraft.network.codec.StreamCodec<? super B, T>, int)
public abstract <T extends net.minecraft.network.protocol.common.custom.CustomPacketPayload> net.minecraft.network.protocol.common.custom.CustomPacketPayload$TypeAndCodec<? super B, T> registerLarge(net.minecraft.network.protocol.common.custom.CustomPacketPayload$Type<T>, net.minecraft.network.codec.StreamCodec<? super B, T>, java.util.function.IntSupplier)
public static net.fabricmc.fabric.api.networking.v1.PayloadTypeRegistry<net.minecraft.network.FriendlyByteBuf> serverboundConfiguration()
public static net.fabricmc.fabric.api.networking.v1.PayloadTypeRegistry<net.minecraft.network.FriendlyByteBuf> clientboundConfiguration()
public static net.fabricmc.fabric.api.networking.v1.PayloadTypeRegistry<net.minecraft.network.RegistryFriendlyByteBuf> serverboundPlay()
public static net.fabricmc.fabric.api.networking.v1.PayloadTypeRegistry<net.minecraft.network.RegistryFriendlyByteBuf> clientboundPlay()
```
