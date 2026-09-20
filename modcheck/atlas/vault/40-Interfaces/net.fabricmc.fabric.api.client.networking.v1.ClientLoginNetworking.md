---
type: "interface"
fqcn: "net.fabricmc.fabric.api.client.networking.v1.ClientLoginNetworking"
module: "fabric-networking-api-v1"
sha256: "dfff56a878bba654646e986d90cf05913d7a914ad6c1292874de1ad505474544"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.client.networking.v1.ClientLoginNetworking

Module: [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] -- kind: class

```java
public static boolean registerGlobalReceiver(net.minecraft.resources.Identifier, net.fabricmc.fabric.api.client.networking.v1.ClientLoginNetworking$LoginQueryRequestHandler)
public static net.fabricmc.fabric.api.client.networking.v1.ClientLoginNetworking$LoginQueryRequestHandler unregisterGlobalReceiver(net.minecraft.resources.Identifier)
public static java.util.Set<net.minecraft.resources.Identifier> getGlobalReceivers()
public static boolean registerReceiver(net.minecraft.resources.Identifier, net.fabricmc.fabric.api.client.networking.v1.ClientLoginNetworking$LoginQueryRequestHandler) throws java.lang.IllegalStateException
public static net.fabricmc.fabric.api.client.networking.v1.ClientLoginNetworking$LoginQueryRequestHandler unregisterReceiver(net.minecraft.resources.Identifier) throws java.lang.IllegalStateException
```
