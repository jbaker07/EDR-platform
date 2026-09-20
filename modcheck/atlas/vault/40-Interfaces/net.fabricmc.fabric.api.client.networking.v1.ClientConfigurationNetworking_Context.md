---
type: "interface"
fqcn: "net.fabricmc.fabric.api.client.networking.v1.ClientConfigurationNetworking$Context"
module: "fabric-networking-api-v1"
sha256: "dfff56a878bba654646e986d90cf05913d7a914ad6c1292874de1ad505474544"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.client.networking.v1.ClientConfigurationNetworking$Context

Module: [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] -- kind: interface

```java
public abstract net.minecraft.client.Minecraft client()
public abstract net.minecraft.client.multiplayer.ClientConfigurationPacketListenerImpl packetListener()
public abstract net.fabricmc.fabric.api.networking.v1.PacketSender responseSender()
public net.fabricmc.fabric.api.networking.v1.context.PacketContext packetContext()
```
