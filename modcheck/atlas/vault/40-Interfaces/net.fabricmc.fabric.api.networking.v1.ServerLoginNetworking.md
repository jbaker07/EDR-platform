---
type: "interface"
fqcn: "net.fabricmc.fabric.api.networking.v1.ServerLoginNetworking"
module: "fabric-networking-api-v1"
sha256: "dfff56a878bba654646e986d90cf05913d7a914ad6c1292874de1ad505474544"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.networking.v1.ServerLoginNetworking

Module: [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] -- kind: class

```java
public static boolean registerGlobalReceiver(net.minecraft.resources.Identifier, net.fabricmc.fabric.api.networking.v1.ServerLoginNetworking$LoginQueryResponseHandler)
public static net.fabricmc.fabric.api.networking.v1.ServerLoginNetworking$LoginQueryResponseHandler unregisterGlobalReceiver(net.minecraft.resources.Identifier)
public static java.util.Set getGlobalReceivers()
public static boolean registerReceiver(net.minecraft.server.network.ServerLoginPacketListenerImpl, net.minecraft.resources.Identifier, net.fabricmc.fabric.api.networking.v1.ServerLoginNetworking$LoginQueryResponseHandler)
public static net.fabricmc.fabric.api.networking.v1.ServerLoginNetworking$LoginQueryResponseHandler unregisterReceiver(net.minecraft.server.network.ServerLoginPacketListenerImpl, net.minecraft.resources.Identifier)
public static net.minecraft.server.MinecraftServer getServer(net.minecraft.server.network.ServerLoginPacketListenerImpl)
public static net.fabricmc.fabric.api.networking.v1.LoginPacketSender getSender(net.minecraft.server.network.ServerLoginPacketListenerImpl)
```
