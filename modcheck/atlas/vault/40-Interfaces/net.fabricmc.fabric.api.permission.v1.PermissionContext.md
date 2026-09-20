---
type: "interface"
fqcn: "net.fabricmc.fabric.api.permission.v1.PermissionContext"
module: "fabric-permission-api-v1"
sha256: "a3a82771b3fd9f2eb36e1098c5f90759ac877982b9ac297831a1fce9b2185bf8"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.permission.v1.PermissionContext

Module: [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] -- kind: interface

```java
public static final net.fabricmc.fabric.api.permission.v1.PermissionContext$Key<java.lang.String> NAME
public static final net.fabricmc.fabric.api.permission.v1.PermissionContext$Key<net.minecraft.world.phys.Vec3> POSITION
public static final net.fabricmc.fabric.api.permission.v1.PermissionContext$Key<net.minecraft.core.BlockPos> BLOCK_POSITION
public static final net.fabricmc.fabric.api.permission.v1.PermissionContext$Key<net.minecraft.world.entity.Entity> ENTITY
public static final net.fabricmc.fabric.api.permission.v1.PermissionContext$Key<net.minecraft.commands.CommandSourceStack> COMMAND_SOURCE_STACK
public static final net.fabricmc.fabric.api.permission.v1.PermissionContext$Key<net.minecraft.world.level.Level> LEVEL
public static final net.fabricmc.fabric.api.permission.v1.PermissionContext$Key<net.minecraft.server.MinecraftServer> SERVER
public static net.fabricmc.fabric.api.permission.v1.MutablePermissionContext create(java.util.UUID, net.fabricmc.fabric.api.permission.v1.PermissionContext$Type, net.minecraft.server.permissions.PermissionLevel)
public static java.util.concurrent.CompletableFuture<net.fabricmc.fabric.api.permission.v1.MutablePermissionContext> offlinePlayer(java.util.UUID, net.minecraft.server.MinecraftServer)
public static java.util.concurrent.CompletableFuture<net.fabricmc.fabric.api.permission.v1.MutablePermissionContext> offlinePlayer(net.minecraft.server.players.NameAndId, net.minecraft.server.MinecraftServer)
public static <T> net.fabricmc.fabric.api.permission.v1.PermissionContext$Key<T> key(net.minecraft.resources.Identifier)
public abstract java.util.UUID uuid()
public abstract net.fabricmc.fabric.api.permission.v1.PermissionContext$Type type()
public abstract <T> T get(net.fabricmc.fabric.api.permission.v1.PermissionContext$Key<T>)
public default <T> T orElse(net.fabricmc.fabric.api.permission.v1.PermissionContext$Key<T>, T)
public default net.fabricmc.fabric.api.permission.v1.MutablePermissionContext mutable()
public abstract net.minecraft.server.permissions.PermissionLevel permissionLevel()
public abstract java.util.Set<net.fabricmc.fabric.api.permission.v1.PermissionContext$Key<?>> keys()
public default net.fabricmc.fabric.api.permission.v1.PermissionContext getPermissionContext()
static {}
```
