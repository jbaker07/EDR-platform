---
type: "interface"
fqcn: "net.fabricmc.fabric.api.networking.v1.context.PacketContext"
module: "fabric-networking-api-v1"
sha256: "dfff56a878bba654646e986d90cf05913d7a914ad6c1292874de1ad505474544"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.networking.v1.context.PacketContext

Module: [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] -- kind: interface

```java
public static final net.fabricmc.fabric.api.networking.v1.context.PacketContext$ReadKey SERVER_INSTANCE
public static final net.fabricmc.fabric.api.networking.v1.context.PacketContext$ReadKey REGISTRY_ACCESS
public static final net.fabricmc.fabric.api.networking.v1.context.PacketContext$ReadKey GAME_PROFILE
public static final net.fabricmc.fabric.api.networking.v1.context.PacketContext$ReadKey CONNECTION
public abstract java.lang.Object get(net.fabricmc.fabric.api.networking.v1.context.PacketContext$ReadKey)
public java.lang.Object orElseThrow(net.fabricmc.fabric.api.networking.v1.context.PacketContext$ReadKey)
public java.lang.Object orElse(net.fabricmc.fabric.api.networking.v1.context.PacketContext$ReadKey, java.lang.Object)
public abstract void set(net.fabricmc.fabric.api.networking.v1.context.PacketContext$Key, java.lang.Object)
public static net.fabricmc.fabric.api.networking.v1.context.PacketContext get()
public static net.fabricmc.fabric.api.networking.v1.context.PacketContext orElseThrow()
public static void runWithContext(net.fabricmc.fabric.api.networking.v1.context.PacketContextProvider, java.lang.Runnable)
public static java.lang.Object supplyWithContext(net.fabricmc.fabric.api.networking.v1.context.PacketContextProvider, java.util.function.Supplier)
public static void runWithoutContext(java.lang.Runnable)
public static java.lang.Object supplyWithoutContext(java.util.function.Supplier)
public static net.fabricmc.fabric.api.networking.v1.context.PacketContext$Key key(net.minecraft.resources.Identifier)
```
