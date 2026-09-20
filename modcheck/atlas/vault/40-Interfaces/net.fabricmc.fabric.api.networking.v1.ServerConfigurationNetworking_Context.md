---
type: "interface"
fqcn: "net.fabricmc.fabric.api.networking.v1.ServerConfigurationNetworking$Context"
module: "fabric-networking-api-v1"
sha256: "dfff56a878bba654646e986d90cf05913d7a914ad6c1292874de1ad505474544"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.networking.v1.ServerConfigurationNetworking$Context

Module: [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] -- kind: interface

```java
public abstract net.minecraft.server.MinecraftServer server()
public abstract net.minecraft.server.network.ServerConfigurationPacketListenerImpl packetListener()
public abstract net.fabricmc.fabric.api.networking.v1.PacketSender responseSender()
public default net.fabricmc.fabric.api.networking.v1.context.PacketContext packetContext()
```
