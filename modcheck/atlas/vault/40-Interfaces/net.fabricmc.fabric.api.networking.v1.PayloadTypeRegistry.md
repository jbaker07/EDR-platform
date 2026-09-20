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
public abstract net.minecraft.network.protocol.common.custom.CustomPacketPayload$TypeAndCodec register(net.minecraft.network.protocol.common.custom.CustomPacketPayload$Type, net.minecraft.network.codec.StreamCodec)
public abstract net.minecraft.network.protocol.common.custom.CustomPacketPayload$TypeAndCodec registerLarge(net.minecraft.network.protocol.common.custom.CustomPacketPayload$Type, net.minecraft.network.codec.StreamCodec, int)
public abstract net.minecraft.network.protocol.common.custom.CustomPacketPayload$TypeAndCodec registerLarge(net.minecraft.network.protocol.common.custom.CustomPacketPayload$Type, net.minecraft.network.codec.StreamCodec, java.util.function.IntSupplier)
public static net.fabricmc.fabric.api.networking.v1.PayloadTypeRegistry serverboundConfiguration()
public static net.fabricmc.fabric.api.networking.v1.PayloadTypeRegistry clientboundConfiguration()
public static net.fabricmc.fabric.api.networking.v1.PayloadTypeRegistry serverboundPlay()
public static net.fabricmc.fabric.api.networking.v1.PayloadTypeRegistry clientboundPlay()
```
