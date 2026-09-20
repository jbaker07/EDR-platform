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
public static final net.fabricmc.fabric.api.networking.v1.context.PacketContext$ReadKey<net.minecraft.server.MinecraftServer> SERVER_INSTANCE
public static final net.fabricmc.fabric.api.networking.v1.context.PacketContext$ReadKey<net.minecraft.core.RegistryAccess> REGISTRY_ACCESS
public static final net.fabricmc.fabric.api.networking.v1.context.PacketContext$ReadKey<com.mojang.authlib.GameProfile> GAME_PROFILE
public static final net.fabricmc.fabric.api.networking.v1.context.PacketContext$ReadKey<net.minecraft.network.Connection> CONNECTION
public abstract <T> T get(net.fabricmc.fabric.api.networking.v1.context.PacketContext$ReadKey<T>)
public default <T> T orElseThrow(net.fabricmc.fabric.api.networking.v1.context.PacketContext$ReadKey<T>)
public default <T> T orElse(net.fabricmc.fabric.api.networking.v1.context.PacketContext$ReadKey<T>, T)
public abstract <T> void set(net.fabricmc.fabric.api.networking.v1.context.PacketContext$Key<T>, T)
public static net.fabricmc.fabric.api.networking.v1.context.PacketContext get()
public static net.fabricmc.fabric.api.networking.v1.context.PacketContext orElseThrow()
public static void runWithContext(net.fabricmc.fabric.api.networking.v1.context.PacketContextProvider, java.lang.Runnable)
public static <T> T supplyWithContext(net.fabricmc.fabric.api.networking.v1.context.PacketContextProvider, java.util.function.Supplier<T>)
public static void runWithoutContext(java.lang.Runnable)
public static <T> T supplyWithoutContext(java.util.function.Supplier<T>)
public static <T> net.fabricmc.fabric.api.networking.v1.context.PacketContext$Key<T> key(net.minecraft.resources.Identifier)
static {}
```
